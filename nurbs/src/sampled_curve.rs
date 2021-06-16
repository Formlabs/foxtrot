use nalgebra_glm::{dot, length, length2, DVec3};
use crate::{abstract_curve::AbstractCurve, nd_curve::NDBSplineCurve};

#[derive(Debug)]
pub struct SampledCurve<const N: usize> {
    curve: NDBSplineCurve<N>,
    samples: Vec<(f64, DVec3)>,
}

impl<const N: usize> SampledCurve<N>
    where NDBSplineCurve<N>: AbstractCurve
{
    pub fn new(curve: NDBSplineCurve<N>) -> Self {
        const N: usize = 8;
        let mut samples = Vec::new();
        for i in 0..curve.knots.len() - 1 {
            // Skip multiple knots
            if curve.knots[i] == curve.knots[i + 1] {
                continue;
            }
            // Iterate over a grid within this region
            for u in 0..N {
                let frac = (u as f64) / (N as f64 - 1.0);
                let u = curve.knots[i] * (1.0 - frac) + curve.knots[i + 1] * frac;

                let q = curve.point(u);
                samples.push((u, q));
            }
        }

        Self { curve, samples }
    }

    // Section 6.1 (start middle page 232)
    pub fn u_from_point_newtons_method(&self, P: DVec3, u_0: f64) -> f64 {
        let eps1 = 0.01; // a Euclidean distance error bound
        let eps2 = 0.01; // a cosine error bound

        let mut u_i = u_0;
        loop {
            let derivs = self.curve.derivs::<2>(u_i);
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
            if u_ip1 < self.curve.min_u() {
                u_ip1 = if self.curve.open {
                    self.curve.min_u()
                } else {
                    self.curve.max_u() - (self.curve.min_u() - u_ip1)
                };
            }
            if u_ip1 > self.curve.max_u() {
                u_ip1 = if self.curve.open {
                    self.curve.max_u()
                } else {
                    self.curve.min_u() + (u_ip1 - self.curve.max_u())
                };
            }

            // if the point didnt move much, return
            if length(&((u_ip1 - u_i) * C_p)) <= eps1 {
                return u_ip1;
            }

            u_i = u_ip1;
        }
    }

    pub fn u_from_point(&self, p: DVec3) -> f64 {
        use ordered_float::OrderedFloat;
        let best_u = self.samples.iter()
            .min_by_key(|(_u, pos)| OrderedFloat((pos - p).norm()))
            .unwrap().0;
        self.u_from_point_newtons_method(p, best_u)
    }

    pub fn as_polyline(&self, u_start: f64, u_end: f64, num_points_per_knot: usize) -> Vec<DVec3> {
        let (u_min, u_max) = if u_start < u_end {
            (u_start, u_end)
        } else {
            (u_end, u_start)
        };

        let mut result = vec![self.curve.point(u_min)];

        // TODO this could be faster if we skip to the right start/end sections

        assert!(num_points_per_knot > 0);
        for i in 0..self.curve.knots.len() - 1 {
            // Skip multiple knots
            if self.curve.knots[i] == self.curve.knots[i + 1] {
                continue;
            }
            // Iterate over a grid within this region
            for u in 0..num_points_per_knot {
                let frac = (u as f64) / (num_points_per_knot as f64);
                let u = self.curve.knots[i] * (1.0 - frac) + self.curve.knots[i + 1] * frac;
                if u > u_min && u < u_max {
                    result.push(self.curve.point(u));
                }
            }
        }
        result.push(self.curve.point(u_max));

        if u_start > u_end {
            result.reverse();
        }
        result
    }
}
