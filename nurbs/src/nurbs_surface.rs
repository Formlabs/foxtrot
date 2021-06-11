use nalgebra_glm::{DVec2, DVec3};
use crate::{abstract_surface::AbstractSurface, nd_surface::NDBSplineSurface, VecF};

pub type NURBSSurface = NDBSplineSurface<4>;

impl AbstractSurface for NURBSSurface {
    fn point(&self, uv: DVec2) -> DVec3 {
        let p = self.surface_point(uv);
        p.xyz() / p.w
    }
    fn point_from_basis(&self, uspan: usize, Nu: &VecF,
                               vspan: usize, Nv: &VecF) -> DVec3
    {
        let p = self.surface_point_from_basis(uspan, Nu, vspan, Nv);
        p.xyz() / p.w
    }

    fn derivs<const E: usize>(&self, uv: DVec2) -> Vec<Vec<DVec3>> {
        let derivs = self.surface_derivs::<E>(uv);
        let mut SKL = vec![vec![DVec3::zeros(); E + 1]; E + 1];
        let bin = |a, b| num_integer::binomial(a, b) as f64;
        for k in 0..=E {
            for l in 0..=(E - k) {
                let mut v = derivs[k][l].xyz();
                for j in 1..=l {
                    v -= bin(l, j) * derivs[0][j].w * SKL[k][l - j];
                }
                for i in 1..=k {
                    v -= bin(k, i) * derivs[i][0].w * SKL[k - i][l];
                    let mut v2 = DVec3::zeros();
                    for j in 1..=l {
                        v2 += bin(l, j) * derivs[i][j].w * SKL[k - i][l - j];
                    }
                    v -= bin(k, i) * v2;
                }
                SKL[k][l] = v / derivs[0][0].w;
            }
        }
        SKL
    }
}

