use nalgebra_glm::{DVec2, DVec3};

/// Trait for a curve which maps from 2D (uv) to 3D
/// This trait is implement for both Bezier and NURBS surfaces, and abstracts
/// over them in the [`SampledSurface`] `struct`
pub trait AbstractSurface {
    fn point(&self, uv: DVec2) -> DVec3;
    fn derivs(&self, uv: DVec2, d: usize) -> Vec<DVec3>;
}

