use std::convert::TryInto;
use nalgebra_glm::{DVec3, U32Vec3};

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

#[derive(Default)]
pub struct Triangulation {
    verts: Vec<Vertex>,
    index: Vec<Triangle>,
    num_shells: usize,
    num_faces: usize,
    num_errors: usize,
    num_panics: usize,
}

impl Triangulation {
    // Combine two triangulations with an associative binary operator
    // (why yes, this _is_ a monoid)
    pub fn combine(mut a: Self, b: Self) -> Self {
        let dv = a.verts.len().try_into().expect("too many triangles");
        a.verts.extend(b.verts);
        a.index.extend(b.index.into_iter()
            .map(|t| Triangle { verts: t.verts.add_scalar(dv) }));
        a.num_shells += b.num_shells;
        a.num_faces += b.num_faces;
        a.num_errors += b.num_errors;
        a.num_panics += b.num_panics;
        a
    }
}
