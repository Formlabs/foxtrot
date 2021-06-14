use nalgebra_glm::{dot, length, length2, DMat2x2, DVec2, DVec3};
use crate::{abstract_surface::AbstractSurface, nd_surface::NDBSplineSurface};
use log::error;

#[derive(Debug, Clone)]
pub struct SampledSurface<const N: usize> {
    pub surf: NDBSplineSurface<N>,
    samples: Vec<(DVec2, DVec3)>,
}

impl<const N: usize> SampledSurface<N>
    where NDBSplineSurface<N>: AbstractSurface
{
    pub fn new(surf: NDBSplineSurface<N>) -> Self {
        const N: usize = 8;
        let mut samples = Vec::new();
        for i in 0..surf.u_knots.len() - 1 {
            // Skip multiple knots
            if surf.u_knots[i] == surf.u_knots[i + 1] {
                continue;
            }
            for j in 0..surf.v_knots.len() - 1 {
                if surf.v_knots[j] == surf.v_knots[j + 1] {
                    continue;
                }
                // Iterate over a grid within this region
                for u in 0..N {
                    let frac = (u as f64) / (N as f64 - 1.0);
                    let u = surf.u_knots[i] * (1.0 - frac) + surf.u_knots[i + 1] * frac;

                    // Cache the u basis function outside the loop
                    let u_span = surf.u_knots.find_span(u);
                    let u_basis = surf.u_knots.basis_funs_for_span(u_span, u);
                    for v in 0..N {
                        let frac = (v as f64) / (N as f64 - 1.0);
                        let v = surf.v_knots[j] * (1.0 - frac) + surf.v_knots[j + 1] * frac;
                        let uv = DVec2::new(u, v);

                        let v_span = surf.v_knots.find_span(v);
                        let v_basis = surf.v_knots.basis_funs_for_span(v_span, v);
                        let q = surf.point_from_basis(
                            u_span, &u_basis, v_span, &v_basis);
                        samples.push((uv, q));
                    }
                }
            }
        }
        Self { surf, samples }
    }

    // Section 6.1 (start middle page 232)
    pub fn uv_from_point_newtons_method(&self, P: DVec3, uv_0: DVec2) -> Option<DVec2> {
        let eps1 = 0.01; // a Euclidean distance error bound
        let eps2 = 0.01; // a cosine error bound

        let mut uv_i = uv_0;
        for _ in 0..256 {
            // The surface and its derivatives at uv_i
            let derivs = self.surf.derivs::<2>(uv_i);
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

            if uv_ip1.x < self.surf.min_u() {
                uv_ip1.x = if self.surf.u_open {
                    self.surf.min_u()
                } else {
                    self.surf.max_u() - (self.surf.min_u() - uv_ip1.x)
                };
            }
            if uv_ip1.x > self.surf.max_u() {
                uv_ip1.x = if self.surf.u_open {
                    self.surf.max_u()
                } else {
                    self.surf.min_u() + (uv_ip1.x - self.surf.max_u())
                };
            }

            if uv_ip1.y < self.surf.min_v() {
                uv_ip1.y = if self.surf.v_open {
                    self.surf.min_v()
                } else {
                    self.surf.max_v() - (self.surf.min_v() - uv_ip1.y)
                };
            }
            if uv_ip1.y > self.surf.max_v() {
                uv_ip1.y = if self.surf.v_open {
                    self.surf.max_v()
                } else {
                    self.surf.min_v() + (uv_ip1.y - self.surf.max_v())
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
        error!("Could not find UV coordinates");
        None
    }

    pub fn uv_from_point(&self, p: DVec3) -> Option<DVec2> {
        assert!(!self.samples.is_empty());
        use ordered_float::OrderedFloat;
        let best_uv = self.samples.iter()
            .min_by_key(|(_uv, pos)| OrderedFloat((pos - p).norm()))
            .unwrap().0;
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
