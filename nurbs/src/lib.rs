#![allow(non_snake_case)]
// This file is translations of algorithms from the 70s, which use awkward
// single-character names everywhere, so we're matching their convention.

use nalgebra_glm::{DVec2, DVec3};

#[derive(Debug, Clone)]
pub struct KnotVector {
    U: Vec<f64>,
}

impl KnotVector {
    pub fn from_multiplicities(knots: &[f64], multiplicities: &[usize]) -> Self {
        assert!(knots.len() == multiplicities.len());
        let mut out = Vec::new();
        for (k, m) in knots.iter().zip(multiplicities.iter()) {
            for _ in 0..*m {
                out.push(*k);
            }
        }
        Self { U: out }
    }

    /// For basis functions of order `p + 1`, finds the span in the knot vector
    /// that is relevant for position `u`.
    ///
    /// ALGORITHM A2.1
    pub fn find_span(&self, p: usize, u: f64) -> usize {
        // U is [u_0, u_1, ... u_m]
        let m = self.U.len() - 1;
        let n = m - (p + 1); // max basis index

        if u == self.U[n + 1] {
            return n;
        }
        let mut low = p;
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

    /// Computes non-vanishing basis functions of order `p + 1` at point `u`.
    ///
    /// ALGORITHM A2.2
    pub fn basis_funs(&self, p: usize, u: f64) -> Vec<f64> {
        let i = self.find_span(p, u);
        self.basis_funs_(i, p, u)
    }

    pub fn basis_funs_(&self, i: usize, p: usize, u: f64) -> Vec<f64> {
        let mut N = vec![0.0; p + 1];

        let mut left = vec![0.0; p + 1];
        let mut right = vec![0.0; p + 1];
        N[0] = 1.0;
        for j in 1..=p {
            left[j] = u - self.U[i + 1 - j];
            right[j] = self.U[i + j] - u;
            let mut saved = 0.0;
            for r in 0..j {
                let temp = N[r] / (right[r + 1] + left[j - r]);
                N[r] = saved + right[r + 1] * temp;
                saved = left[j - r]*temp;
            }
            N[j] = saved;
        }
        N
    }
}

#[derive(Debug, Clone)]
pub struct BSplineSurface {
    u_knots: KnotVector,
    v_knots: KnotVector,
    control_points: Vec<Vec<DVec3>>,
}

/// Non-rational b-spline surface with 3D control points
impl BSplineSurface {
    pub fn new(u_knots: KnotVector, v_knots: KnotVector,
               control_points: Vec<Vec<DVec3>>) -> Self {
        Self { u_knots, v_knots, control_points }
    }
    /// Converts a point at position uv onto the 3D mesh, using basis functions
    /// of order `p + 1` and `u + 1` respectively.
    ///
    /// ALGORITHM A3.5
    pub fn surface_point(&self, p: usize, q: usize, uv: DVec2) -> DVec3 {
        let uspan = self.u_knots.find_span(p, uv.x);
        let Nu = self.u_knots.basis_funs_(uspan, p, uv.x);

        let vspan = self.v_knots.find_span(q, uv.y);
        let Nv = self.v_knots.basis_funs_(vspan, q, uv.y);

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
}


////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_span() {
        let k = KnotVector { U: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0] };
        assert!(k.find_span(0, 0.0) == 2);
        assert!(k.find_span(0, 0.99) == 2);
        assert!(k.find_span(1, 0.99) == 2);
        assert!(k.find_span(2, 0.99) == 2);
    }
}
