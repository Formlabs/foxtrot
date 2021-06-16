use std::cmp::min;
use nalgebra_glm::TVec;
use crate::KnotVector;

#[derive(Debug, Clone)]
pub struct NDBSplineCurve<const D: usize> {
    pub open: bool,
    pub knots: KnotVector,
    control_points: Vec<TVec<f64, D>>,
}

/// Abstract b-spline curve with N-dimensional control points
impl<const D: usize> NDBSplineCurve<D> {
    pub fn new(
        open: bool,
        knots: KnotVector,
        control_points: Vec<TVec<f64, D>>,
    ) -> Self {
        Self {
            open,
            knots,
            control_points,
        }
    }

    pub fn min_u(&self) -> f64 {
        self.knots.min_t()
    }
    pub fn max_u(&self) -> f64 {
        self.knots.max_t()
    }

    /// Converts a point at position t onto the 3D line, using basis functions
    /// of order `p + 1` respectively.
    ///
    /// ALGORITHM A3.1
    pub fn curve_point(&self, u: f64) -> TVec<f64, D> {
        let p = self.knots.degree();

        let span = self.knots.find_span(u);
        let N = self.knots.basis_funs_for_span(span, u);

        let mut C = TVec::zeros();
        for i in 0..=p {
            C += N[i] * self.control_points[span - p + i]
        }
        C
    }

    /// Computes the derivatives of the curve of order up to and including `d` at location `t`,
    /// using basis functions of order `p + 1` respectively.
    ///
    /// ALGORITHM A3.2
    pub fn curve_derivs<const E: usize>(&self, u: f64) -> Vec<TVec<f64, D>> {
        let p = self.knots.degree();

        let du = min(E, p);

        let span = self.knots.find_span(u);
        let N_derivs = self.knots.basis_funs_derivs_for_span(span, u, du);

        let mut CK = vec![TVec::zeros(); E + 1];
        for k in 0..=du {
            for j in 0..=p {
                CK[k] += N_derivs[k][j] * self.control_points[span - p + j]
            }
        }
        CK
    }

    pub fn as_polyline(&self, u_start: f64, u_end: f64, num_points_per_knot: usize) -> Vec<TVec<f64, D>> {
        let (u_min, u_max) = if u_start < u_end {
            (u_start, u_end)
        } else {
            (u_end, u_start)
        };

        let mut result = vec![self.curve_point(u_min)];

        // TODO this could be faster if we skip to the right start/end sections

        assert!(num_points_per_knot > 0);
        for i in 0..self.knots.len() - 1 {
            // Skip multiple knots
            if self.knots[i] == self.knots[i + 1] {
                continue;
            }
            // Iterate over a grid within this region
            for u in 0..num_points_per_knot {
                let frac = (u as f64) / (num_points_per_knot as f64);
                let u = self.knots[i] * (1.0 - frac) + self.knots[i + 1] * frac;
                if u > u_min && u < u_max {
                    result.push(self.curve_point(u));
                }
            }
        }
        result.push(self.curve_point(u_max));

        if u_start > u_end {
            result.reverse();
        }
        result
    }
}
