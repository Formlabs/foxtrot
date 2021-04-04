use std::collections::BTreeMap;
use std::num::NonZeroUsize;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::predicates::{circumcenter, circumradius2, distance2, orient2d, pseudo_angle};
use crate::Point;

#[derive(Copy, Clone)]
struct PointIndex(usize);

#[derive(Copy, Clone)]
struct EdgeIndex(NonZeroUsize);

#[derive(Copy, Clone)]
struct Edge {
    src: PointIndex,
    dst: PointIndex,
    buddy: Option<EdgeIndex>,
}


struct Triangulation<'a> {
    points: &'a[Point],
    order: Vec<usize>, // Ordering of the points, from inner to outer

    // This stores the start of an edge (as a pseudoangle) as an index into
    // the edges array
    hull: BTreeMap<OrderedFloat<f64>, EdgeIndex>,
    edges: Vec<Edge>,
}

impl<'a> Triangulation<'a> {
    pub fn new(points: &'a [Point]) -> Triangulation<'a> {
        let seed = Self::seed_triangle(points);

        let center = circumcenter(points[seed.0], points[seed.1], points[seed.2]);
        let mut pts: Vec<(usize, Point)> = points.iter()
            .cloned()
            .enumerate()
            .collect();
        pts.sort_by_key(|p| OrderedFloat(distance2(center, p.1)));
        let order: Vec<usize> = pts.iter().map(|p| p.0).collect();

        let mut out = Triangulation {
            points,
            order,

            // edges[0] is a reserved slot, so other edges can use None (0)
            // to indicate when they don't have a matched pair.
            edges: vec![Edge { src: PointIndex(0), dst: PointIndex(0), buddy: None }],
            hull: BTreeMap::new(),
        };
        let e_ab = out.push_edge(PointIndex(seed.0), PointIndex(seed.1), None);
        let e_bc = out.push_edge(PointIndex(seed.1), PointIndex(seed.2), None);
        let e_ca = out.push_edge(PointIndex(seed.2), PointIndex(seed.0), None);

        out.hull.insert(OrderedFloat(pseudo_angle(points[seed.0])), e_ab);
        out.hull.insert(OrderedFloat(pseudo_angle(points[seed.1])), e_bc);
        out.hull.insert(OrderedFloat(pseudo_angle(points[seed.2])), e_ca);

        out
    }

    fn push_edge(&mut self, src: PointIndex, dst: PointIndex, buddy: Option<EdgeIndex>) -> EdgeIndex {
        let n = self.edges.len();
        // We assume that this edge will produce a new triangle soon
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
}

