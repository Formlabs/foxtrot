#![allow(non_snake_case)]
// This file is translations of algorithms from the 70s, which use awkward
// single-character names everywhere, so we're matching their convention.

use nalgebra_glm::{dot, length, length2, DMat2x2, DVec2, DVec3};
use std::cmp::min;
use std::mem::swap;

fn as_usize_assert(i: i32) -> usize {
    assert!(i >= 0);
    i as usize
}

/// Builds the symmetric matrix [[a, b], [b, d]]
fn symmetric2x2(a: f64, b: f64, d: f64) -> DMat2x2 {
    // In column major order; because it's symmetric, it doesn't matter
    let mut mat = DMat2x2::identity();
    mat.set_column(0, &DVec2::new(a, b));
    mat.set_column(1, &DVec2::new(b, d));
    mat
}

#[derive(Debug, Clone)]
pub struct KnotVector {
    /// Knot positions
    U: Vec<f64>,

    /// Degree of the knot vector
    p: usize,
}

impl KnotVector {
    /// Constructs a new knot vector of over
    pub fn from_multiplicities(p: usize, knots: &[f64], multiplicities: &[usize]) -> Self {
        assert!(knots.len() == multiplicities.len());
        let mut out = Vec::new();
        for (k, m) in knots.iter().zip(multiplicities.iter()) {
            for _ in 0..*m {
                out.push(*k);
            }
        }
        Self { p: p, U: out }
    }

    /// For basis functions of order `p + 1`, finds the span in the knot vector
    /// that is relevant for position `u`.
    ///
    /// ALGORITHM A2.1
    pub fn find_span(&self, u: f64) -> usize {
        // U is [u_0, u_1, ... u_m]
        let m = self.U.len() - 1;
        let n = m - (self.p + 1); // max basis index

        if u == self.U[n + 1] {
            return n;
        }
        let mut low = self.p;
        let mut high = n + 1;
        let mut mid = (low + high) / 2;
        while u < self.U[mid] || u >= self.U[mid + 1] {
            if u < self.U[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        mid
    }

    pub fn degree(&self) -> usize {
        self.p
    }
    pub fn len(&self) -> usize {
        self.U.len()
    }
    pub fn min_t(&self) -> f64 {
        self.U[self.p]
    }
    pub fn max_t(&self) -> f64 {
        self.U[self.U.len() - 1 - self.p]
    }

    /// Computes non-vanishing basis functions of order `p + 1` at point `u`.
    ///
    /// ALGORITHM A2.2
    pub fn basis_funs(&self, u: f64) -> Vec<f64> {
        let i = self.find_span(u);
        self.basis_funs_for_span(i, u)
    }

    // Inner implementation of basis_funs
    pub fn basis_funs_for_span(&self, i: usize, u: f64) -> Vec<f64> {
        let mut N = vec![0.0; self.p + 1];

        let mut left = vec![0.0; self.p + 1];
        let mut right = vec![0.0; self.p + 1];
        N[0] = 1.0;
        for j in 1..=self.p {
            left[j] = u - self.U[i + 1 - j];
            right[j] = self.U[i + j] - u;
            let mut saved = 0.0;
            for r in 0..j {
                let temp = N[r] / (right[r + 1] + left[j - r]);
                N[r] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            N[j] = saved;
        }
        N
    }

    /// Computes the derivatives (up to and including the `nth` derivative) of non-vanishing
    /// basis functions of order `p + 1` at point `u`.
    ///
    /// ALGORITHM A2.3
    /// if ders = basis_funs_derivs_(), then ders[k][j] is the `kth` derivative
    /// of the function `N_{i-p+j, p}` at `u`
    pub fn basis_funs_derivs(&self, u: f64, n: usize) -> Vec<Vec<f64>> {
        let i = self.find_span(u);
        self.basis_funs_derivs_for_span(i, u, n)
    }

    pub fn basis_funs_derivs_for_span(&self, i: usize, u: f64, n: usize) -> Vec<Vec<f64>> {
        let mut ndu = vec![vec![0.0; self.p + 1]; self.p + 1];
        let mut a = vec![vec![0.0; self.p + 1]; 2];
        let mut left = vec![0.0; self.p + 1];
        let mut right = vec![0.0; self.p + 1];

        let mut ders = vec![vec![0.0; self.p + 1]; n + 1];

        ndu[0][0] = 1.0;
        for j in 1..=self.p {
            left[j] = u - self.U[i + 1 - j];
            right[j] = self.U[i + j] - u;
            let mut saved = 0.0;
            for r in 0..j {
                ndu[j][r] = right[r + 1] + left[j - r];
                let temp = ndu[r][j - 1] / ndu[j][r];

                ndu[r][j] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            ndu[j][j] = saved;
        }
        for j in 0..=self.p {
            ders[0][j] = ndu[j][self.p];
        }
        for r in 0..=self.p {
            let mut s1 = 0;
            let mut s2 = 1;
            a[0][0] = 1.0;
            for k in 1..=n {
                let aus = as_usize_assert;
                let mut d = 0.0;
                let rk = (r as i32) - (k as i32);
                let pk = (self.p as i32) - (k as i32);
                if r >= k {
                    a[s2][0] = a[s1][0] / ndu[aus(pk + 1)][rk as usize];
                    d = a[s2][0] * ndu[aus(rk)][aus(pk)];
                }
                let j1 = aus(if rk >= -1 { 1 } else { -rk });
                let j2 = aus(if r as i32 - 1 <= pk as i32 {
                    k as i32 - 1
                } else {
                    self.p as i32 - r as i32
                });

                for j in j1..=j2 {
                    a[s2][j] = (a[s1][j] - a[s1][j - 1]) / ndu[aus(pk + 1)][aus(rk + j as i32)];
                    d += a[s2][j] * ndu[aus(rk + j as i32)][aus(pk)];
                }
                if r as i32 <= pk {
                    a[s2][k] = -a[s1][k - 1] / ndu[aus(pk + 1)][r];
                    d += a[s2][k] * ndu[r][aus(pk)];
                }
                ders[k][r] = d;
                swap(&mut s1, &mut s2);
            }
        }

        let mut r = self.p;
        for k in 1..=n {
            for j in 0..=self.p {
                ders[k][j] *= r as f64;
            }
            r *= self.p - k;
        }
        ders
    }
}


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

        let mut CK = vec![DVec3::zeros(); du + 1];
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

    pub fn uv_from_point(&self, P: DVec3) -> f64 {
        let mut best_score = std::f64::INFINITY;
        let mut best_u = 0.0;

        const N: usize = 8;
        for i in 0..self.knots.len() - 1 {
            // Skip multiple knots
            if self.knots.U[i] == self.knots.U[i + 1] {
                continue;
            }
            // Iterate over a grid within this region
            for u in 0..N {
                let frac = (u as f64) / (N as f64 - 1.0);
                let u = self.knots.U[i] * (1.0 - frac) + self.knots.U[i + 1] * frac;
                
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
}


#[derive(Debug, Clone)]
pub struct BSplineSurface {
    u_open: bool,
    v_open: bool,
    u_knots: KnotVector,
    v_knots: KnotVector,
    control_points: Vec<Vec<DVec3>>,
}

/// Non-rational b-spline surface with 3D control points
impl BSplineSurface {
    pub fn new(
        u_open: bool,
        v_open: bool,
        u_knots: KnotVector,
        v_knots: KnotVector,
        control_points: Vec<Vec<DVec3>>,
    ) -> Self {
        Self {
            u_open,
            v_open,
            u_knots,
            v_knots,
            control_points,
        }
    }

    pub fn min_u(&self) -> f64 {
        self.u_knots.min_t()
    }
    pub fn max_u(&self) -> f64 {
        self.u_knots.max_t()
    }
    pub fn min_v(&self) -> f64 {
        self.v_knots.min_t()
    }
    pub fn max_v(&self) -> f64 {
        self.v_knots.max_t()
    }

    /// Converts a point at position uv onto the 3D mesh, using basis functions
    /// of order `p + 1` and `q + 1` respectively.
    ///
    /// ALGORITHM A3.5
    pub fn surface_point(&self, uv: DVec2) -> DVec3 {
        let p = self.u_knots.degree();
        let q = self.v_knots.degree();

        let uspan = self.u_knots.find_span(uv.x);
        let Nu = self.u_knots.basis_funs_for_span(uspan, uv.x);

        let vspan = self.v_knots.find_span(uv.y);
        let Nv = self.v_knots.basis_funs_for_span(vspan, uv.y);

        let uind = uspan - p;
        let mut S = DVec3::zeros();
        for l in 0..=q {
            let mut temp = DVec3::zeros();
            let vind = vspan - q + l;
            for k in 0..=p {
                temp += Nu[k] * self.control_points[uind + k][vind];
            }
            S += Nv[l] * temp;
        }
        S
    }

    /// Returns all derivatives of the surface.  If `D = surface_derivs()`,
    /// `D[k][l]` is the derivative of the surface `k` times in the `u`
    /// direction and `l` times in the `v` direction.
    ///
    /// We compute derivatives up to and including the `d`'th order derivatives.
    ///
    /// ALGORITHM A3.6
    pub fn surface_derivs<const D: usize>(&self, uv: DVec2) -> Vec<Vec<DVec3>> {
        let p = self.u_knots.degree();
        let q = self.v_knots.degree();

        // Simple initialization of du
        let du = min(D, p);
        let dv = min(D, q);

        // The output matrix goes all the way to order d, even if some of the
        // surfaces are lower order (those values will be locked at 0)
        let mut SKL = vec![vec![DVec3::zeros(); D + 1]; D + 1];

        let uspan = self.u_knots.find_span(uv.x);
        let Nu_deriv = self.u_knots.basis_funs_derivs_for_span(uspan, uv.x, du);

        let vspan = self.v_knots.find_span(uv.y);
        let Nv_deriv = self.v_knots.basis_funs_derivs_for_span(vspan, uv.y, dv);

        let mut temp = vec![DVec3::zeros(); q + 1];
        for k in 0..=du {
            for s in 0..=q {
                temp[s] = DVec3::zeros();
                for r in 0..=p {
                    temp[s] += Nu_deriv[k][r] * self.control_points[uspan - p + r][vspan - q + s];
                }
            }
            let dd = min(D - k, dv);
            for l in 0..=dd {
                for s in 0..=q {
                    SKL[k][l] += Nv_deriv[l][s] * temp[s];
                }
            }
        }
        SKL
    }

    // Section 6.1 (start middle page 232)
    pub fn uv_from_point_newtons_method(&self, P: DVec3, uv_0: DVec2) -> DVec2 {
        let eps1 = 0.01; // a Euclidean distance error bound
        let eps2 = 0.01; // a cosine error bound

        let mut uv_i = uv_0;
        loop {
            // The surface and its derivatives at uv_i
            let derivs = self.surface_derivs::<2>(uv_i);
            let S = derivs[0][0];
            let S_u = derivs[1][0];
            let S_v = derivs[0][1];
            let S_uu = derivs[2][0];
            let S_uv = derivs[1][1]; // S_vu is the same
            let S_vv = derivs[0][2];
            let r = S - P;

            // If |S(uv_i) - P| < \epsilon_1  and
            //    |S_u(uv_i) dot (S(uv_i) - P)| / |S_u(uv_i)| / |S(uv_i) - P| < \epsilon_2  and
            //    |S_v(uv_i) dot (S(uv_i) - P)| / |S_v(uv_i)| / |S(uv_i) - P| < \epsilon_2
            // then we are done
            if length(&r) < eps1
                && dot(&r, &S_u).abs() / length(&S_u) / length(&r) < eps2
                && dot(&r, &S_v).abs() / length(&S_v) / length(&r) < eps2
            {
                return uv_i;
            }

            // Otherwise, compute uv_{i+1} by computing:
            // let r(u, v) = S(u, v) - P
            // let f(u, v) = r(u, v) dot S_u(u, v)
            // let g(u, v) = r(u, v) dot S_v(u, v)
            // let K_i = -(f(uv_{i}), g(uv_{i}))
            // let J_i = [[df/du, df/dv], [dg/du, dg/dv]]
            //           = [[|S_u|^2 + r dot S_uu, S_u dot S_v + r dot S_uv],
            //              [S_u dot S_v + r dot S_vu, |S_v|^2 + r dot S_vv]]
            // let delta_i = (J_i)^{-1} * K_i
            // let uv_{i+1} = delta_i + uv_i
            let f = dot(&r, &S_u);
            let g = dot(&r, &S_v);
            let K_i = -DVec2::new(f, g);
            let J_i = symmetric2x2(
                length2(&S_u) + dot(&r, &S_uu),
                dot(&S_u, &S_v) + dot(&r, &S_uv),
                length2(&S_v) + dot(&r, &S_vv),
            );
            let delta_i = J_i.try_inverse().expect("Could not invert") * K_i;
            let mut uv_ip1 = uv_i + delta_i;

            // clamp uv_{i+p} by doing:
            // if u_{i+1} < min_u: u_{i+1} = min_u if u_open else max_u - (min_u - u_{i+1})
            // if u_{i+1} > max_u: u_{i+1} = max_u if u_open else min_u + (u_{i+1} - max_u)
            // if v_{i+1} < min_v: v_{i+1} = min_v if v_open else max_v - (min_v - v_{i+1})
            // if v_{i+1} > max_v: v_{i+1} = max_v if v_open else min_v + (v_{i+1} - max_v)

            if uv_ip1.x < self.min_u() {
                uv_ip1.x = if self.u_open {
                    self.min_u()
                } else {
                    self.max_u() - (self.min_u() - uv_ip1.x)
                };
            }
            if uv_ip1.x > self.max_u() {
                uv_ip1.x = if self.u_open {
                    self.max_u()
                } else {
                    self.min_u() + (uv_ip1.x - self.max_u())
                };
            }

            if uv_ip1.y < self.min_v() {
                uv_ip1.y = if self.v_open {
                    self.min_v()
                } else {
                    self.max_v() - (self.min_v() - uv_ip1.y)
                };
            }
            if uv_ip1.y > self.max_v() {
                uv_ip1.y = if self.v_open {
                    self.max_v()
                } else {
                    self.min_v() + (uv_ip1.y - self.max_v())
                };
            }

            // If the values didn't change much, we can stop iterating
            // if |(u_{i+1} - u_i) * S_u(u_i, v_i) + (v_{i+1} - v_i) * S_v(u_i, v_i) | < \epsilon_1

            let delta_i = uv_ip1 - uv_i;
            if length(&(delta_i.x * S_u + delta_i.y * S_v)) < eps1 {
                return uv_ip1;
            }

            // otherwise, iterate again
            uv_i = uv_ip1;
        }
    }

    pub fn uv_from_point(&self, p: DVec3) -> DVec2 {
        let mut best_score = std::f64::INFINITY;
        let mut best_uv = DVec2::zeros();

        const N: usize = 8;
        for i in 0..self.u_knots.len() - 1 {
            // Skip multiple knots
            if self.u_knots.U[i] == self.u_knots.U[i + 1] {
                continue;
            }
            for j in 0..self.v_knots.len() - 1 {
                if self.v_knots.U[j] == self.v_knots.U[j + 1] {
                    continue;
                }
                // Iterate over a grid within this region
                for u in 0..N {
                    let frac = (u as f64) / (N as f64 - 1.0);
                    let u = self.u_knots.U[i] * (1.0 - frac) + self.u_knots.U[i + 1] * frac;
                    for v in 0..N {
                        let frac = (v as f64) / (N as f64 - 1.0);
                        let v = self.v_knots.U[j] * (1.0 - frac) + self.v_knots.U[j + 1] * frac;
                        let uv = DVec2::new(u, v);
                        let q = self.surface_point(uv);
                        let score = (p - q).norm();
                        if score < best_score {
                            best_score = score;
                            best_uv = uv;
                        }
                    }
                }
            }
        }
        self.uv_from_point_newtons_method(p, best_uv)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_span() {
        let k = KnotVector {
            U: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        };
        assert!(k.find_span(0, 0.0) == 2);
        assert!(k.find_span(0, 0.99) == 2);
        assert!(k.find_span(1, 0.99) == 2);
        assert!(k.find_span(2, 0.99) == 2);
    }
}
