use std::cmp::min;
use nalgebra_glm::TVec;
use crate::KnotVector;

#[derive(Debug, Clone)]
pub struct NDBsplineSurface<const D: usize> {
    u_open: bool,
    v_open: bool,
    u_knots: KnotVector,
    v_knots: KnotVector,
    control_points: Vec<Vec<TVec<f64, D>>>,
}

/// Non-rational b-spline surface with 3D control points
impl<const D: usize> BSplineSurface<D> {
    pub fn new(
        u_open: bool,
        v_open: bool,
        u_knots: KnotVector,
        v_knots: KnotVector,
        control_points: Vec<Vec<TVec<f64, D>>>,
    ) -> Self {
        Self {
            u_open,
            v_open,
            u_knots,
            v_knots,
            control_points,
        };
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
    pub fn surface_point(&self, uv: DVec2) -> TVec<f64, D> {
        let uspan = self.u_knots.find_span(uv.x);
        let Nu = self.u_knots.basis_funs_for_span(uspan, uv.x);

        let vspan = self.v_knots.find_span(uv.y);
        let Nv = self.v_knots.basis_funs_for_span(vspan, uv.y);

        self.surface_point_from_basis(uspan, &Nu, vspan, &Nv)
    }

    pub fn surface_point_from_basis(&self,
        uspan: usize, Nu: &VecF,
        vspan: usize, Nv: &VecF) -> TVec<f64, D>
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
    pub fn surface_derivs<const E: usize>(&self, uv: DVec2) -> Vec<Vec<TVec<f64, D>>> {
        let p = self.u_knots.degree();
        let q = self.v_knots.degree();

        // Simple initialization of du
        let du = min(E, p);
        let dv = min(E, q);

        // The output matrix goes all the way to order d, even if some of the
        // surfaces are lower order (those values will be locked at 0)
        let mut SKL = vec![vec![TVec::zeros(); E + 1]; E + 1];

        let uspan = self.u_knots.find_span(uv.x);
        let Nu_deriv = self.u_knots.basis_funs_derivs_for_span(uspan, uv.x, du);

        let vspan = self.v_knots.find_span(uv.y);
        let Nv_deriv = self.v_knots.basis_funs_derivs_for_span(vspan, uv.y, dv);

        let mut temp = vec![TVec::zeros(); q + 1];
        for k in 0..=du {
            for s in 0..=q {
                temp[s] = TVec::zeros();
                for r in 0..=p {
                    temp[s] += Nu_deriv[k][r] * self.control_points[uspan - p + r][vspan - q + s];
                }
            }
            let dd = min(E - k, dv);
            for l in 0..=dd {
                for s in 0..=q {
                    SKL[k][l] += Nv_deriv[l][s] * temp[s];
                }
            }
        }
        SKL
    }
}
