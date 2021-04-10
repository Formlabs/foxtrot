use itertools::Itertools;

use crate::predicates::{acute, orient2d, in_circle};
use crate::{Point, PointIndex, PointVec, EdgeIndex};
use crate::{half, half::Half, sweepline::hull::Hull};

const TERMINAL_LEFT: PointIndex = PointIndex { val: 0 };
const TERMINAL_RIGHT: PointIndex = PointIndex { val: 1 };

pub struct Triangulation {
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
        let (x_bounds, y_bounds) = Self::bbox(points);

        // The scratch buffer contains point orders and their y coordinates
        let mut scratch = Vec::with_capacity(points.len());
        scratch.extend(points.iter()
            .enumerate()
            .map(|(j, p)| (j, p.1)));

        let mut sorted_points = PointVec::with_capacity(points.len());

        // usize in original array -> PointIndex in sorted array
        let mut map_forward = vec![PointIndex::new(0); points.len()];

        // PointIndex in sorted array -> usize in original array
        let mut map_reverse = PointVec::with_capacity(points.len());

        scratch.sort_unstable_by(|k, r| k.1.partial_cmp(&r.1).unwrap());

        // Add two phantom points to the point list, so that the hull is
        // always guaranteed to be below points in the original set.
        let dx = x_bounds.1 - x_bounds.0;
        let dy = y_bounds.1 - y_bounds.0;
        let x_bounds = (x_bounds.0 - dx / 8.0, x_bounds.1 + dx / 8.0);
        let y_lower = y_bounds.0 - dy / 8.0;
        sorted_points.push((x_bounds.0, y_lower));
        sorted_points.push((x_bounds.1, y_lower));
        map_reverse.push(0); // Dummy values
        map_reverse.push(0);

        // Then, copy the rest of the sorted points into sorted_points and
        // store the full maps.
        for p in scratch.into_iter() {
            sorted_points.push(points[p.0]);
            map_forward[p.0] = map_reverse.push(p.0);
        }

        ////////////////////////////////////////////////////////////////////////
        let mut out = Triangulation {
            hull: Hull::new(x_bounds.0, x_bounds.1, &sorted_points),
            half: Half::new(sorted_points.len() * 2 - 5),

            remap: map_reverse,
            next: PointIndex::new(3), // we've already built a, b, c

            // Endings are assigned later
            endings: PointVec{ vec: vec![(0,0); sorted_points.len() + 2] },
            ending_data: vec![],

            points: sorted_points, // moved out here
        };

        let pa = TERMINAL_LEFT;
        let pb = TERMINAL_RIGHT;
        let pc = PointIndex::new(2);
        let e_ab = out.half.insert(pa, pb, pc,
                                   half::EMPTY, half::EMPTY, half::EMPTY);
        let e_bc = out.half.next(e_ab);
        let e_ca = out.half.prev(e_ab);

        out.hull.insert_lower_edge(pa, pb);
        out.hull.update(pa, e_ca);
        out.hull.insert(pc, e_bc);

        ////////////////////////////////////////////////////////////////////////
        // Iterate over edges, counting which points have a termination
        let mut termination_count = PointVec { vec: vec![0; points.len() + 2] };
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

    fn orient2d(&self, pa: PointIndex, pb: PointIndex, pc: PointIndex) -> f64 {
        orient2d(self.points[pa], self.points[pb], self.points[pc])
    }

    fn acute(&self, pa: PointIndex, pb: PointIndex, pc: PointIndex) -> f64 {
        acute(self.points[pa], self.points[pb], self.points[pc])
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
        assert!(e_ab == self.hull.edge(b));

        // Sanity-check that p is on the correct side of b->a
        let o = self.orient2d(b, a, p);
        assert!(o != 0.0);
        assert!(o > 0.0);

        let f = self.half.insert(b, a, p, half::EMPTY, half::EMPTY, e_ab);

        // Replaces the previous item in the hull
        self.hull.update(b, self.half.prev(f));

        // Insert the new edge into the hull
        self.hull.insert(p, self.half.next(f));

        self.legalize(f);

        // Check and fill acute angles
        self.check_acute_left(p);
        self.check_acute_right(p);

        // Check and fill basins
        //self.check_basin_right(p);

        // Finally, we check whether this point terminates any edges that are
        // locked in the triangulation (the "constrainted" part of Constrained
        // Delaunay Triangulation).
        let (start, end) = self.endings[p];
        for i in start..end {
            self.handle_fixed_edge(p, self.ending_data[i]);
        }

        true
    }

    fn check_acute_left(&mut self, p: PointIndex) {
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
        let mut b = self.half.edge(self.hull.prev_edge(p)).dst;
        while b != TERMINAL_LEFT { // Walking left around the hull
            let e_pb = self.hull.edge(b);
            let e_bq = self.hull.prev_edge(b);
            let q = self.half.edge(e_bq).dst;

            // Check that the inner angle is less that pi/2, and that the
            // inner triangle is correctly wound; if either is not the case,
            // then break immediately.
            if self.acute(p, b, q) <= 0.0 {
                break;
            }
            assert!(self.orient2d(p, b, q) < 0.0);

            // Friendship ended with p->b->q
            self.hull.erase(b);

            // Now p->q is my new friend
            let edge_pq = self.half.insert(p, q, b, e_bq, e_pb, half::EMPTY);
            self.hull.update(q, edge_pq);
            b = q;

            // Then legalize from the two new triangle edges (bp and qb)
            self.legalize(self.half.next(edge_pq));
            self.legalize(self.half.prev(edge_pq));
        }
    }

    fn check_acute_right(&mut self, p: PointIndex) {
        /*  Rightward equivalent of check_acute_right
         *         p        q
         *        / ^      / \
         *       /   \    /   \
         *      V  f  \  v     \
         *     b------->a       \
         *     .<-------
         *          e
         */
        let mut a = self.half.edge(self.hull.edge(p)).src;
        while a != TERMINAL_RIGHT {
            let e_ap = self.hull.edge(p);
            let e_qa = self.hull.next_edge(p);
            let q = self.half.edge(e_qa).src;
            if self.acute(p, a, q) <= 0.0 {
                break;
            }
            assert!(self.orient2d(p, a, q) > 0.0);

            self.hull.erase(a);
            let edge_qp = self.half.insert(q, p, a, e_ap, e_qa, half::EMPTY);
            self.hull.update(p, edge_qp);
            a = q;

            // Then legalize from the two new triangle edges (bp and qb)
            self.legalize(self.half.next(edge_qp));
            self.legalize(self.half.prev(edge_qp));
        }
    }

    fn handle_fixed_edge(&mut self, p: PointIndex, src: PointIndex) {
        /*  We've just built a triangle that contains a fixed edge, and need
            to walk through the triangulation and implement that edge.

            The only thing we know going in is that point p is on the hull of
            the triangulation.

            We start by finding the triangle a->p->b which contains the edge
            p->src, e.g.

                       p
                     / :^
                    / :  \
                   /  :   \
                  /  :     \
                 V   :      \
                b---:------->a
                    :
                   src

            This triangle may not exist!  For example, if the p->src edge
            remains outside the hull, then we start in Mode::Left
        */
        let e_right = self.hull.edge(p);
        let e_left = self.hull.prev_edge(p);

        // Note that p-b-a is not necessarily a triangle at this point, it's
        // just a wedge, which could contain multiple triangles.  (For example,
        // this would be the case if there was an edge flip of b->a)
        let wedge_left = self.half.edge(e_left).dst;
        let wedge_right = self.half.edge(e_right).src;

        // Easy cases: the fixed edge is the same as the hull edge
        if src == wedge_left {
            return self.half.lock(e_left);
        } else if src == wedge_right {
            return self.half.lock(e_right);
        }

        let o_left = self.orient2d(p, wedge_left, src);
        let o_right = self.orient2d(p, src, wedge_right);

        // For now, we don't handle cases where fixed edges have coincident
        // points that are not the start/end of the fixed edge.
        assert!(o_left != 0.0);
        assert!(o_right != 0.0);

        /*  Now, the fun begins!

            We'll be walking along the line from p to src, which will either be
            - Inside the triangulation
            - Outside the triangulation, to the left of the hull
            - Outside the triangulation, to the right of the hull
         */
        #[derive(Copy, Clone, Debug, PartialEq)]
        enum Mode { Left, Right, Inside };

        let default_mode = if self.points[src].0 > self.points[p].0 {
            Mode::Right
        } else if self.points[src].0 < self.points[p].0 {
            Mode::Left
        } else {
            Mode::Inside
        };

        let (mut mode, mut current_edge_index) = if o_left < 0.0 {
            (Mode::Left, e_left)
        } else if o_right < 0.0 {
            (Mode::Right, e_right)
        } else {
            // Walk the inside of the wedge until we find the
            // subtriangle which captures the p-src line.
            let mut e_ap = self.half.edge(e_left).prev;

            loop {
                let edge_ap = self.half.edge(e_ap);
                let a = edge_ap.src;
                if a == src {
                    /* Lucky break: the src point is one of the edges directly
                       within the wedge, e.g.:
                               p
                             / ^\
                            /  | \
                           /   |  \
                          /    |   \
                         V     |    \
                        ------>a------ (a == src)
                    */
                    self.half.lock(e_ap);
                    return;
                }

                let intersected_index = edge_ap.prev;
                let intersected_edge = self.half.edge(intersected_index);

                let o = self.orient2d(p, src, a);
                assert!(o != 0.0);
                if o > 0.0 {
                    /*
                               p
                             /:^\
                            / :| \
                           /  :|  \
                          /  : |   \
                         V   : |    \
                        ----:->a------
                            :
                           src
                    */
                    break if intersected_edge.buddy == half::EMPTY {
                        assert!(default_mode != Mode::Inside);
                        (default_mode, intersected_index)
                    } else {
                        (Mode::Inside, intersected_edge.buddy)
                    };
                } else {
                    /*
                               p
                             / ^:\
                            /  |: \
                           /   | : \
                          /    | :  \
                         V     |  :  \
                        ------>a--:---
                                  :
                                  src
                    */
                    let buddy = edge_ap.buddy;

                    // We can't have walked out of the wedge, because otherwise
                    // o_right would be < 0.0 and we wouldn't be in this branch
                    assert!(buddy != half::EMPTY);
                    e_ap = self.half.edge(buddy).prev;
                }
            }
        };

        loop {
            match mode {
                /*
                                 / p
                               / /  ^
                             /  /    \
                           /   /e     \
                          /   /        \
                        /    V          \
                      /     ------------>\
                    src

                    (as the loop runs, e may not start at point p, but it
                    will be the most recent hull edge)
                 */
                Mode::Left => {
                    // Check the next hull edge to see if it either intersects
                    // the new line or terminates it.
                    let next_index = self.hull.prev_edge(
                        self.half.edge(current_edge_index).dst);
                    let next_edge = self.half.edge(next_index);

                    // If we've reached the target point, then rejoice!
                    if next_edge.dst == src {
                        break;
                    }
                    assert!(p != next_edge.dst);
                    let o = self.orient2d(p, src, next_edge.dst);
                    // If we're still outside the triangulation, then update
                    // the edge and continue
                    if o > 0.0 {
                        current_edge_index = next_index;
                    } else if o < 0.0 {
                        // Otherwise, we've crossed over and now need to run
                        // in the inside-the-triangulation mode.
                        current_edge_index = next_index;
                        mode = Mode::Inside;
                    } else {
                        // Lightly brushing by the point on the hull
                        assert!(false);
                    }
                }

                /*
                              p\
                            /  ^ \
                           /    \  \
                          /      \   \
                         /       e\    \
                        V          \     \
                       ------------>\      \
                                             \
                                              src
                 */
                Mode::Right => {
                    let next_index = self.hull.edge(
                        self.half.edge(current_edge_index).src);
                    let next_edge = self.half.edge(next_index);

                    if next_edge.src == src {
                        break; // rejoice!
                    }
                    assert!(p != next_edge.src);
                    let o = self.orient2d(p, next_edge.src, src);
                    if o > 0.0 {
                        current_edge_index = next_index;
                    } else if o < 0.0 {
                        current_edge_index = next_index;
                        mode = Mode::Inside;
                    } else {
                        // TODO
                        assert!(false); // stabbing the hull point
                    }
                }

                /*            p
                             :
                            :
                       b<--:-------a
                        \ :    e   ^
                         :\      /
                        :   v  /
                       :     c
                      src
                 */
                Mode::Inside => {
                    // There has been a p->src intersection with edge e,
                    // and we're now within a particular triangle.  We
                    // check which edge we exit along, and whether we exit
                    // into another triangle or back onto the hull
                    let edge_ab = self.half.edge(current_edge_index);
                    let e_bc = edge_ab.next;
                    let e_ca = edge_ab.prev;

                    let c = self.half.edge(e_bc).dst;

                    if c == src {
                        break; // rejoice!
                    }

                    let o_psc = self.orient2d(p, src, c);
                    if o_psc > 0.0 {
                        // Exiting the b->c edge
                        let buddy = self.half.edge(e_bc).buddy;
                        if buddy != half::EMPTY {
                            current_edge_index = buddy;
                        } else {
                            current_edge_index = e_bc;
                            assert!(default_mode != Mode::Inside);
                            mode = default_mode;
                        }
                    } else if o_psc < 0.0 {
                        // Exiting the c->a edge
                        let buddy = self.half.edge(e_ca).buddy;
                        if buddy != half::EMPTY {
                            current_edge_index = buddy;
                        } else {
                            current_edge_index = e_ca;
                            assert!(default_mode != Mode::Inside);
                            mode = default_mode;
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
        const SCALE: f64 = 250.0;
        let (x_bounds, y_bounds) = Self::bbox(&self.points);
        let line_width = (x_bounds.1 - x_bounds.0).max(y_bounds.1 - y_bounds.0) / 250.0 * SCALE;
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
