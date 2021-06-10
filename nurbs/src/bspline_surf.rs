use std::cmp::min;
use nalgebra_glm::{dot, length, length2, DVec2, DMat2x2, DVec3};
use crate::{KnotVector, VecF};

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
        let uspan = self.u_knots.find_span(uv.x);
        let Nu = self.u_knots.basis_funs_for_span(uspan, uv.x);

        let vspan = self.v_knots.find_span(uv.y);
        let Nv = self.v_knots.basis_funs_for_span(vspan, uv.y);

        self.surface_point_from_basis(uspan, &Nu, vspan, &Nv)
    }

    pub fn surface_point_from_basis(&self,
        uspan: usize, Nu: &VecF,
        vspan: usize, Nv: &VecF) -> DVec3
    {
        let p = self.u_knots.degree();
        let q = self.v_knots.degree();

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
    pub fn uv_from_point_newtons_method(&self, P: DVec3, uv_0: DVec2) -> Option<DVec2> {
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
                return Some(uv_i);
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
            let delta_i = match J_i.try_inverse() {
                None => return None,
                Some(m) => m * K_i,
            };
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
                return Some(uv_ip1);
            }

            // otherwise, iterate again
            uv_i = uv_ip1;
        }
    }

    pub fn uv_from_point(&self, p: DVec3) -> Option<DVec2> {
        let mut best_score = std::f64::INFINITY;
        let mut best_uv = DVec2::zeros();

        const N: usize = 8;
        for i in 0..self.u_knots.len() - 1 {
            // Skip multiple knots
            if self.u_knots[i] == self.u_knots[i + 1] {
                continue;
            }
            for j in 0..self.v_knots.len() - 1 {
                if self.v_knots[j] == self.v_knots[j + 1] {
                    continue;
                }
                // Iterate over a grid within this region
                for u in 0..N {
                    let frac = (u as f64) / (N as f64 - 1.0);
                    let u = self.u_knots[i] * (1.0 - frac) + self.u_knots[i + 1] * frac;

                    // Cache the u basis function outside the loop
                    let u_span = self.u_knots.find_span(u);
                    let u_basis = self.u_knots.basis_funs_for_span(u_span, u);
                    for v in 0..N {
                        let frac = (v as f64) / (N as f64 - 1.0);
                        let v = self.v_knots[j] * (1.0 - frac) + self.v_knots[j + 1] * frac;
                        let uv = DVec2::new(u, v);

                        let v_span = self.v_knots.find_span(v);
                        let v_basis = self.v_knots.basis_funs_for_span(v_span, v);
                        let q = self.surface_point_from_basis(
                            u_span, &u_basis, v_span, &v_basis);
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

/// Builds the symmetric matrix [[a, b], [b, d]]
fn symmetric2x2(a: f64, b: f64, d: f64) -> DMat2x2 {
    // In column major order; because it's symmetric, it doesn't matter
    let mut mat = DMat2x2::identity();
    mat.set_column(0, &DVec2::new(a, b));
    mat.set_column(1, &DVec2::new(b, d));
    mat
}
