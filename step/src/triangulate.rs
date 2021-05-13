use std::convert::TryInto;
use nalgebra_glm as glm;
use nalgebra_glm::{DVec2, DVec3, DVec4, DMat4, U32Vec3};
use crate::ap214::StepFile;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: DVec3,
    pub norm: DVec3,
    pub color: DVec3,
}
#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub verts: U32Vec3,
}

use crate::ap214::{Entity, Id};

pub struct Triangulator<'a, S> {
    data: &'a [Entity<S>],
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl<'a, S: std::fmt::Debug> Triangulator<'a, S> {
    fn new(d: &'a StepFile<S>) -> Self {
        Self {
            data: &d.0,
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn run(d: &'a StepFile<S>) -> Self {
        let mut t = Self::new(d);
        t.triangulate();
        t
    }

    /// Writes the triangulation to a STL, for debugging
    pub fn save_stl(&self, filename: &str) -> std::io::Result<()> {
        let mut out: Vec<u8> = Vec::new();
        for _ in 0..80 { // header
            out.push('x' as u8);
        }
        let u: u32 = self.triangles.len().try_into()
            .expect("Too many triangles");
        out.extend(&u.to_le_bytes());
        for t in self.triangles.iter() {
            out.extend(std::iter::repeat(0).take(12)); // normal
            for v in t.verts.iter() {
                let v = self.vertices[*v as usize];
                out.extend(&(v.pos.x as f32).to_le_bytes());
                out.extend(&(v.pos.y as f32).to_le_bytes());
                out.extend(&(v.pos.z as f32).to_le_bytes());
            }
            out.extend(std::iter::repeat(0).take(2)); // attributes
        }
        std::fs::write(filename, out)
    }

    fn triangulate(&mut self) {
        for e in self.data {
            if let Entity::AdvancedFace(_, bounds, surface, same_sense) = e {
                self.advanced_face(bounds, *surface, *same_sense);
            }
        }
    }

    fn entity(&self, i: Id) -> &Entity<S> {
        &self.data[i.0]
    }

    fn advanced_face(&mut self, bounds: &[Id], surface: Id, same_sense: bool) {
        let mut bound_contours = Vec::new();
        for b in bounds {
            if let &Entity::FaceBound(_, bound, orientation) = self.entity(*b) {
                bound_contours.push(self.face_bounds(bound, orientation));
            } else {
                panic!("Expected FaceBounds; got {:?}", self.entity(*b));
            }
        }

        // For each contour, project from 3D down to the surface, then
        // start collecting them as constrained edges for triangulation
        let offset = self.vertices.len();
        let s = self.get_surface(surface);
        let mut pts = Vec::new();
        let mut contours = Vec::new();
        for bc in bound_contours {
            // Start a new contour here
            let mut contour = Vec::new();

            // Record the initial point to close the loop
            let start = pts.len();

            for pt in bc {
                // The contour marches forward!
                contour.push(pts.len());

                // Project to the 2D subspace for triangulation
                let proj = s.lower(pt);
                pts.push((proj.x, proj.y));

                // Also store this vertex in the 3D triangulation
                self.vertices.push(Vertex {
                    pos: pt,
                    norm: s.normal(pt),
                    color: DVec3::new(0.0, 0.0, 0.0),
                });
            }
            // The last point is a duplicate, because it closes the contours,
            // so we skip it here and reattach the contour to the start.
            pts.pop();
            self.vertices.pop();

            // Close the loop by returning to the starting point
            *contour.last_mut().unwrap() = start;

            // Store this contour in the global list
            contours.push(contour);
        }

        let mut t = cdt::Triangulation::new_from_contours(&pts, &contours)
            .expect("Could not build CDT triangulation");
        match t.run() {
            Ok(()) => for (a, b, c) in t.triangles() {
                let a = (a + offset) as u32;
                let b = (b + offset) as u32;
                let c = (c + offset) as u32;
                self.triangles.push(Triangle { verts:
                    if same_sense ^ s.sign() {
                        U32Vec3::new(a, b, c)
                    } else {
                        U32Vec3::new(a, c, b)
                    }
                });
            },
            Err(e) => {
                eprintln!("Got error when triangulating: {:?}", e);
                t.save_debug_svg(&format!("out{}.svg", surface.0))
                    .expect("Could not save debug SVG");
            }
        }
    }

    fn get_surface(&self, surface: Id) -> Surface {
        if let &Entity::CylindricalSurface(_, position, radius) = self.entity(surface) {
            let (location, axis, ref_direction) = self.axis2_placement_3d_(position);
            Surface::new_cylinder(axis, ref_direction, location, radius)
        } else if let &Entity::Plane(_, position) = self.entity(surface) {
            let (location, axis, ref_direction) = self.axis2_placement_3d_(position);
            Surface::new_plane(axis, ref_direction, location)
        } else {
            panic!("Could not get surface {:?}", self.entity(surface));
        }
    }

    fn face_bounds(&mut self, bound: Id, orientation: bool) -> Vec<DVec3> {
        if let Entity::EdgeLoop(_, edge_list) = self.entity(bound) {
            let edge_list = edge_list.clone(); // TODO: this is inefficient
            let mut d = self.edge_loop(&edge_list);
            if !orientation {
                d.reverse()
            }
            d
        } else {
            panic!("{:?} is not an EdgeLoop", self.entity(bound));
        }
    }

    fn edge_loop(&mut self, edge_list: &[Id]) -> Vec<DVec3> {
        let mut out = vec![DVec3::new(0.0, 0.0, 0.0)];
        for e in edge_list {
            out.pop();
            if let &Entity::OrientedEdge(_, element, orientation) = self.entity(*e) {
                out.extend(self.oriented_edge(element, orientation).into_iter());
            } else {
                panic!("Invalid OrientedEdge {:?}", self.entity(*e));
            }
        }
        out
    }

    fn oriented_edge(&mut self, element: Id, orientation: bool) -> Vec<DVec3> {
        if let &Entity::EdgeCurve(_, edge_start, edge_end, edge_geometry, same_sense) = self.entity(element) {
            let mut d = self.edge_curve(edge_start, edge_end, edge_geometry, same_sense);
            if !orientation {
                d.reverse()
            }
            d
        } else {
            panic!("Invalid");
        }
    }

    fn edge_curve(&mut self, edge_start: Id, edge_end: Id, edge_geometry: Id, same_sense: bool) -> Vec<DVec3> {
        let u = if let &Entity::VertexPoint(_, i) = self.entity(edge_start) {
            self.vertex_point(i)
        } else {
            panic!("Could not get vertex from {:?}", self.entity(edge_start));
        };
        let v = if let &Entity::VertexPoint(_, i) = self.entity(edge_end) {
            self.vertex_point(i)
        } else {
            panic!("Could not get vertex from {:?}", self.entity(edge_start));
        };

        if let &Entity::Circle(_, position, radius) = self.entity(edge_geometry) {
            assert!(edge_start == edge_end);
            self.circle(u, v, position, radius)
        } else if let &Entity::Line(_, pnt, dir) = self.entity(edge_geometry) {
            self.line(u, v, pnt, dir)
        } else {
            panic!("Could not get edge from {:?}", self.entity(edge_geometry));
        }
    }

    fn vertex_point(&self, vertex_geometry: Id) -> DVec3 {
        if let &Entity::CartesianPoint(_, (x, y, z)) = self.entity(vertex_geometry) {
            DVec3::new(x, y, z)
        } else {
            panic!("Could not get CartesianPoint from {:?}", self.entity(vertex_geometry));
        }
    }

    fn circle(&self, u: DVec3, _v: DVec3, position: Id, radius: f64) -> Vec<DVec3> {
        let (location, axis, ref_direction) = self.axis2_placement_3d_(position);

        // Build a rotation matrix to go from flat (XY) to 3D space
        let (mat, mat_i) = Surface::build_mats(axis, ref_direction, location);

        // Project from 3D into the circle's flat plane
        let u_flat = mat_i * DVec4::new(u.x, u.y, u.z, 1.0);

        // Pick the starting angle in the circle's flat plane
        let start_ang = u_flat.y.atan2(u_flat.x);
        let end_ang = start_ang + std::f64::consts::PI * 2.0;

        const N: usize = 10;
        let mut out = Vec::new();
        // Project onto the pnt + dir, and walk from start to end
        for i in 0..N {
            let frac = ((N - i - 1) as f64) / ((N - 1) as f64);
            let ang = start_ang * (1.0 - frac) + end_ang * frac;
            let pos = DVec4::new(ang.cos() * radius, ang.sin() * radius, 0.0, 1.0);

            // Project back into 3D
            out.push(glm::vec4_to_vec3(&(mat * pos)));
        }
        out
    }

    fn axis2_placement_3d_(&self, id: Id) -> (DVec3, DVec3, DVec3) {
        if let &Entity::Axis2Placement3d(_, location, axis, ref_direction) = self.entity(id) {
            self.axis2_placement_3d(location, axis, ref_direction)
        } else {
            panic!("Could not get Axis2Placement3d {:?}", self.entity(id));
        }
    }

    fn axis2_placement_3d(&self, location: Id, axis: Id, ref_direction: Id) -> (DVec3, DVec3, DVec3) {
        let location = if let &Entity::CartesianPoint(_, (x, y, z)) = self.entity(location) {
            DVec3::new(x, y, z)
        } else {
            panic!("Could not get CartesianPoint from {:?}", self.entity(location));
        };
        let axis = if let &Entity::Direction(_, (x, y, z)) = self.entity(axis) {
            DVec3::new(x, y, z)
        } else {
            panic!("Could not get Direction from {:?}", self.entity(axis));
        };
        let ref_direction = if let &Entity::Direction(_, (x, y, z)) = self.entity(ref_direction) {
            DVec3::new(x, y, z)
        } else {
            panic!("Could not get Direction from {:?}", self.entity(ref_direction));
        };
        (location, axis, ref_direction)
    }

    fn line(&self, u: DVec3, v: DVec3, pnt: Id, dir: Id) -> Vec<DVec3> {
        let pnt = self.vertex_point(pnt);
        let dir = if let &Entity::Vector(_, orientation, magnitude) = self.entity(dir) {
            self.vector(orientation, magnitude)
        } else {
            panic!("Could not get vector from {:?}", self.entity(dir));
        };
        let start = (u - pnt).dot(&dir);
        let end = (v - pnt).dot(&dir);

        // Project onto the pnt + dir, and walk from start to end
        vec![pnt + dir * start, pnt + dir * end]
    }

    fn vector(&self, orientation: Id, magnitude: f64) -> DVec3 {
        if let &Entity::Direction(_, (x, y, z)) = self.entity(orientation) {
            DVec3::new(x * magnitude, y * magnitude, z * magnitude)
        } else {
            panic!("Could not get Direction from {:?}", self.entity(orientation));
        }
    }
}

pub fn triangulate<S: std::fmt::Debug>(step: &StepFile<S>) -> (Vec<Vertex>, Vec<Triangle>) {
    let mut t = Triangulator::new(step);
    t.triangulate();

    // Ignore t for now
    (
        vec![
            Vertex {
                pos: DVec3::new(0.0, 0.0, 0.0),
                norm: DVec3::new(0.0, 0.0, 1.0),
                color: DVec3::new(1.0, 0.0, 0.0),
            }, Vertex {
                pos: DVec3::new(1.0, 0.0, 0.0),
                norm: DVec3::new(0.0, 0.0, 1.0),
                color: DVec3::new(0.0, 1.0, 0.0),
            }, Vertex {
                pos: DVec3::new(0.0, 1.0, 0.0),
                norm: DVec3::new(0.0, 0.0, 1.0),
                color: DVec3::new(0.0, 0.0, 1.0),
            },
        ],
        vec![ Triangle { verts: U32Vec3::new(0, 1, 2) } ]
    )
}

////////////////////////////////////////////////////////////////////////////////

// Represents a surface in 3D space, with a function to project a 3D point
// on the surface down to a 2D space.
#[derive(Debug, Copy, Clone)]
enum Surface {
    Cylinder {
        location: DVec3,
        axis: DVec3,
        mat_i: DMat4,
        radius: f64,
    },
    Plane {
        normal: DVec3,
        mat_i: DMat4,
    }
}

impl Surface {
    pub fn new_cylinder(axis: DVec3, ref_direction: DVec3, location: DVec3, radius: f64) -> Self {
        Surface::Cylinder {
            mat_i: Self::build_mat_i(axis, ref_direction, location),
            axis, radius, location,
        }
    }

    pub fn new_plane(axis: DVec3, ref_direction: DVec3, location: DVec3) -> Self {
        Surface::Plane {
            mat_i: Self::build_mat_i(axis, ref_direction, location),
            normal: axis,
        }
    }

    fn build_mat(axis: DVec3, ref_direction: DVec3, location: DVec3) -> DMat4 {
        let mut mat = DMat4::identity();
        mat.set_column(0, &glm::vec3_to_vec4(&ref_direction));
        mat.set_column(1, &glm::vec3_to_vec4(&axis.cross(&ref_direction)));
        mat.set_column(2, &glm::vec3_to_vec4(&axis));
        mat.set_column(3, &glm::vec3_to_vec4(&location));
        *mat.get_mut((3, 3)).unwrap() =  1.0;
        mat
    }

    fn build_mats(axis: DVec3, ref_direction: DVec3, location: DVec3) -> (DMat4, DMat4) {
        let m = Self::build_mat(axis, ref_direction, location);
        (m, m.try_inverse().expect("Could not invert"))
    }

    fn build_mat_i(axis: DVec3, ref_direction: DVec3, location: DVec3) -> DMat4 {
        Self::build_mat(axis, ref_direction, location)
            .try_inverse()
            .expect("Could not invert")
    }

    /// Lowers a 3D point on a specific surface into a 2D space defined by
    /// the surface type.
    pub fn lower(&self, p: DVec3) -> DVec2 {
        let p = DVec4::new(p.x, p.y, p.z, 1.0);
        match self {
            Surface::Plane { mat_i, .. } => {
                glm::vec4_to_vec2(&(mat_i * p))
            },
            Surface::Cylinder { mat_i, radius, .. } => {
                let p = mat_i * p;
                // We convert the Z coordinates to either add or subtract from
                // the radius, so that we maintain the right topology (instead
                // of doing something like theta-z coordinates, which wrap
                // around awkwardly).

                // Assume that Z is roughly on the same order of magnitude
                // as the radius, and use a sigmoid function
                let scale = 1.0 / (1.0 + (-p.z / radius).exp());
                DVec2::new(p.x * scale, p.y * scale)
            }
            _ => unimplemented!(),
        }
    }

    pub fn normal(&self, p: DVec3) -> DVec3 {
        match self {
            Surface::Plane { normal, .. } => *normal,
            Surface::Cylinder { location, axis, .. } => {
                // Project the point onto the axis
                let proj = (p - location).dot(axis);

                // Find the nearest point along the axis
                let nearest = location + axis * proj;

                // Then the normal is just pointing along that direction
                // (same hack as below)
                -(p - nearest).normalize()
            }
        }
    }

    pub fn sign(&self) -> bool {
        // TODO: this is a hack, why are cylinders different from planes?
        match self {
            Surface::Plane {..} => false,
            Surface::Cylinder {..} => true,
        }
    }
}
