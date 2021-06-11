#![allow(non_snake_case)]
// This crate is translations of algorithms from the 70s, which use awkward
// single-character names everywhere, so we're matching their convention.

mod abstract_curve;
mod bspline_curve;
mod bspline_surf;
mod knot_vector;
mod nd_curve;
mod nurbs_curve;
mod sampled_curve;

use smallvec::{SmallVec};
type VecF = SmallVec<[f64; 8]>;

pub use crate::abstract_curve::AbstractCurve;
pub use crate::bspline_curve::BSplineCurve;
pub use crate::bspline_surf::BSplineSurface;
pub use crate::knot_vector::KnotVector;
pub use crate::nd_curve::NDBSplineCurve;
pub use crate::nurbs_curve::NURBSCurve;
pub use crate::sampled_curve::SampledCurve;
