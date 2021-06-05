use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use nalgebra_glm as glm;
use glm::{DVec3, DVec4, DMat4, U32Vec3};
use rayon::prelude::*;
use log::warn;

use step2::{step_file::StepFile, id::Id, ap214::Entity, ap214, ap214::*};
use crate::{
    curve::Curve,
    mesh, mesh::{Mesh, Triangle},
    stats::Stats,
    surface::Surface
};
use nurbs::{BSplineSurface, KnotVector};

pub fn triangulate(s: &StepFile) -> (Mesh, Stats) {
    // Build the tree of transforms, from the root up
    let mut roots = HashSet::new();
    let mut leaves = HashSet::new();

    // from root to REPRESENTATION_RELATIONSHIP id
    let mut lookup: HashMap<Representation, Vec<usize>> = HashMap::new();
    for (i, e) in s.0.iter().enumerate() {
        let r = match e {
            Entity::RepresentationRelationshipWithTransformation(r) => {
                leaves.insert(r.rep_1);
                r.rep_2
            },
            Entity::ShapeRepresentationRelationship(r) => r.rep_2,
            Entity::ShapeDefinitionRepresentation(r) => r.used_representation,
            _ => continue,
        };
        roots.insert(r);
        lookup.entry(r).or_insert_with(Vec::new).push(i);
    }
    // Pick out the roots of the transformation DAG
    let mut todo: Vec<_> = roots.difference(&leaves)
        .map(|i| (*i, DMat4::identity()))
        .collect();

    // We'll store leaves of the graph along with their transform here
    let mut to_mesh = Vec::new();

    // Iterate through the transformation DAG
    while let Some((id, mat)) = todo.pop() {
        for v in &lookup[&id] {
            match &s.0[*v] {
                Entity::RepresentationRelationshipWithTransformation(r) => {
                    assert!(r.rep_2 == id);
                    let next_mat = item_defined_transformation(s, r.transformation_operator.cast());
                    if roots.contains(&r.rep_1) {
                        todo.push((r.rep_1, mat * next_mat));
                    } else {
                        to_mesh.push((r.rep_1, mat * next_mat));
                    }
                },
                Entity::ShapeRepresentationRelationship(r) =>
                    to_mesh.push((r.rep_2, mat)),
                Entity::ShapeDefinitionRepresentation(r) =>
                    to_mesh.push((r.used_representation, mat)),
                e => panic!("Invalid entity {:?}", e),
            }
        }
    }

    let (mesh, stats) = to_mesh.par_iter()
        .map(|(id, mat)| {
            let (mut mesh, stats) = shape_representation(s, *id);
            for v in 0..mesh.verts.len() {
                 let p = mesh.verts[v].pos;
                 mesh.verts[v].pos = (mat * DVec4::new(p.x, p.y, p.z, 1.0)).xyz();
                 let n = mesh.verts[v].norm;
                 mesh.verts[v].norm = (mat * DVec4::new(n.x, n.y, n.z, 0.0)).xyz();
             }
            (mesh, stats)
        })
        .reduce(|| (Mesh::default(), Stats::default()),
                |a, b| (Mesh::combine(a.0, b.0), Stats::combine(a.1, b.1)));

    println!("num_shells: {}", stats.num_shells);
    println!("num_faces: {}", stats.num_faces);
    println!("num_errors: {}", stats.num_errors);
    println!("num_panics: {}", stats.num_panics);
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

fn shape_representation(s: &StepFile, b: Representation) -> (Mesh, Stats) {
    let items = match &s.0[b.0] {
        Entity::AdvancedBrepShapeRepresentation(b) => &b.items,
        Entity::ShapeRepresentation(b) => &b.items,
        Entity::ManifoldSurfaceShapeRepresentation(b) => &b.items,
        e => panic!("Cannot get shape from {:?}", e),
    };

    let mut mesh = Mesh::default();
    let mut stats = Stats::default();
    for i in items {
        match &s.0[i.0] {
            Entity::ManifoldSolidBrep(b) =>
                closed_shell(s, b.outer, &mut mesh, &mut stats),
            Entity::Axis2Placement3d(..) => (), // continue silently
            e => eprintln!("Skipping {:?} (not a ManifoldSolidBrep)", e),
        }
    }
    (mesh, stats)
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
    let surf = match get_surface(s, face.face_geometry) {
        Some(s) => s,
        None => return,
    };

    // This is the starting point at which we insert new vertices
    let offset = mesh.verts.len();

    // For each contour, project from 3D down to the surface, then
    // start collecting them as constrained edges for triangulation
    let mut pts = Vec::new();
    let mut edges = Vec::new();
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
                // Project to the 2D subspace for triangulation
                let proj = surf.lower(bound_contours[0]);
                pts.push((proj.x, proj.y));

                mesh.verts.push(mesh::Vertex {
                    pos: bound_contours[0],
                    norm: surf.normal(bound_contours[0], proj),
                    color: DVec3::new(0.0, 0.0, 0.0),
                });
            },

            // Default for lists of contour points
            _ => {
                // Record the initial point to close the loop
                let start = pts.len();
                for pt in bound_contours {
                    // The contour marches forward!
                    edges.push((pts.len(), pts.len() + 1));

                    // Project to the 2D subspace for triangulation
                    let proj = surf.lower(pt);
                    pts.push((proj.x, proj.y));

                    // Also store this vertex in the 3D triangulation
                    mesh.verts.push(mesh::Vertex {
                        pos: pt,
                        norm: surf.normal(pt, proj),
                        color: DVec3::new(0.0, 0.0, 0.0),
                    });
                }
                // The last point is a duplicate, because it closes the
                // contours, so we skip it here and reattach the contour to
                // the start.
                pts.pop();
                mesh.verts.pop();

                // Close the loop by returning to the starting point
                edges.pop();
                edges.last_mut().unwrap().1 = start;
            }
        }
    }

    let result = std::panic::catch_unwind(move || {
        let mut t = cdt::Triangulation::new_with_edges(&pts, &edges)
            .expect("Could not build CDT triangulation");
        match t.run() {
            Ok(()) => Ok(t),
            Err(e) => {
                t.save_debug_svg(&format!("out{}.svg", face.face_geometry.0))
                    .expect("Could not save debug SVG");
                Err(e)
            },
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
            eprintln!("Got error while triangulating: {:?}", e);
            stats.num_errors += 1;
        },
        Err(e) => {
            eprintln!("Got panic while triangulating: {:?}", e);
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
    let bound = s.entity(b).expect("Could not get FaceBound");
    match &s.0[bound.bound.0] {
        Entity::EdgeLoop(e) => {
            let mut d = edge_loop(s, &e.edge_list);
            if !bound.orientation {
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
