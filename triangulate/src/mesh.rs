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
pub struct Mesh {
    pub verts: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    // Combine two triangulations with an associative binary operator
    // (why yes, this _is_ a monoid)
    pub fn combine(mut a: Self, b: Self) -> Self {
        let dv = a.verts.len().try_into().expect("too many triangles");
        a.verts.extend(b.verts);
        a.triangles.extend(b.triangles.into_iter()
            .map(|t| Triangle { verts: t.verts.add_scalar(dv) }));
        a
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
                let v = self.verts[*v as usize];
                out.extend(&(v.pos.x as f32).to_le_bytes());
                out.extend(&(v.pos.y as f32).to_le_bytes());
                out.extend(&(v.pos.z as f32).to_le_bytes());
            }
            out.extend(std::iter::repeat(0).take(2)); // attributes
        }
        std::fs::write(filename, out)
    }
}
