use std::collections::BTreeMap;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::predicates::{circumcenter, circumradius2, distance2, orient2d, pseudo_angle};
use crate::{Point, PointIndex, EdgeIndex};
use crate::half::Half;

pub struct Triangulation<'a> {
    points: &'a[Point],
    center: Point,
    order: Vec<usize>, // Ordering of the points, from inner to outer

    // This stores the start of an edge (as a pseudoangle) as an index into
    // the edges array
    hull: BTreeMap<OrderedFloat<f64>, EdgeIndex>,
    half: Half,
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
        let order: Vec<usize> = pts.iter()
            .map(|p| p.0)
            .filter(|&i| i != seed.0 && i != seed.1 && i != seed.2)
            .collect();

        let mut out = Triangulation {
            points,
            order,
            center,

            half: Half::new(),
            hull: BTreeMap::new(),
        };

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

    pub fn to_svg(&self) -> String {
        let x_bounds = self.points.iter().map(|p| p.0).minmax().into_option().unwrap();
        let y_bounds = self.points.iter().map(|p| p.1).minmax().into_option().unwrap();
        let line_width = (x_bounds.1 - x_bounds.0).max(y_bounds.1 - y_bounds.0) / 40.0;
        let dx = |x| { x - x_bounds.0 + line_width};
        let dy = |y| { y_bounds.1 - y - line_width};

         let mut out = String::new();
         // Put a dummy rectangle in the SVG so that rsvg-convert doesn't clip
         out.push_str(&format!(
            r#"<svg viewbox="auto" xmlns="http://www.w3.org/2000/svg">
    <rect x="0" y="0" width="{}" height="{}"
     style="fill:none" />"#,
            dx(x_bounds.1) + line_width,
            dy(y_bounds.0) + 2.0 * line_width));

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
