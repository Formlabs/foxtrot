use nalgebra_glm::DVec3;

/// Trait for a curve which maps from 1D to 3D
/// This trait is implement for both Bezier and NURBS curves, and abstracts
/// over them in the [`SampledCurve`] `struct`.
pub trait AbstractCurve {
    fn point(&self, u: f64) -> DVec3;
    fn derivs<const E: usize>(&self, u: f64) -> Vec<DVec3>;
}
