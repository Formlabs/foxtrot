use std::collections::BTreeMap;
use std::num::NonZeroUsize;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::predicates::{circumcenter, circumradius2, distance2, orient2d, pseudo_angle};
use crate::Point;

#[derive(Copy, Clone, Debug)]
struct PointIndex(usize);

#[derive(Copy, Clone, Debug)]
struct EdgeIndex(NonZeroUsize);

#[derive(Copy, Clone, Debug)]
struct Edge {
    src: PointIndex,
    dst: PointIndex,
    buddy: Option<EdgeIndex>,
}

struct Triangulation<'a> {
    points: &'a[Point],
    center: Point,
    order: Vec<usize>, // Ordering of the points, from inner to outer

    // This stores the start of an edge (as a pseudoangle) as an index into
    // the edges array
    hull: BTreeMap<OrderedFloat<f64>, EdgeIndex>,
    edges: Vec<Edge>,
}

impl<'a> Triangulation<'a> {
    pub fn new(points: &'a [Point]) -> Triangulation<'a> {
        let seed = Self::seed_triangle(points);
        println!("{:?}", seed);
        let center = circumcenter(points[seed.0], points[seed.1], points[seed.2]);
        let mut pts: Vec<(usize, Point)> = points.iter()
            .cloned()
            .enumerate()
            .collect();
        pts.sort_by_key(|p| OrderedFloat(distance2(center, p.1)));
        let order: Vec<usize> = pts.iter()
            .map(|p| p.0)
            .filter(|&i| i != seed.0 && i != seed.1 && i != seed.2)
            .collect();

        let mut out = Triangulation {
            points,
            order,
            center,

            // edges[0] is a reserved slot, so other edges can use None (0)
            // to indicate when they don't have a matched pair.
            edges: vec![Edge { src: PointIndex(0), dst: PointIndex(0), buddy: None }],
            hull: BTreeMap::new(),
        };

        let pa = PointIndex(seed.0);
        let pb = PointIndex(seed.1);
        let pc = PointIndex(seed.2);

        let e_ab = out.push_edge(pa, pb, None);
        let e_bc = out.push_edge(pb, pc, None);
        let e_ca = out.push_edge(pc, pa, None);

        out.hull.insert(out.key(pa), e_ab);
        out.hull.insert(out.key(pb), e_bc);
        out.hull.insert(out.key(pc), e_ca);

        out
    }

    fn key(&self, p: PointIndex) -> OrderedFloat<f64> {
        let p = self.points[p.0];
        OrderedFloat(crate::predicates::pseudo_angle(
            (p.0 - self.center.0, p.1 - self.center.1)))
    }

    fn push_edge(&mut self, src: PointIndex, dst: PointIndex, buddy: Option<EdgeIndex>) -> EdgeIndex {
        let n = self.edges.len();
        self.edges.push(Edge {src, dst, buddy});
        EdgeIndex(NonZeroUsize::new(n).unwrap())
    }

    fn edge(&self, e: EdgeIndex) -> Edge {
        self.edges[e.0.get()]
    }

    fn point(&self, p: PointIndex) -> Point {
        self.points[p.0]
    }

    // Calculates a seed triangle from the given set of points
    // TODO: make robust to < 3 points and colinear inputs
    fn seed_triangle(pts: &[Point]) -> (usize, usize, usize) {
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
            .position_min_by_key(
                |(j, p)| OrderedFloat(if *j == a {
                    std::f64::INFINITY
                } else {
                    distance2(pts[a], **p)
                }))
            .expect("Could not get second point");
        let c = pts.iter().enumerate()
            .position_min_by_key(
                |(j, p)| OrderedFloat(if *j == a || *j == b {
                    std::f64::INFINITY
                } else {
                    circumradius2(pts[a], pts[b], **p)
                }))
            .expect("Could not get third point");

        if orient2d(pts[a], pts[b], pts[c]) > 0.0 {
            (a, b, c)
        } else {
            (a, c, b)
        }
    }

    /// Returns the edge of the bounding hull which the given point projects
    /// onto, as an index into self.edges.
    fn get_hull_edge(&self, p: PointIndex) -> (f64, Option<EdgeIndex>) {
        use std::ops::Bound::*;
        let k = self.key(p);
        let mut r = self.hull.range((Unbounded, Included(k)));
        (k.into_inner(), r.next_back().map(|p| *p.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let pts = vec![
            (0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (2.0, 2.0)
        ];
        let t = Triangulation::new(&pts);
        assert_eq!(t.order.len(), 1);
        assert_eq!(t.order[0], 3);
        for i in 0..4 {
            println!("{}: {:?}, {}", i, pts[i], t.key(PointIndex(i)));
        }
        println!("{:?}", t.hull.range((std::ops::Bound::Unbounded,
                std::ops::Bound::Included(t.key(PointIndex(3))))));
    }
}
