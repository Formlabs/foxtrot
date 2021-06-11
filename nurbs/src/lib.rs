#![allow(non_snake_case)]
// This crate is translations of algorithms from the 70s, which use awkward
// single-character names everywhere, so we're matching their convention.

mod knot_vector;
mod bspline_curve;
mod bspline_surf;
mod nd_curve;
mod curve_traits;
mod sampled_curve;

use smallvec::{SmallVec};
type VecF = SmallVec<[f64; 8]>;

pub use crate::knot_vector::KnotVector;
pub use crate::bspline_curve::BSplineCurve;
pub use crate::sampled_curve::SampledCurve;
pub use crate::bspline_surf::BSplineSurface;
