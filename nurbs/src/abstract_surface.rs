use nalgebra_glm::{DVec2, DVec3};
use crate::VecF;

/// Trait for a curve which maps from 2D (uv) to 3D
/// This trait is implement for both Bezier and NURBS surfaces, and abstracts
/// over them in the [`SampledSurface`] `struct`
pub trait AbstractSurface {
    fn point(&self, uv: DVec2) -> DVec3;

    /// Low-level function to calculate a point with a basis function hint
    /// (used as an optimization when we're re-using basis functions)
    fn point_from_basis(&self, uspan: usize, Nu: &VecF,
                               vspan: usize, Nv: &VecF) -> DVec3;

    fn derivs<const E: usize>(&self, uv: DVec2) -> Vec<Vec<DVec3>>;
}

