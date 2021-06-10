use std::cmp::min;
use nalgebra_glm::{dot, length, length2, DVec3};
use crate::KnotVector;

#[derive(Debug, Clone)]
pub struct BSplineCurve {
    open: bool,
    knots: KnotVector,
    control_points: Vec<DVec3>,
}

/// Non-rational b-spline surface with 3D control points
impl BSplineCurve {
    pub fn new(
        open: bool,
        knots: KnotVector,
        control_points: Vec<DVec3>,
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
    pub fn curve_point(&self, u: f64) -> DVec3 {
        let p = self.knots.degree();

        let span = self.knots.find_span(u);
        let N = self.knots.basis_funs_for_span(span, u);

        let mut C = DVec3::zeros();
        for i in 0..=p {
            C += N[i] * self.control_points[span - p + i]
        }
        C
    }

    /// Computes the derivatives of the curve of order up to and including `d` at location `t`,
    /// using basis functions of order `p + 1` respectively.
    ///
    /// ALGORITHM A3.2
    pub fn curve_derivs(&self, u: f64, d: usize) -> Vec<DVec3> {
        let p = self.knots.degree();

        let du = min(d, p);

        let span = self.knots.find_span(u);
        let N_derivs = self.knots.basis_funs_derivs_for_span(span, u, du);

        let mut CK = vec![DVec3::zeros(); d + 1];
        for k in 0..=du {
            for j in 0..=p {
                CK[k] += N_derivs[k][j] * self.control_points[span - p + j]
            }
        }
        CK
    }

    // Section 6.1 (start middle page 232)
    pub fn u_from_point_newtons_method(&self, P: DVec3, u_0: f64) -> f64 {
        let eps1 = 0.01; // a Euclidean distance error bound
        let eps2 = 0.01; // a cosine error bound

        let mut u_i = u_0;
        loop {
            let derivs = self.curve_derivs(u_i, 2);
            let C = derivs[0];
            let C_p = derivs[1];
            let C_pp = derivs[2];
            let r = C - P;

            // If we are close to the point and close to the right angle, then return
            if length(&r) <= eps1 && dot(&C_p, &r) / length(&C_p) / length(&r) <= eps2 {
                return u_i;
            }

            // calculate the next `u`
            // let f(u) = C'(u) dot (C(u) - P)
            // u_{ip1} = u_i - (f(u_i) / f'(u_i)) = u_i - (C'(u_i) dot (C(u_i) - P)) / (C''(u_i) dot (C(u_i) - P) + |C'(u_i)|^2)
            let delta_i = -dot(&C_p, &r) / (dot(&C_pp, &r) + length2(&C_p));
            let mut u_ip1 = u_i + delta_i;

            // clamp the `u` onto the curve
            if u_ip1 < self.min_u() {
                u_ip1 = if self.open {
                    self.min_u()
                } else {
                    self.max_u() - (self.min_u() - u_ip1)
                };
            }
            if u_ip1 > self.max_u() {
                u_ip1 = if self.open {
                    self.max_u()
                } else {
                    self.min_u() + (u_ip1 - self.max_u())
                };
            }

            // if the point didnt move much, return
            if length(&((u_ip1 - u_i) * C_p)) <= eps1 {
                return u_ip1;
            }

            u_i = u_ip1;
        }
    }

    pub fn u_from_point(&self, P: DVec3) -> f64 {
        let mut best_score = std::f64::INFINITY;
        let mut best_u = 0.0;

        const N: usize = 8;
        for i in 0..self.knots.len() - 1 {
            // Skip multiple knots
            if self.knots[i] == self.knots[i + 1] {
                continue;
            }
            // Iterate over a grid within this region
            for u in 0..N {
                let frac = (u as f64) / (N as f64 - 1.0);
                let u = self.knots[i] * (1.0 - frac) + self.knots[i + 1] * frac;

                let q = self.curve_point(u);
                let score = (P - q).norm();
                if score < best_score {
                    best_score = score;
                    best_u = u;
                }
            }
        }
        self.u_from_point_newtons_method(P, best_u)
    }


    pub fn as_polyline(&self, u_start: f64, u_end: f64, num_points_per_knot: usize) -> Vec<DVec3> {
        let (u_min, u_max) = if u_start < u_end { (u_start, u_end) } else { (u_end, u_start) };

        let mut result: Vec<DVec3> = Vec::new();
        result.push(self.curve_point(u_min));

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

