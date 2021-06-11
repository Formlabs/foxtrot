use nalgebra_glm::{DVec3};
use crate::{nd_curve::NDBSplineCurve, abstract_curve::AbstractCurve};

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
    fn derivs<const E: usize>(&self, u: f64) -> Vec<DVec3> {
        let derivs = self.curve_derivs::<E>(u);
        let mut CK = vec![DVec3::zeros(); E + 1];
        for k in 0..=E {
            let mut v = derivs[k].xyz();
            for i in 1..=k {
                let b = num_integer::binomial(k, i);
                v -= b as f64 * derivs[i].w * CK[k - 1];
            }
            CK[k] = v / derivs[0].w;
        }
        CK
    }
}

