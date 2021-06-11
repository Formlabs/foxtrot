use nalgebra_glm::{DVec2, DVec3};
use crate::{nd_surface::NDBSplineSurface, abstract_surface::AbstractSurface, VecF};

pub type BSplineSurface = NDBSplineSurface<3>;

impl AbstractSurface for BSplineSurface {
    fn point(&self, uv: DVec2) -> DVec3 {
        self.surface_point(uv)
    }

    fn point_from_basis(&self, uspan: usize, Nu: &VecF,
                               vspan: usize, Nv: &VecF) -> DVec3
    {
        self.surface_point_from_basis(uspan, Nu, vspan, Nv)
    }

    fn derivs<const E: usize>(&self, uv: DVec2) -> Vec<Vec<DVec3>> {
        self.surface_derivs::<E>(uv)
    }
}
