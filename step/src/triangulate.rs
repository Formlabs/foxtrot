use nalgebra_glm::{DVec3, U32Vec3};

pub struct Vertex {
    pub pos: DVec3,
    pub norm: DVec3,
    pub color: DVec3,
}
pub struct Triangle {
    pub verts: U32Vec3,
}

use crate::ap214::{Entity, Id};

struct Triangulator<'a, S> {
    data: &'a [Entity<S>],
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl<'a, S: std::fmt::Debug> Triangulator<'a, S> {
    fn new(d: &'a [Entity<S>]) -> Self {
        Self {
            data: d,
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
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
        vec![]
    }
}

pub fn triangulate<S: std::fmt::Debug>(step: &[Entity<S>]) -> (Vec<Vertex>, Vec<Triangle>) {
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
