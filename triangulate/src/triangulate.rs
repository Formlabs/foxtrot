use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use nalgebra_glm as glm;
use glm::{DVec3, DVec4, DMat4, U32Vec3};
use rayon::prelude::*;
use log::{info, warn, error};

use step::{
    ap214, ap214::*, step_file::{FromEntity, StepFile}, id::Id, ap214::Entity,
};
use crate::{
    curve::Curve,
    mesh, mesh::{Mesh, Triangle},
    stats::Stats,
    surface::Surface
};
use nurbs::{BSplineSurface, KnotVector};

const SAVE_DEBUG_SVGS: bool = false;
const SAVE_PANIC_SVGS: bool = false;

pub fn triangulate(s: &StepFile) -> (Mesh, Stats) {
    let styled_items: Vec<_> = s.0.iter()
        .filter_map(|e| MechanicalDesignGeometricPresentationRepresentation_::try_from_entity(e))
        .flat_map(|m| m.items.iter())
        .filter_map(|item| s.entity(item.cast::<StyledItem_>()))
        .collect();
    let brep_colors: HashMap<_, DVec3> = styled_items.iter()
        .filter_map(|styled|
            if styled.styles.len() != 1 {
                None
            } else {
                presentation_style_color(s, styled.styles[0])
                    .map(|c| (styled.item, c))
            })
        .collect();
    let manifold_solid_breps: HashSet<RepresentationItem> = s.0.iter()
        .enumerate()
        .filter(|(_i, e)| ManifoldSolidBrep_::try_from_entity(e).is_some())
        .map(|(i, _e)| Id::new(i))
        .collect();

    // Store a map of parent -> (child, transform)
    let mut transform_stack: HashMap<_, Vec<_>> = HashMap::new();
    for r in s.0.iter()
        .filter_map(|e|
            RepresentationRelationshipWithTransformation_::try_from_entity(e))
    {
        transform_stack.entry(r.rep_2)
            .or_default()
            .push((r.rep_1, r.transformation_operator));
    }

    let children: HashSet<_> = transform_stack
        .values()
        .flat_map(|v| v.iter())
        .map(|v| v.0)
        .collect();
    let mut todo: Vec<_> = transform_stack
        .keys()
        .filter(|k| !children.contains(k))
        .map(|v| (*v, DMat4::identity()))
        .collect();

    let mut to_mesh: HashMap<Id<_>, Vec<_>> = HashMap::new();
    while let Some((id, mat)) = todo.pop() {
        if let Some(children) = transform_stack.get(&id) {
            for (child, transform) in children {
                let next_mat = item_defined_transformation(s, transform.cast());
                todo.push((*child, mat * next_mat));
            }
        } else {
            // Bind this transform to the RepresentationItem, which is
            // presumably a ManifoldSolidBrep
            let items = match &s.0[id.0] {
                Entity::AdvancedBrepShapeRepresentation(b) => &b.items,
                Entity::ShapeRepresentation(b) => &b.items,
                Entity::ManifoldSurfaceShapeRepresentation(b) => &b.items,
                e => panic!("Could not get shape from {:?}", e),
            };

            for m in items.iter().filter(|i| manifold_solid_breps.contains(*i))
            {
                to_mesh.entry(*m).or_default().push(mat);
            }
        }
    }
    // If there are items in manifold_solid_breps that aren't attached to
    // a transformation chain, then draw them individually
    let seen: HashSet<Id<_>> = to_mesh.keys().map(|i| *i).collect();
    for m in manifold_solid_breps.difference(&seen) {
        to_mesh.entry(*m).or_default().push(DMat4::identity());
    }

    let (mesh, stats) = to_mesh.par_iter()
        .fold(
            // Empty constructor
            || (Mesh::default(), Stats::default()),

            // Fold operation
            |(mut mesh, mut stats), (id, mats)| {
                let v_start = mesh.verts.len();
                let t_start = mesh.triangles.len();
                let brep: &ManifoldSolidBrep_ = match s.entity(id.cast()) {
                    Some(b) => b,
                    None => {
                        warn!("Skipping {:?} (not a ManifoldSolidBrep)",
                              s.0[id.0]);
                        return (mesh, stats);
                    },
                };
                closed_shell(s, brep.outer, &mut mesh, &mut stats);

                // Pick out a color from the color map and apply it to each
                // newly-created vertex
                let color = brep_colors.get(id)
                    .map(|c| *c)
                    .unwrap_or(DVec3::new(0.5, 0.5, 0.5));

                // Build copies of the mesh by copying and applying transforms
                let v_end = mesh.verts.len();
                let t_end = mesh.triangles.len();
                for mat in &mats[1..] {
                    for v in v_start..v_end {
                        let p = mesh.verts[v].pos;
                        let p_h = DVec4::new(p.x, p.y, p.z, 1.0);
                        let pos = (mat * p_h).xyz();

                        let n = mesh.verts[v].norm;
                        let norm = (mat * glm::vec3_to_vec4(&n)).xyz();

                        mesh.verts.push(mesh::Vertex { pos, norm, color });
                    }
                    let offset = mesh.verts.len() - v_end;
                    for t in t_start..t_end {
                        let mut tri = mesh.triangles[t];
                        tri.verts.add_scalar_mut(offset as u32);
                        mesh.triangles.push(tri);
                    }
                }

                // Now that we've built all of the other copies of the mesh,
                // re-use the original mesh and apply the first transform
                let mat = mats[0];
                for v in v_start..v_end {
                    let p = mesh.verts[v].pos;
                    let p_h = DVec4::new(p.x, p.y, p.z, 1.0);
                    mesh.verts[v].pos = (mat * p_h).xyz();

                    let n = mesh.verts[v].norm;
                    mesh.verts[v].norm = (mat * glm::vec3_to_vec4(&n)).xyz();

                    mesh.verts[v].color = color;
                }
                (mesh, stats)
            })
        .reduce(|| (Mesh::default(), Stats::default()),
                |a, b| (Mesh::combine(a.0, b.0), Stats::combine(a.1, b.1)));

    info!("num_shells: {}", stats.num_shells);
    info!("num_faces: {}", stats.num_faces);
    info!("num_errors: {}", stats.num_errors);
    info!("num_panics: {}", stats.num_panics);
    (mesh, stats)
}

fn item_defined_transformation(s: &StepFile, t: Id<ItemDefinedTransformation_>) -> DMat4 {
    let i = s.entity(t).expect("Could not get ItemDefinedTransform");
    let (location, axis, ref_direction) = axis2_placement_3d(s,
        i.transform_item_2.cast::<Axis2Placement3d_>());

    // Build a rotation matrix to go from flat (XY) to 3D space
    Surface::make_affine_transform(axis,
        ref_direction,
        axis.cross(&ref_direction),
        location)
}

fn presentation_style_color(s: &StepFile, p: PresentationStyleAssignment)
    -> Option<DVec3>
{
    // AAAAAHHHHH
    s.entity(p)
        .and_then(|p: &PresentationStyleAssignment_| {
                let mut surf = p.styles.iter().filter_map(|y| {
                    // This is an ambiguous parse, so we hard-code the first
                    // Entity item in the enum
                    use PresentationStyleSelect::PreDefinedPresentationStyle;
                    if let PreDefinedPresentationStyle(u) = y {
                        s.entity(u.cast::<SurfaceStyleUsage_>())
                    } else {
                        None
                    }});
                let out = surf.next();
                out
            })
        .and_then(|surf: &SurfaceStyleUsage_|
            s.entity(surf.style.cast::<SurfaceSideStyle_>()))
        .and_then(|surf: &SurfaceSideStyle_| if surf.styles.len() != 1 {
                None
            } else {
                s.entity(surf.styles[0].cast::<SurfaceStyleFillArea_>())
            })
        .map(|surf: &SurfaceStyleFillArea_|
            s.entity(surf.fill_area).expect("Could not get fill_area"))
        .and_then(|fill: &FillAreaStyle_| if fill.fill_styles.len() != 1 {
                None
            } else {
                s.entity(fill.fill_styles[0].cast::<FillAreaStyleColour_>())
            })
        .and_then(|f: &FillAreaStyleColour_|
            s.entity(f.fill_colour.cast::<ColourRgb_>()))
        .map(|c| DVec3::new(c.red, c.green, c.blue))
}

fn cartesian_point(s: &StepFile, a: Id<CartesianPoint_>) -> DVec3 {
    let p = s.entity(a).expect("Could not get cartesian point");
    DVec3::new(p.coordinates[0].0, p.coordinates[1].0, p.coordinates[2].0)
}

fn direction(s: &StepFile, a: Direction) -> DVec3 {
    let p = s.entity(a).expect("Could not get cartesian point");
    DVec3::new(p.direction_ratios[0],
               p.direction_ratios[1],
               p.direction_ratios[2])
}

fn axis2_placement_3d(s: &StepFile, t: Id<Axis2Placement3d_>) -> (DVec3, DVec3, DVec3) {
    let a = s.entity(t).expect("Could not get Axis2Placement3d");
    let location = cartesian_point(s, a.location);
    let axis = direction(s, a.axis.expect("Missing axis"));
    let ref_direction = direction(s, a.ref_direction.expect("Missing ref_direction"));
    (location, axis, ref_direction)
}

fn closed_shell(s: &StepFile, c: ClosedShell, mesh: &mut Mesh, stats: &mut Stats) {
    let cs = s.entity(c).expect("Could not get ClosedShell");
    for face in &cs.cfs_faces {
        advanced_face(s, face.cast(), mesh, stats);
    }
    stats.num_shells += 1;
}

fn advanced_face(s: &StepFile, f: AdvancedFace, mesh: &mut Mesh, stats: &mut Stats) {
    let face = s.entity(f).expect("Could not get AdvancedFace");
    stats.num_faces += 1;

    // Grab the surface, returning early if it's unimplemented
    let mut surf = match get_surface(s, face.face_geometry) {
        Some(s) => s,
        None => return,
    };

    // This is the starting point at which we insert new vertices
    let offset = mesh.verts.len();

    // For each contour, project from 3D down to the surface, then
    // start collecting them as constrained edges for triangulation
    let mut edges = Vec::new();
    let v_start = mesh.verts.len();
    let mut num_pts = 0;
    for b in &face.bounds {
        let bound_contours = face_bound(s, *b);

        match bound_contours.len() {
            // If we don't know how to build an edge for a subtype,
            // then we returned an empty vector, and skip the whole face
            // here
            0 => return,

            // Special case for a single-vertex point, which shows up in
            // cones: we push it as a Steiner point, but without any
            // associated contours.
            1 => {
                num_pts += 1;
                mesh.verts.push(mesh::Vertex {
                    pos: bound_contours[0],
                    norm: DVec3::zeros(),
                    color: DVec3::new(0.0, 0.0, 0.0),
                });
            },

            // Default for lists of contour points
            _ => {
                // Record the initial point to close the loop
                let start = num_pts;
                for pt in bound_contours {
                    // The contour marches forward!
                    edges.push((num_pts, num_pts + 1));

                    // Also store this vertex in the 3D triangulation
                    mesh.verts.push(mesh::Vertex {
                        pos: pt,
                        norm: DVec3::zeros(),
                        color: DVec3::new(0.0, 0.0, 0.0),
                    });
                    num_pts += 1;
                }
                // The last point is a duplicate, because it closes the
                // contours, so we skip it here and reattach the contour to
                // the start.
                num_pts -= 1;
                mesh.verts.pop();

                // Close the loop by returning to the starting point
                edges.pop();
                edges.last_mut().unwrap().1 = start;
            }
        }
    }

    // We inject Stiner points based on the surface type to improve curvature,
    // e.g. for spherical sections.  However, we don't want triagulation to
    // _fail_ due to these points, so if that happens, we nuke the point (by
    // assigning it to the first point in the list, which causes it to get
    // deduplicated), then retry.
    let mut pts = surf.lower_verts(&mut mesh.verts[v_start..]);
    let bonus_points = pts.len();
    surf.add_steiner_points(&mut pts, &mut mesh.verts);
    let result = std::panic::catch_unwind(|| {
        // TODO: this is only needed because we use pts below to save a debug
        // SVG if this panics.  Once we're confident in never panicking, we
        // can remove this.
        let mut pts = pts.clone();
        loop {
            let mut t = match cdt::Triangulation::new_with_edges(&pts, &edges) {
                Err(e) => break Err(e),
                Ok(t) => t,
            };
            match t.run() {
                Ok(()) => break Ok(t),
                // If triangulation failed due to a Steiner point on a fixed
                // edge, then reassign that point to pts[0] (so it will be
                // ignored as a duplicate)
                Err(cdt::Error::PointOnFixedEdge(p)) if p >= bonus_points => {
                    pts[p] = pts[0];
                    continue;
                },
                Err(e) => {
                    if SAVE_DEBUG_SVGS {
                        let filename = format!("err{}.svg", face.face_geometry.0);
                        t.save_debug_svg(&filename)
                            .expect("Could not save debug SVG");
                    }
                    break Err(e)
                },
            }
        }
    });
    match result {
        Ok(Ok(t)) => {
            for (a, b, c) in t.triangles() {
                let a = (a + offset) as u32;
                let b = (b + offset) as u32;
                let c = (c + offset) as u32;
                mesh.triangles.push(Triangle { verts:
                    if face.same_sense ^ surf.sign() {
                        U32Vec3::new(a, b, c)
                    } else {
                        U32Vec3::new(a, c, b)
                    }
                });
            }
        },
        Ok(Err(e)) => {
            error!("Got error while triangulating {}: {:?}",
                   face.face_geometry.0, e);
            stats.num_errors += 1;
        },
        Err(e) => {
            error!("Got panic while triangulating {}: {:?}",
                   face.face_geometry.0, e);
            if SAVE_PANIC_SVGS {
                let filename = format!("panic{}.svg", face.face_geometry.0);
                cdt::save_debug_panic(&pts, &edges, &filename)
                    .expect("Could not save debug SVG");
            }
            stats.num_panics += 1;
        }
    }
}

fn get_surface(s: &StepFile, surf: ap214::Surface) -> Option<Surface> {
    match &s.0[surf.0] {
        Entity::CylindricalSurface(c) => {
            let (location, axis, ref_direction) = axis2_placement_3d(s, c.position);
            Some(Surface::new_cylinder(axis, ref_direction, location, c.radius.0.0.0))
        },
        Entity::Plane(p) => {
            let (location, axis, ref_direction) = axis2_placement_3d(s, p.position);
            Some(Surface::new_plane(axis, ref_direction, location))
        },
        // We treat cones like planes, since that's a valid mapping into 2D
        Entity::ConicalSurface(c) => {
            let (location, axis, ref_direction) = axis2_placement_3d(s, c.position);
            Some(Surface::new_plane(axis, ref_direction, location))
        },
        Entity::SphericalSurface(c) => {
            // We'll ignore axis and ref_direction in favor of building an
            // orthonormal basis later on
            let (location, _axis, _ref_direction) = axis2_placement_3d(s, c.position);
            Some(Surface::new_sphere(location, c.radius.0.0.0))
        },
        Entity::BSplineSurfaceWithKnots(b) =>
        {
            assert!(b.u_closed.0.unwrap() == false);
            assert!(b.v_closed.0.unwrap() == false);
            assert!(b.self_intersect.0.unwrap() == false);

            // TODO: make KnotVector::from_multiplicies accept iterators?
            let u_knots: Vec<f64> = b.u_knots.iter().map(|k| k.0).collect();
            let u_multiplicities: Vec<usize> = b.u_multiplicities.iter()
                .map(|&k| k.try_into().expect("Got negative multiplicity"))
                .collect();
            let u_knot_vec = KnotVector::from_multiplicities(
                b.u_degree.try_into().expect("Got negative degree"),
                &u_knots, &u_multiplicities);

            let v_knots: Vec<f64> = b.v_knots.iter().map(|k| k.0).collect();
            let v_multiplicities: Vec<usize> = b.v_multiplicities.iter()
                .map(|&k| k.try_into().expect("Got negative multiplicity"))
                .collect();
            let v_knot_vec = KnotVector::from_multiplicities(
                b.v_degree.try_into().expect("Got negative degree"),
                &v_knots, &v_multiplicities);

            let control_points_list = control_points_2d(s, &b.control_points_list);

            let surf = BSplineSurface::new(
                b.u_closed.0.unwrap() == false,
                b.v_closed.0.unwrap() == false,
                u_knot_vec,
                v_knot_vec,
                control_points_list,
            );
            Some(Surface::new_bspline(surf))
        },
        e => {
            warn!("Could not get surface {:?}", e);
            None
        },
    }
}

fn control_points_1d(s: &StepFile, row: &Vec<CartesianPoint>) -> Vec<DVec3> {
    row.iter().map(|p| cartesian_point(s, *p)).collect()
}

fn control_points_2d(s: &StepFile, rows: &Vec<Vec<CartesianPoint>>) -> Vec<Vec<DVec3>> {
    rows.iter()
        .map(|row| control_points_1d(s, row))
        .collect()
}

fn face_bound(s: &StepFile, b: FaceBound) -> Vec<DVec3> {
    let (bound, orientation) = match &s.0[b.0] {
        Entity::FaceBound(b) => (b.bound, b.orientation),
        Entity::FaceOuterBound(b) => (b.bound, b.orientation),
        e => panic!("Could not get bound from {:?} at {:?}", e, b),
    };
    match &s.0[bound.0] {
        Entity::EdgeLoop(e) => {
            let mut d = edge_loop(s, &e.edge_list);
            if !orientation {
                d.reverse()
            }
            d
        },
        Entity::VertexLoop(v) => {
            // This is an "edge loop" with a single vertex, which is
            // used for cones and not really anything else.
            vec![vertex_point(s, v.loop_vertex)]
        }
        e => panic!("{:?} is not an EdgeLoop", e),
    }
}

fn edge_loop(s: &StepFile, edge_list: &[OrientedEdge]) -> Vec<DVec3> {
    let mut out = Vec::new();
    for (i, e) in edge_list.iter().enumerate() {
        // Remove the last item from the list, since it's the beginning
        // of the following list (hopefully)
        if i > 0 {
            out.pop();
        }
        let edge = s.entity(*e).expect("Could not get OrientedEdge");
        let o = edge_curve(s, edge.edge_element.cast(), edge.orientation);

        // Special case: return an empty vector if we don't
        // know how to triangulate any of the components.
        if o.is_empty() {
            return vec![];
        }
        out.extend(o.into_iter());
    }
    out
}

fn edge_curve(s: &StepFile, e: EdgeCurve, orientation: bool) -> Vec<DVec3> {
    let edge_curve = s.entity(e).expect("Could not get EdgeCurve");
    let curve = match &s.0[edge_curve.edge_geometry.0] {
        Entity::Circle(c) => {
            let (location, axis, ref_direction) = axis2_placement_3d(s, c.position.cast());
            Curve::new_circle(location, axis, ref_direction, c.radius.0.0.0,
                              edge_curve.edge_start == edge_curve.edge_end,
                              edge_curve.same_sense ^ !orientation)
        },
        Entity::Ellipse(c) => {
            let (location, axis, ref_direction) = axis2_placement_3d(s, c.position.cast());
            Curve::new_ellipse(location, axis, ref_direction,
                               c.semi_axis_1.0.0.0, c.semi_axis_2.0.0.0,
                               edge_curve.edge_start == edge_curve.edge_end,
                               edge_curve.same_sense ^ !orientation)
        },
        Entity::BSplineCurveWithKnots(c) => {
            assert!(c.closed_curve.0.unwrap() == false);
            assert!(c.self_intersect.0.unwrap()== false);

            let control_points_list = control_points_1d(
                s, &c.control_points_list);

            let knots: Vec<f64> = c.knots.iter().map(|k| k.0).collect();
            let multiplicities: Vec<usize> = c.knot_multiplicities.iter()
                .map(|&k| k.try_into().expect("Got negative multiplicity"))
                .collect();
            let knot_vec = KnotVector::from_multiplicities(
                c.degree.try_into().expect("Got negative degree"),
                &knots, &multiplicities);

            let curve = nurbs::BSplineCurve::new(
                c.closed_curve.0.unwrap() == false,
                knot_vec,
                control_points_list,
            );
            Curve::new_bspline_with_knots(curve)
        },
        // The Line type ignores pnt / dir and just uses u and v
        Entity::Line(_) => Curve::new_line(),
        e => {
            warn!("Could not get edge from {:?}", e);
            return vec![]
        },
    };
    let (start, end) = if orientation {
        (edge_curve.edge_start, edge_curve.edge_end)
    } else {
        (edge_curve.edge_end, edge_curve.edge_start)
    };
    let u = vertex_point(s, start);
    let v = vertex_point(s, end);
    curve.build(u, v)
}

fn vertex_point(s: &StepFile, v: Vertex) -> DVec3 {
    cartesian_point(s,
        s.entity(v.cast::<VertexPoint_>())
            .expect("Could not get VertexPoint")
            .vertex_geometry
            .cast())
}
