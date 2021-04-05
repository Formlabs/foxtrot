use std::collections::BTreeMap;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::predicates::{centroid, distance2, orient2d, in_circle, pseudo_angle};
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

        // seed_triangle() writes out.order and out.center
        let (pa, pb, pc) = out.seed_triangle();

        let e_ab = out.half.insert(pa, pb, pc, None, None, None);
        out.hull.insert(out.key(pa), e_ab);
        out.hull.insert(out.key(pb), out.half.next(e_ab));
        out.hull.insert(out.key(pc), out.half.prev(e_ab));

        out.next = 3;
        out
    }

    fn point(&self, p: PointIndex) -> Point {
        self.points[p.0]
    }

    fn key(&self, p: PointIndex) -> OrderedFloat<f64> {
        let p = self.point(p);
        OrderedFloat(pseudo_angle((p.0 - self.center.0, p.1 - self.center.1)))
    }

    pub fn run(&mut self) {
        while self.step() {}
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
        let o = orient2d(self.point(b), self.point(a), self.point(p));
        assert!(o != 0.0);
        assert!(o > 0.0);

        let f = self.half.insert(b, a, p, None, None, Some(e));
        let edge_mut = self.half.edge_mut(e);
        assert!(edge_mut.buddy.is_none());
        edge_mut.buddy = Some(f);

        // Replaces the previous item in the hull
        self.hull.insert(self.key(a), self.half.next(f));
        self.hull.insert(phi, self.half.prev(f));

        self.legalize(f);
        true
    }

    fn legalize(&mut self, e_ab: EdgeIndex) {
        /* We're given this
         *            c
         *          /  ^
         *         /    \
         *        /      \
         *       /        \
         *      V     e    \
         *     a----------->\
         *     \<-----------b
         *      \    f     ^
         *       \        /
         *        \      /
         *         \    /
         *          V  /
         *           d
         *  We check whether d is within the circumcircle of abc.
         *  If so, then we flip the edge and recurse based on the triangles
         *  across from edges ad and db.
         */
        let edge = self.half.edge(e_ab);
        let a = edge.src;
        let b = edge.dst;
        let c = self.half.edge(self.half.next(e_ab)).dst;

        if edge.buddy.is_none() {
            return;
        }
        let e_ba = edge.buddy.unwrap();
        let e_ad = self.half.next(e_ba);
        let d = self.half.edge(e_ad).dst;

        if in_circle(self.point(a), self.point(b), self.point(c),
                     self.point(d)) > 0.0
        {
            let e_db = self.half.prev(e_ba);

            self.half.swap(e_ab).expect("Swap failed");
            self.legalize(e_ad);
            self.legalize(e_db);
        }
    }

    //  Picking the seed triangle and center point is tricky!
    //
    //  We want a center which is contained within the seed triangle,
    //  and with the property that the seed triangle is the initial
    //  three points when sorted by distance to the center.
    //
    //  The paper suggests using the center of the bounding box, but in that
    //  case, you can end up with cases where the center is _outside_ of the
    //  initial seed triangle, which is awkward.
    //
    //  delaunator and its ports instead pick the circumcenter of a triangle
    //  near the bbox center, which has the same issue.
    //
    //  Picking the centroid of the seed triangle instead of the circumcenter
    //  can also lead to issues, as another point could be closer, which
    //  will violate the condition that points are always outside the hull.
    //
    //  We iterate, repeatedly picking a center and checking to see if the
    //  conditions hold; otherwise, we resort and try again.
    fn seed_triangle(&mut self) -> (PointIndex, PointIndex, PointIndex) {
        let (x_bounds, y_bounds) = self.bbox();

        // These are kept as as separate variables to satisfy the borrow
        // checker; otherwise, sorting order while looking at point(...)
        // becomes tricky.  We store then before returning.
        let mut center = ((x_bounds.0 + x_bounds.1) / 2.0,
                          (y_bounds.0 + y_bounds.1) / 2.0);
        let mut order: Vec<_> = (0..self.points.len())
            .map(PointIndex)
            .collect();

        for _ in 0..100 {
            order.sort_by_key(
                |&p| OrderedFloat(distance2(center, self.point(p))));

            let pa = order[0];
            let mut pb = order[1];
            let mut pc = order[2];

            if orient2d(self.point(pa), self.point(pb), self.point(pc)) < 0.0 {
                std::mem::swap(&mut pb, &mut pc);
            }
            if orient2d(self.point(pa), self.point(pb), center) > 0.0 &&
               orient2d(self.point(pb), self.point(pc), center) > 0.0 &&
               orient2d(self.point(pc), self.point(pa), center) > 0.0
            {
                // Swap in these values (which should be cheap)
                std::mem::swap(&mut self.center, &mut center);
                std::mem::swap(&mut self.order, &mut order);
                return (pa, pb, pc);
            } else {
                center = centroid(self.point(pa), self.point(pb), self.point(pc));
                eprintln!("Got center {:?}", center);
            }
        }
        panic!("Could not find seed triangle");
    }

    /// Returns the edge of the bounding hull which the given point projects
    /// onto, as an index into self.edges.
    fn get_hull_edge(&self, p: PointIndex) -> (OrderedFloat<f64>, EdgeIndex) {
        use std::ops::Bound::*;
        let k = self.key(p);

        // If we don't find an item in the target range, then it must be below
        // every other value in the tree, so we return the last item in the
        // BTreeMap (which is the edge that wraps around from 1 -> 0)
        let mut r = self.hull.range((Unbounded, Included(k)));
        let next = r.next_back();
        match next {
            Some(e) => (k, *e.1),
            None => (k, self.hull.iter().next_back().map(|p| *p.1).unwrap()),
        }
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
        let line_width = (x_bounds.1 - x_bounds.0).max(y_bounds.1 - y_bounds.0) / 80.0 * SCALE;
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
                dx(self.point(pa).0),
                dy(self.point(pa).1),
                dx(self.point(pb).0),
                dy(self.point(pb).1),
                line_width))
         }

         for e in self.hull.values() {
             let edge = self.half.edge(*e);
             let (pa, pb) = (edge.src, edge.dst);
             out.push_str(&format!(
                r#"
    <line x1="{}" y1="{}" x2="{}" y2="{}"
     style="stroke:rgb(255,255,0)"
     stroke-width="{}" stroke-dasharray="{}"
     stroke-linecap="round" />"#,
                dx(self.point(pa).0),
                dy(self.point(pa).1),
                dx(self.point(pb).0),
                dy(self.point(pb).1),
                line_width, line_width * 2.0))
         }

         // Add a circle at the origin
         out.push_str(&format!(
            r#"
    <circle cx="{}" cy="{}" r="{}" style="fill:rgb(0,255,0)" />"#,
            dx(self.center.0), dy(self.center.1), line_width));
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
        assert_eq!(t.order[0], PointIndex(3));
    }

    #[test]
    fn inline_pts() {
        let pts = vec![
            (0.0, 0.0), (1.0, 0.0), (0.0, 1.0),
            (0.0, 2.0), (2.0, 0.0), (1.0, 1.0), // <- this is the inline one
            (-2.0, -2.0), // Tweak bbox center to seed from first three points
        ];
        let mut t = Triangulation::new(&pts);
        while t.step() {}
        assert!(true);
    }
}
