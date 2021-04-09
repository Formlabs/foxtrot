use itertools::Itertools;

use crate::predicates::{acute, centroid, distance2, orient2d, in_circle};
use crate::{Point, PointIndex, PointVec, EdgeIndex};
use crate::{half, half::Half, hull::Hull, util::min4};
pub struct Triangulation {
    center: Point,
    points: PointVec<Point>,    // Sorted in the constructor
    remap: PointVec<usize>,     // self.points[i] = input[self.remap[i]]
    next: PointIndex,           // Progress of the triangulation

    // If a point p terminates fixed edges, then endings[p] will be a tuple
    // range into ending_data containing the starting points of those edges.
    endings: PointVec<(usize, usize)>,
    ending_data: Vec<PointIndex>,

    // This stores the start of an edge (as a pseudoangle) as an index into
    // the edges array
    hull: Hull,
    half: Half,
}

impl Triangulation {
    pub fn new_with_edges<'a, E>(points: &[Point], edges: E) -> Triangulation
        where E: IntoIterator<Item=&'a (usize, usize)> + Copy + Clone
    {
        //  Picking the seed triangle and center point is tricky!
        //
        //  We want a center which is contained within the seed triangle,
        //  and with the property that the seed triangle is the closest
        //  three points when sorted by distance to the center.
        //
        //  The paper suggests using the center of the bounding box, but in
        //  that case, you can end up with cases where the center is _outside_
        //  of the initial seed triangle, which is awkward.
        //
        //  delaunator and its ports instead pick the circumcenter of a
        //  triangle near the bbox center, which has the same issue.
        //
        //  Picking the centroid of the seed triangle instead of the
        //  circumcenter can also lead to issues, as another point could be
        //  closer, which will violate the condition that points are always
        //  outside the hull when they are added to the triangulation.
        //
        //  We iterate, repeatedly picking a center and checking to see if the
        //  conditions hold; otherwise, we pick a new center and try again.

        // Start by picking a center which is at the center of the bbox
        let (x_bounds, y_bounds) = Self::bbox(points);
        let mut center = ((x_bounds.0 + x_bounds.1) / 2.0,
                          (y_bounds.0 + y_bounds.1) / 2.0);

        // The scratch buffer contains our points, their indexes, and a distance
        // relative to the current center.
        let mut scratch = Vec::with_capacity(points.len());
        scratch.extend(points.iter()
            .enumerate()
            .map(|(j, p)| (j, distance2(center, *p))));

        let mut sorted_points = PointVec::with_capacity(points.len());

        // usize in original array -> PointIndex in sorted array
        let mut map_forward = vec![PointIndex::new(0); points.len()];

        // PointIndex in sorted array -> usize in original array
        let mut map_reverse = PointVec::with_capacity(points.len());

        for _ in 0..100 {
            let arr = min4(&scratch);

            // Pick out the triangle points, ensuring that they're clockwise
            let pa = arr[0];
            let mut pb = arr[1];
            let mut pc = arr[2];
            if orient2d(points[pa], points[pb], points[pc]) < 0.0 {
                std::mem::swap(&mut pb, &mut pc);
            }

            // If the center is contained within the triangle formed by the
            // three closest points, then we're clear to sort the list and
            // return it.
            if orient2d(points[pa], points[pb], center) > 0.0 &&
               orient2d(points[pb], points[pc], center) > 0.0 &&
               orient2d(points[pc], points[pa], center) > 0.0
            {
                // Sort with a special comparison function that puts the first
                // three keys at the start of the list, and uses OrderedFloat
                // otherwise.  The order of the first three keys is not
                // guaranteed, which we fix up below.
                scratch.sort_unstable_by(|k, r|
                    if k.0 == pa || k.0 == pb || k.0 == pc {
                        std::cmp::Ordering::Less
                    } else {
                        k.1.partial_cmp(&r.1).unwrap()
                    });

                // Apply sorting to initial three points, ignoring distance
                // values at this point because they're unused.
                scratch[0].0 = pa;
                scratch[1].0 = pb;
                scratch[2].0 = pc;

                for p in scratch.into_iter() {
                    sorted_points.push(points[p.0]);
                    map_forward[p.0] = map_reverse.push(p.0);
                }
                break;
            } else {
                // Pick a new centroid, then retry
                center = centroid(points[pa], points[pb], points[pc]);

                // Re-calculate distances in the scratch buffer
                scratch.iter_mut()
                    .for_each(|p| p.1 = distance2(center, points[p.0]));
            }
        }

        if sorted_points.is_empty() {
            panic!("Could not find seed triangle");
        }

        ////////////////////////////////////////////////////////////////////////
        let mut out = Triangulation {
            hull: Hull::new(center, &sorted_points), // borrowed here
            half: Half::new(points.len() * 2 - 5),

            center,
            points: sorted_points, // moved out here
            remap: map_reverse,
            next: PointIndex::new(3), // we've already built a, b, c

            // No points are endings right now
            endings: PointVec{ vec: vec![(0,0); points.len()] },
            ending_data: vec![],
        };

        let pa = PointIndex::new(0);
        let pb = PointIndex::new(1);
        let pc = PointIndex::new(2);
        let e_ab = out.half.insert(pa, pb, pc,
                                   half::EMPTY, half::EMPTY, half::EMPTY);
        let e_bc = out.half.next(e_ab);
        let e_ca = out.half.prev(e_ab);

        out.hull.insert_first(pa, e_ab);
        out.hull.insert(pb, e_bc);
        out.hull.insert(pc, e_ca);

        ////////////////////////////////////////////////////////////////////////
        // Iterate over edges, counting which points have a termination
        let mut termination_count = PointVec { vec: vec![0; points.len()] };
        let edge_iter = || edges.clone()
            .into_iter()
            .map(|&(src, dst)| {
                let src = map_forward[src];
                let dst = map_forward[dst];
                if src > dst { (dst, src) } else { (src, dst) }
            });
        for (src, dst) in edge_iter() {
            // Lock any edges that appear in the seed triangle.  Because the
            // (src, dst) tuple is sorted, there are only three possible
            // matches here.
            if (src, dst) == (pa, pb) {
                out.half.lock(e_ab);
            } else if (src, dst) == (pa, pc) {
                out.half.lock(e_ca);
            } else if (src, dst) == (pb, pc) {
                out.half.lock(e_bc);
            }
            termination_count[dst] += 1;
        }
        // Ending data will be tightly packed into the ending_data array; each
        // point stores its range into that array in self.endings[pt].  If the
        // point has no endings, then the range is (n,n) for some value n.
        let mut cumsum = 0;
        for (dst, t) in termination_count.iter().enumerate() {
            out.endings[PointIndex::new(dst)] = (cumsum, cumsum);
            cumsum += t;
        }
        out.ending_data.resize(cumsum, PointIndex::new(0));
        for (src, dst) in edge_iter() {
            let t = &mut out.endings[dst].1;
            out.ending_data[*t] = src;
            *t += 1;
        }

        // ...and we're done!
        out
    }

    pub fn new(points: & [Point]) -> Triangulation {
        let edges: [(usize, usize); 0] = [];
        return Self::new_with_edges(points, &edges);
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn step(&mut self) -> bool {
        if self.next == self.points.len() {
            return false;
        }

        // Pick the next point in our pre-sorted array
        let p = self.next;
        self.next += 1usize;

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
        let o = orient2d(self.points[b], self.points[a], self.points[p]);
        assert!(o != 0.0);
        assert!(o > 0.0);

        let f = self.half.insert(b, a, p, half::EMPTY, half::EMPTY, e_ab);

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
            if acute(self.points[p], self.points[b], self.points[q]) <= 0.0 ||
               orient2d(self.points[p], self.points[b], self.points[q]) >= 0.0
            {
                break;
            }

            // Friendship ended with p->b->q
            self.hull.erase(b);

            // Now p->q is my new friend
            let edge_pq = self.half.insert(p, q, b, e_bq, e_pb, half::EMPTY);
            self.hull.update(p, edge_pq);
            b = q;

            // Then legalize from the two new triangle edges (bp and qb)
            self.legalize(self.half.next(edge_pq));
            self.legalize(self.half.prev(edge_pq));
        }

        /*  Then ,do the same thing in the other direction
         *         p        q
         *        / ^      / \
         *       /   \    /   \
         *      V  f  \  v     \
         *     b------->a       \
         *     .<-------
         *          e
         */
        let mut a = a;
        loop {
            let e_ap = self.hull.edge(a);
            let e_qa = self.hull.prev_edge(a);
            let q = self.half.edge(e_qa).src;
            if acute(self.points[p], self.points[a], self.points[q]) <= 0.0 ||
               orient2d(self.points[p], self.points[a], self.points[q]) <= 0.0
            {
                break;
            }

            self.hull.erase(a);
            let edge_qp = self.half.insert(q, p, a, e_ap, e_qa, half::EMPTY);
            self.hull.update(q, edge_qp);
            a = q;

            // Then legalize from the two new triangle edges (bp and qb)
            self.legalize(self.half.next(edge_qp));
            self.legalize(self.half.prev(edge_qp));
        }

        // Next, we check whether this point terminates any edges that are
        // locked in the triangulation (the "constrainted" part of Constrained
        // Delaunay Triangulation).
        let (start, end) = self.endings[p];
        for i in start..end {
            self.handle_fixed_edge(p, self.ending_data[i], f);
        }

        true
    }

    fn handle_fixed_edge(&mut self, p: PointIndex, src: PointIndex, e_ba: EdgeIndex) {
        /* We've just built a triangle that contains a fixed edge.  For example,
         * it could be here:
         *            p
         *          / :^
         *         / :  \
         *        /  :   \
         *       /  :     \
         *      V   : e_ba \
         *     b---:------->a
         *         :
         *        src
         *
         * Or outside the hull!
         * Or a mix of both (ugh)
         */

        // First, handle the easy cases, if the source of the fixed edge happens
        // to line up with one of the points on the newly-added triangle, then
        // we fix that edge and count our blessings.
        let edge = self.half.edge(e_ba);
        if src == edge.src {
            return self.half.lock(edge.prev);
        } else if src == edge.dst {
            return self.half.lock(edge.next);
        }

        /* Now, the fun begins!
         *
         * We'll be walking along the line from p to src, which will either be
         *  - Inside the triangulation
         *  - Outside the triangulation, to the left of the hull
         *  - Outside the triangulation, to the right of the hull
         *
         *  (left/right using the picture above; in practice, it's a circle,
         *  so we're using clockwise/counter-clockwise predicates)
         * on the left and right side.  Those edges will either be edges
         */
        let mut left_contour:  Vec<EdgeIndex> = Vec::new();
        let mut right_contour: Vec<EdgeIndex> = Vec::new();
        enum Mode { Left, Right, Inside };

        let b = edge.src;
        let a = edge.dst;

        // Figure out whether the p-src line falls within the p-b-a triangle,
        // or to which side it lies.  This determines what mode we use for
        // walking (although the mode will change if we enter or exit the
        // triangulation later on)
        let o_left = orient2d(self.points[p], self.points[b], self.points[src]);
        let o_right = orient2d(self.points[p], self.points[src], self.points[a]);
        assert!(o_left != 0.0);
        assert!(o_right != 0.0); // Not handled yet
        let (mut mode, mut e) = if o_left > 0.0 && o_right > 0.0 {
            assert!(edge.buddy != half::EMPTY);
            (Mode::Inside, edge.buddy)
        } else if o_left < 0.0 {
            (Mode::Left, edge.prev)
        } else if o_right < 0.0 {
            (Mode::Right, edge.next)
        } else {
            panic!("Invalid hull winding");
        };

        loop {
            match mode {
                /*
                 *               / p
                 *             / /  ^
                 *           /  /    \
                 *         /   /e     \
                 *        /   /        \
                 *      /    V          \
                 *    /     ------------>\
                 *  src
                 *
                 *  (as the loop runs, will may not start at point p, but it
                 *  will be the most recent hull edge)
                 */
                Mode::Left => {
                    // Check the next hull edge to see if it either intersects
                    // the new line or terminates it.
                    let next_index = self.hull.next_edge(self.half.edge(e).dst);
                    let next_edge = self.half.edge(next_index);

                    // If we've reached the target point, then rejoice!
                    if next_edge.dst == src {
                        break;
                    }
                    let o = orient2d(self.points[p], self.points[src],
                                     self.points[next_edge.dst]);
                    // If we're still outside the triangulation, then update
                    // the edge and continue
                    if o > 0.0 {
                        e = next_index;
                    } else if o < 0.0 {
                        // Otherwise, we've crossed over and now need to run
                        // in the inside-the-triangulation mode.
                        e = next_index;
                        mode = Mode::Inside;
                    } else {
                        // Lightly brushing by the point on the hull
                        assert!(false);
                    }
                }

                /*
                 *            p\
                 *          /  ^ \
                 *         /    \  \
                 *        /      \   \
                 *       /       e\    \
                 *      V          \     \
                 *     ------------>\      \
                 *                           \
                 *                            src
                 */
                Mode::Right => {
                    let next_index = self.hull.prev_edge(self.half.edge(e).src);
                    let next_edge = self.half.edge(next_index);

                    if next_edge.src == src {
                        break; // rejoice!
                    }
                    let o = orient2d(self.points[p], self.points[next_edge.src],
                                     self.points[src]);
                    if o > 0.0 {
                        e = next_index;
                    } else if o < 0.0 {
                        e = next_index;
                        mode = Mode::Inside;
                    } else {
                        // TODO
                        assert!(false); // stabbing the hull point
                    }
                }

                /*            p
                 *           :
                 *          :
                 *     b<--:-------a
                 *      \ :    e   ^
                 *       :\      /
                 *      :   v  /
                 *     :     c
                 *    src
                 */
                Mode::Inside => {
                    // There has been a p->src intersection with edge e,
                    // and we're now within a particular triangle.  We
                    // check which edge we exit along, and whether we exit
                    // into another triangle or back onto the hull
                    let edge_ab = self.half.edge(e);
                    let a = edge.src;
                    let b = edge.dst;
                    let c = self.half.edge(edge.next).dst;

                    if c == src {
                        break; // rejoice!
                    }

                    let o_psc = orient2d(self.points[p], self.points[src], self.points[c]);
                    if o_psc > 0.0 {
                        // Exiting the b->c edge
                        let e_bc = edge.next;
                        let buddy = self.half.edge(e_bc).buddy;
                        if buddy != half::EMPTY {
                            e = buddy;
                        } else {
                            e = e_bc;
                            mode = Mode::Left;
                        }
                    } else if o_psc < 0.0 {
                        // Exiting the c->a edge
                        let e_ca = edge.prev;
                        let buddy = self.half.edge(e_ca).buddy;
                        if buddy != half::EMPTY {
                            e = buddy;
                        } else {
                            e = e_ca;
                            mode = Mode::Right;
                        }
                    } else {
                        assert!(false); // stabbing c in the heart
                    }
                }
            }
        }
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
         *
         *  This function may be called with a half-empty edge, e.g. while
         *  recursing; in that case, then return immediately.
         */
        let edge = self.half.edge(e_ab);
        if edge.buddy == half::EMPTY {
            return;
        }
        let a = edge.src;
        let b = edge.dst;
        let c = self.half.edge(self.half.next(e_ab)).dst;

        let e_ba = edge.buddy;
        let e_ad = self.half.next(e_ba);
        let d = self.half.edge(e_ad).dst;

        if in_circle(self.points[a], self.points[b], self.points[c],
                     self.points[d]) > 0.0
        {
            let e_db = self.half.prev(e_ba);

            self.half.swap(e_ab);
            self.legalize(e_ad);
            self.legalize(e_db);
        }
    }

    /// Calculates a bounding box, returning ((xmin, xmax), (ymin, ymax))
    pub fn bbox(points: &[Point]) -> ((f64, f64), (f64, f64)) {
        let x = points.iter().map(|p| p.0).minmax().into_option().unwrap();
        let y = points.iter().map(|p| p.1).minmax().into_option().unwrap();
        (x, y)
    }

    pub fn triangles(&self) -> impl Iterator<Item=(usize, usize, usize)> + '_ {
        self.half.iter_triangles()
            .map(move |(a, b, c)|
                (self.remap[a], self.remap[b], self.remap[c]))
    }

    pub fn to_svg(&self) -> String {
        const SCALE: f64 = 100.0;
        let (x_bounds, y_bounds) = Self::bbox(&self.points);
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
         for (pa, pb, fixed) in self.half.iter_edges() {
             out.push_str(&format!(
                r#"
    <line x1="{}" y1="{}" x2="{}" y2="{}"
     style="{}"
     stroke-width="{}"
     stroke-linecap="round" />"#,
                dx(self.points[pa].0),
                dy(self.points[pa].1),
                dx(self.points[pb].0),
                dy(self.points[pb].1),
                if fixed { "stroke:rgb(255,255,255)" }
                    else { "stroke:rgb(255,0,0)" },
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
                dx(self.points[pa].0),
                dy(self.points[pa].1),
                dx(self.points[pb].0),
                dy(self.points[pb].1),
                line_width, line_width * 2.0))
         }

         for p in &self.points {
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
