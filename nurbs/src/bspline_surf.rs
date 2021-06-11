use nalgebra_glm::{DVec2, DVec3};
use crate::{nd_curve::NDBSplineSurface, abstract_surface::AbstractSurface};

pub type BSplineSurface = NDBSplineSurface<3>;

impl AbstractSurface for BSplineSurface {
    fn point(&self, uv: DVec2) -> DVec3 {
        self.surface_point(uv)
    }
    fn derivs(&self, uv: DVec2, d: usize) -> Vec<DVec3> {
        self.surface_derivs(u, d)
    }
}
