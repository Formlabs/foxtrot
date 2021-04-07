use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::predicates::{acute, centroid, distance2, orient2d, in_circle};
use crate::{Point, PointIndex, EdgeIndex};
use crate::{half::Half, hull::Hull};

#[derive(Default)]
pub struct Triangulation<'a> {
    points: &'a[Point],
    center: Point,
    order: Vec<PointIndex>,  // Ordering of the points, from inner to outer
    next: usize,        // Progress of the triangulation

    // This stores the start of an edge (as a pseudoangle) as an index into
    // the edges array
    hull: Hull,
    half: Half,
}

impl<'a> Triangulation<'a> {
    pub fn new(points: &'a [Point]) -> Triangulation<'a> {
        let mut out = Triangulation {
            points,
            half: Half::new(points.len() * 2 - 5),
            ..Default::default()
        };

        // seed_triangle() writes out.order and out.center
        let (pa, pb, pc) = out.seed_triangle();

        out.hull = Hull::new(out.center, points);

        let e_ab = out.half.insert(pa, pb, pc, None, None, None);
        let e_bc = out.half.next(e_ab);
        let e_ca = out.half.prev(e_ab);

        out.hull.insert_first(pa, e_ab);
        out.hull.insert(pb, e_bc);
        out.hull.insert(pc, e_ca);

        out.next = 0;
        out
    }

    fn point(&self, p: PointIndex) -> Point {
        self.points[p.0]
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
        let e_ab = self.hull.get_edge(p);

        /*
         *              p [new point]
         *             / ^
         *            /   \
         *           V  f  \
         *          --------> [new edge]
         *          b<------a [previous hull edge]
         *              e
         */
        let edge = self.half.edge(e_ab);
        let a = edge.src;
        let b = edge.dst;

        // Sanity-check that p is on the correct side of b->a
        let o = orient2d(self.point(b), self.point(a), self.point(p));
        assert!(o != 0.0);
        assert!(o > 0.0);

        let f = self.half.insert(b, a, p, None, None, Some(e_ab));

        // Replaces the previous item in the hull
        self.hull.update(a, self.half.next(f));

        // Insert the new edge into the hull
        self.hull.insert(p, self.half.prev(f));

        self.legalize(f);

        /* Now, we search for sharp angles on each side.  The point q
         * should be the next point along the edge from e
         *
         *      q       p [new point]
         *     | ^     / ^
         *     |  \   /   \
         *     |   \ V  f  \
         *     V    b-------> [new edge]
         *     ---->.<------a [previous hull edge]
         *              e
         */
        let mut b = b;
        loop { // Walking CCW around the hull
            let e_pb = self.hull.edge(p);
            let e_bq = self.hull.edge(b);
            let q = self.half.edge(e_bq).dst;

            // Check that the inner angle is less that pi/2, and that the
            // inner triangle is correctly wound; if either is not the case,
            // then break immediately.
            if acute(self.point(p), self.point(b), self.point(q)) <= 0.0 ||
               orient2d(self.point(p), self.point(b), self.point(q)) >= 0.0
            {
                break;
            }

            // Friendship ended with p->b->q
            self.hull.erase(b);

            // Now p->q is my new friend
            let edge_pq = self.half.insert(q, b, p, Some(e_pb), None, Some(e_bq));
            self.hull.update(p, edge_pq);
            b = q;

            // Then legalize from the two new triangle edges (bp and qb)
            // TODO
        }

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
    //  and with the property that the seed triangle is the closest
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
    //  conditions hold; otherwise, we pick a new center and try again.
    fn seed_triangle(&mut self) -> (PointIndex, PointIndex, PointIndex) {
        // Start by picking a center which is at the center of the bbox
        let (x_bounds, y_bounds) = self.bbox();
        self.center = ((x_bounds.0 + x_bounds.1) / 2.0,
                       (y_bounds.0 + y_bounds.1) / 2.0);

        // The scratch buffer contains our points, their indexes, and a distance
        // relative to the current center.
        let mut scratch = Vec::with_capacity(self.points.len());
        scratch.extend(self.points.iter()
            .enumerate()
            .map(|(j, p)| (PointIndex(j), distance2(self.center, *p))));

        // Finds the four points in the given buffer that are closest to the
        // center, returning them in order (so that out[0] is closest).
        //
        // This is faster than sorting the entire array each time to check
        // the four closest distances to a given point.
        let min4 = |buf: &[(PointIndex, f64)]| -> [PointIndex; 4] {
            let mut array = [(PointIndex(0), std::f64::INFINITY); 4];
            for &(p, score) in buf.iter() {
                if score >= array[3].1 {
                    continue;
                }
                for i in 0..4 {
                    // If the new score is bumping this item out of the array,
                    // then shift all later items over by one and return.
                    if score <= array[i].1 {
                        for j in (i..3).rev() {
                            array[j + 1] = array[j];
                        }
                        array[i] = (p, score);
                        break;
                    }
                }
            }

            let mut out = [PointIndex(0); 4];
            for (i, a) in array.iter().enumerate() {
                out[i] = a.0;
            }
            out
        };

        for _ in 0..100 {
            let arr = min4(&scratch);

            // Pick out the triangle points, ensuring that they're clockwise
            let pa = arr[0];
            let mut pb = arr[1];
            let mut pc = arr[2];
            if orient2d(self.point(pa), self.point(pb), self.point(pc)) < 0.0 {
                std::mem::swap(&mut pb, &mut pc);
            }

            // If the center is contained within the triangle formed by the
            // three closest points, then we're clear to sort the list and
            // return it.
            if orient2d(self.point(pa), self.point(pb), self.center) > 0.0 &&
               orient2d(self.point(pb), self.point(pc), self.center) > 0.0 &&
               orient2d(self.point(pc), self.point(pa), self.center) > 0.0
            {
                // Sort with a special comparison function that puts the first
                // three keys at the start of the list, and uses OrderedFloat
                // otherwise.
                scratch.sort_unstable_by(|k, r|
                    if k.0 == pa || k.0 == pb || k.0 == pc {
                        std::cmp::Ordering::Less
                    } else {
                        OrderedFloat(k.1).cmp(&OrderedFloat(r.1))
                    });

                // reserve + extend is faster than collect, experimentally
                self.order.reserve(scratch.len() - 3);
                self.order.extend(scratch.into_iter()
                    .skip(3) // Skip [pa, pb, pc], which will be at the start
                    .map(|p| p.0));
                return (pa, pb, pc);
            } else {
                // Pick a new centroid, then retry
                self.center = centroid(
                    self.point(pa), self.point(pb), self.point(pc));

                // Re-calculate distances in the scratch buffer
                scratch.iter_mut()
                    .for_each(|p| p.1 = distance2(self.center, self.point(p.0)));
            }
        }
        panic!("Could not find seed triangle");
    }

    /// Calculates a bounding box, returning ((xmin, xmax), (ymin, ymax))
    pub fn bbox(&self) -> ((f64, f64), (f64, f64)) {
        let x = self.points.iter().map(|p| p.0).minmax().into_option().unwrap();
        let y = self.points.iter().map(|p| p.1).minmax().into_option().unwrap();
        (x, y)
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
             let edge = self.half.edge(e);
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

         for p in self.points {
             out.push_str(&format!(
                r#"
        <circle cx="{}" cy="{}" r="{}" style="fill:rgb(255,128,128)" />"#,
                dx(p.0), dy(p.1), line_width));
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
