/// Takes a STEP file (as an array of bytes), and returns a triangle mesh.
///
/// Vertices are packed into rows of 9 floats, representing
/// - Position
/// - Normal
/// - Color
///
/// Vertices are rows of three indexes into the triangle array
///
use wasm_bindgen::prelude::*;
use log::{Level};

#[wasm_bindgen]
pub fn init_log() {
    console_log::init_with_level(Level::Info).expect("Failed to initialize log");
}

#[wasm_bindgen]
pub fn step_to_triangle_buf(data: String) -> Vec<f32> {
    use step::step_file::StepFile;
    use triangulate::triangulate::triangulate; // lol

    let flat = StepFile::strip_flatten(data.as_bytes());
    let step = StepFile::parse(&flat);
    let (mut mesh, _stats) = triangulate(&step);

    let (mut xmin, mut xmax) = (std::f64::INFINITY, -std::f64::INFINITY);
    let (mut ymin, mut ymax) = (std::f64::INFINITY, -std::f64::INFINITY);
    let (mut zmin, mut zmax) = (std::f64::INFINITY, -std::f64::INFINITY);
    for pos in mesh.verts.iter().map(|p| p.pos) {
        xmin = xmin.min(pos.x);
        xmax = xmax.max(pos.x);
        ymin = ymin.min(pos.y);
        ymax = ymax.max(pos.y);
        zmin = ymin.min(pos.z);
        zmax = ymax.max(pos.z);
    }
    let scale = (xmax - xmin).max(ymax - ymin).max(zmax - zmin);
    let xc = (xmax + xmin) / 2.0;
    let yc = (ymax + ymin) / 2.0;
    let zc = (zmax + zmin) / 2.0;
    for pos in mesh.verts.iter_mut().map(|p| &mut p.pos) {
        pos.x = (pos.x - xc) / scale * 200.0;
        pos.y = (pos.y - yc) / scale * 200.0;
        pos.z = (pos.z - zc) / scale * 200.0;
    }

    mesh.triangles.iter()
        .flat_map(|v| v.verts.iter())
        .map(|p| &mesh.verts[*p as usize])
        .flat_map(|v| v.pos.iter().chain(&v.norm).chain(&v.color))
        .map(|f| *f as f32)
        .collect()
}
