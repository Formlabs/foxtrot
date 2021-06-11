use nalgebra_glm::{DVec3};
use crate::{nd_curve::NDBSplineCurve, curve_traits::AbstractCurve};

pub type NURBSCurve = NDBSplineCurve<4>;

impl AbstractCurve for NURBSCurve {
    /// Converts a point at position t onto the 3D line, using basis functions
    /// of order `p + 1` respectively.
    ///
    /// ALGORITHM A4.1
    fn point(&self, u: f64) -> DVec3 {
        let p = self.curve_point(u);
        p.xyz() / p.w
    }

    /// Computes the derivatives of the curve of order up to and including `d` at location `t`,
    /// using basis functions of order `p + 1` respectively.
    ///
    /// ALGORITHM A4.2
    fn derivs(&self, u: f64, d: usize) -> Vec<DVec3> {
        let derivs = self.curve_derivs(u, d);
        unimplemented!()
    }
}

