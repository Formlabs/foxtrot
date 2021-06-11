use nalgebra_glm::DVec3;
use crate::KnotVector;

pub trait AbstractCurve {
    fn point(&self, u: f64) -> DVec3;
    fn derivs(&self, u: f64, d: usize) -> Vec<DVec3>;
}

pub trait CurveWithKnots {
    fn knots(&self) -> &KnotVector;
    fn open(&self) -> bool;

    fn min_u(&self) -> f64 {
        self.knots().min_t()
    }
    fn max_u(&self) -> f64 {
        self.knots().max_t()
    }
}
