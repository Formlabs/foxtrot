use std::convert::TryInto;
use nalgebra_glm as glm;
use nalgebra_glm::{DVec3, DVec4, DMat4, U32Vec3};
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

    pub fn save_stl(&self, filename: &str) -> std::io::Result<()> {
        let mut out: Vec<u8> = Vec::new();
        for c in 0..80 {
            out.push('x' as u8);
        }
        let u: u32 = self.triangles.len().try_into().expect("Too many triangles");
        out.extend(&u.to_le_bytes());
        for t in self.triangles.iter() {
            out.extend(std::iter::repeat(0).take(12)); // normal
            for v in t.verts.iter() {
                let v = self.vertices[*v as usize];
                out.extend(&(v.pos.x as f32).to_le_bytes());
                out.extend(&(v.pos.y as f32).to_le_bytes());
                out.extend(&(v.pos.z as f32).to_le_bytes());
                out.extend(std::iter::repeat(0).take(2)); // attributes
            }
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
        // TODO: project the bound contours onto the surface, triangulate,
        // then deproject and build triangles
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
        let mut out = Vec::new();
        for e in edge_list {
            if let &Entity::OrientedEdge(_, element, orientation) = self.entity(*e) {
                out.extend(self.oriented_edge(element, orientation).into_iter());
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

    fn circle(&self, u: DVec3, v: DVec3, position: Id, radius: f64) -> Vec<DVec3> {
        let (location, axis, ref_direction) = if let &Entity::Axis2Placement3d(_, location, axis, ref_direction) = self.entity(position) {
            self.axis2_placement_3d(location, axis, ref_direction)
        } else {
            panic!("Could not get Axis2Placement3d {:?}", self.entity(position));
        };

        // Build a rotation matrix to go from flat (XY) to 3D space
        let mut mat = DMat4::identity();
        mat.set_column(0, &glm::vec3_to_vec4(&ref_direction));
        mat.set_column(1, &glm::vec3_to_vec4(&axis.cross(&ref_direction)));
        mat.set_column(2, &glm::vec3_to_vec4(&axis));
        mat.set_column(3, &glm::vec3_to_vec4(&location));
        *mat.get_mut((3, 3)).unwrap() =  1.0;

        // Calculate the inverse of this matrix
        let mat_i = mat.try_inverse().expect("Could not invert");

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
        const N: usize = 10;
        let mut out = Vec::new();
        // Project onto the pnt + dir, and walk from start to end
        for i in 0..N {
            let frac = ((N - i - 1) as f64) / ((N - 1) as f64);
            out.push(pnt + dir * (start * frac + end * (1.0 - frac)));
        }
        out
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
