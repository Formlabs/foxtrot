use nalgebra_glm::{DVec2, DVec3, DVec4, DMat4, U32Vec3};

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

pub mod mesh;
pub mod stats;
pub mod surface;
