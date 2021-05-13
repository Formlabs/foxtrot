pub struct Vertex {
    pub pos: [f64; 3],
    pub norm: [f64; 3],
    pub color: [f64; 3],
}
pub struct Triangle {
    pub verts: [u32; 3],
}

use crate::ap214::Entity;

pub fn triangulate<S>(_step: &[Entity<S>]) -> (Vec<Vertex>, Vec<Triangle>) {
    (
        vec![
            Vertex {
                pos: [0.0, 0.0, 0.0],
                norm: [0.0, 0.0, 1.0],
                color: [1.0, 0.0, 0.0],
            }, Vertex {
                pos: [1.0, 0.0, 0.0],
                norm: [0.0, 0.0, 1.0],
                color: [0.0, 1.0, 0.0],
            }, Vertex {
                pos: [0.0, 1.0, 0.0],
                norm: [0.0, 0.0, 1.0],
                color: [0.0, 0.0, 1.0],
            },
        ],
        vec![ Triangle { verts: [0, 1, 2] } ]
    )
}
