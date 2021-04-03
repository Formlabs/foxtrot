use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::predicates::{circumcenter, circumradius2, distance2, orient2d};
use crate::Point;

// Calculates a seed triangle from the given set of points
// TODO: make robust to < 3 points and colinear inputs
pub fn seed_triangle(pts: &[Point]) -> (usize, usize, usize) {
    let x_bounds = pts.iter().map(|p| p.0).minmax().into_option().unwrap();
    let y_bounds = pts.iter().map(|p| p.1).minmax().into_option().unwrap();

    let center = ((x_bounds.0 + x_bounds.1) / 2.0,
                  (y_bounds.0 + y_bounds.1) / 2.0);

    // Pick the initial triangle, with
    //  a) the point closest to the center
    //  b) the point closest to a
    //  c) the point with the minimum circumradius
    let a = pts.iter()
        .position_min_by_key(
            |q| OrderedFloat(distance2(center, **q)))
        .expect("Could not get initial point");
    let b = pts.iter().enumerate()
        .filter(|(j, _)| *j != a)
        .position_min_by_key(
            |(_, p)| OrderedFloat(distance2(pts[a], **p)))
        .expect("Could not get second point");
    let c = pts.iter().enumerate()
        .filter(|(j, _)| *j != a && *j != b)
        .position_min_by_key(
            |(_, p)| OrderedFloat(circumradius2(pts[a], pts[b], **p)))
        .expect("Could not get third point");

    if orient2d(pts[a], pts[b], pts[c]) > 0.0 {
        (a, b, c)
    } else {
        (a, c, b)
    }
}

pub fn triangulate(pts: &[Point]) -> Vec<(usize, usize, usize)> {
    let seed = seed_triangle(pts);
    let center = circumcenter(pts[seed.0], pts[seed.1], pts[seed.2]);
    let mut pts: Vec<Point> = pts.iter().cloned().collect();
    pts.sort_by_key(|p| OrderedFloat(distance2(center, *p)));
    unimplemented!()
}

