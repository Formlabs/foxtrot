use std::collections::BTreeMap;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::predicates::{circumcenter, circumradius2, distance2, orient2d, pseudo_angle};
use crate::{Point, PointIndex, EdgeIndex};
use crate::half::Half;

#[derive(Default)]
pub struct Triangulation<'a> {
    points: &'a[Point],
    center: Point,
    order: Vec<PointIndex>,  // Ordering of the points, from inner to outer
    next: usize,        // Progress of the triangulation

    // This stores the start of an edge (as a pseudoangle) as an index into
    // the edges array
    hull: BTreeMap<OrderedFloat<f64>, EdgeIndex>,
    half: Half,
}

impl<'a> Triangulation<'a> {
    pub fn new(points: &'a [Point]) -> Triangulation<'a> {
        let mut out = Triangulation {
            points,
            ..Default::default()
        };

        let seed = out.seed_triangle();
        out.center = circumcenter(points[seed.0], points[seed.1], points[seed.2]);
        let mut order: Vec<_> = (0..points.len())
            .filter(|&i| i != seed.0 && i != seed.1 && i != seed.2)
            .map(PointIndex)
            .collect();
        order.sort_by_key(
            |&p| OrderedFloat(distance2(out.center, out.points[p.0])));
        out.order = order;

        let pa = PointIndex(seed.0);
        let pb = PointIndex(seed.1);
        let pc = PointIndex(seed.2);

        let e_ab = out.half.insert(pa, pb, pc, None, None, None);
        out.hull.insert(out.key(pa), e_ab);
        out.hull.insert(out.key(pb), out.half.next(e_ab));
        out.hull.insert(out.key(pc), out.half.prev(e_ab));

        out
    }

    fn key(&self, p: PointIndex) -> OrderedFloat<f64> {
        let p = self.points[p.0];
        OrderedFloat(pseudo_angle((p.0 - self.center.0, p.1 - self.center.1)))
    }

    pub fn step(&mut self) -> bool {
        if self.next == self.order.len() {
            return false;
        }

        // Pick the next point in our pre-sorted array
        let p = self.order[self.next];
        self.next += 1;

        // Find the hull edge which will be split by this point
        let (phi, e) = self.get_hull_edge(p);
        let e = e.unwrap();

        /*
         *              p [new point]
         *             / ^
         *            /   \
         *           V  f  \
         *          --------> [new edge]
         *          b<------a [previous hull edge]
         *              e
         */

        let edge = self.half.edge(e);
        let a = edge.src;
        let b = edge.dst;

        // Sanity-check that p is on the correct side of b->a
        let o = orient2d(self.points[b.0], self.points[a.0], self.points[p.0]);
        if o == 0.0 {
            // TODO: implement this
            panic!("On-line case not yet handled");
        } else if o < 0.0 {
            panic!("Tried to place a point inside the hull");
        }

        let f = self.half.insert(b, a, p, None, None, Some(e));
        let edge_mut = self.half.edge_mut(e);
        assert!(edge_mut.buddy.is_none());
        edge_mut.buddy = Some(f);

        // Replaces the previous item in the hull
        self.hull.insert(self.key(a), self.half.next(f));
        self.hull.insert(phi, self.half.prev(f));

        self.next += 1;
        true
    }

    // Calculates a seed triangle from the given set of points
    // TODO: make robust to < 3 points and colinear inputs
    fn seed_triangle(&self) -> (usize, usize, usize) {
        let (x_bounds, y_bounds) = self.bbox();
        let center = ((x_bounds.0 + x_bounds.1) / 2.0,
                      (y_bounds.0 + y_bounds.1) / 2.0);

        // Pick the initial triangle, with
        //  a) the point closest to the center
        //  b) the point closest to a
        //  c) the point with the minimum circumradius
        let a = self.points.iter()
            .position_min_by_key(
                |q| OrderedFloat(distance2(center, **q)))
            .expect("Could not get initial point");
        let b = self.points.iter().enumerate()
            .position_min_by_key(
                |(j, p)| OrderedFloat(if *j == a {
                    std::f64::INFINITY
                } else {
                    distance2(self.points[a], **p)
                }))
            .expect("Could not get second point");
        let c = self.points.iter().enumerate()
            .position_min_by_key(
                |(j, p)| OrderedFloat(if *j == a || *j == b {
                    std::f64::INFINITY
                } else {
                    circumradius2(self.points[a], self.points[b], **p)
                }))
            .expect("Could not get third point");

        if orient2d(self.points[a], self.points[b], self.points[c]) > 0.0 {
            (a, b, c)
        } else {
            (a, c, b)
        }
    }

    /// Returns the edge of the bounding hull which the given point projects
    /// onto, as an index into self.edges.
    fn get_hull_edge(&self, p: PointIndex) -> (OrderedFloat<f64>, Option<EdgeIndex>) {
        use std::ops::Bound::*;
        let k = self.key(p);
        let mut r = self.hull.range((Unbounded, Included(k)));
        (k, r.next_back().map(|p| *p.1))
    }

    /// Calculates a bounding box, returning ((xmin, xmax), (ymin, ymax))
    pub fn bbox(&self) -> ((f64, f64), (f64, f64)) {
        let x = self.points.iter().map(|p| p.0).minmax().into_option().unwrap();
        let y = self.points.iter().map(|p| p.1).minmax().into_option().unwrap();
        return (x, y);
    }

    pub fn to_svg(&self) -> String {
        const SCALE: f64 = 100.0;
        let (x_bounds, y_bounds) = self.bbox();
        let line_width = (x_bounds.1 - x_bounds.0).max(y_bounds.1 - y_bounds.0) / 40.0 * SCALE;
        let dx = |x| { SCALE * (x - x_bounds.0) + line_width};
        let dy = |y| { SCALE * (y_bounds.1 - y) + line_width};

         let mut out = String::new();
         // Put a dummy rectangle in the SVG so that rsvg-convert doesn't clip
         out.push_str(&format!(
            r#"<svg viewbox="auto" xmlns="http://www.w3.org/2000/svg">
    <rect x="0" y="0" width="{}" height="{}"
     style="fill:none" />"#,
     dx(x_bounds.1) + line_width,
     dy(y_bounds.0) + line_width));

         // Push every edge into the SVG
         for (pa, pb) in self.half.iter_edges() {
             out.push_str(&format!(
                r#"
    <line x1="{}" y1="{}" x2="{}" y2="{}"
     style="stroke:rgb(255,0,0)"
     stroke-width="{}"
     stroke-linecap="round" />"#,
                dx(self.points[pa.0].0),
                dy(self.points[pa.0].1),
                dx(self.points[pb.0].0),
                dy(self.points[pb.0].1),
                line_width))
         }

         // Add a circle at the origin
         out.push_str(&format!(
            r#"
    <circle cx="{}" cy="{}" r="{}" style="fill:rgb(0,255,0)" />"#,
            dx(0.0), dy(0.0), line_width));
         out.push_str("\n</svg>");
         out
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
        println!("{}", t.to_svg());
    }
}
