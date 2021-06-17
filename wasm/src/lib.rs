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
use log::{Level, info};

#[wasm_bindgen]
pub fn init_log() {
    console_log::init_with_level(Level::Info).expect("Failed to initialize log");
}

#[wasm_bindgen]
pub fn foxtrot(data: String) -> Vec<f32> {
    use step::step_file::StepFile;
    use triangulate::triangulate::triangulate; // lol

    let flat = StepFile::strip_flatten(data.as_bytes());
    let step = StepFile::parse(&flat);
    info!("Got {} entities", step.0.len());
    let (mesh, _stats) = triangulate(&step);

    mesh.triangles.iter()
        .flat_map(|v| v.verts.iter())
        .map(|p| &mesh.verts[*p as usize])
        .flat_map(|v| v.pos.iter().chain(&v.norm).chain(&v.color))
        .map(|f| *f as f32)
        .collect()
}

#[wasm_bindgen]
pub fn call_me_from_javascript(a: i32, b: i32) -> i32 {
    a + b + 15
}

#[wasm_bindgen]
pub fn call_me2() -> Vec<f32> {
    vec![1.0, 2.0, 3.0]
}
