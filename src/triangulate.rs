use itertools::Itertools;

use crate::predicates::{acute, orient2d, in_circle};
use crate::{Point, PointIndex, PointVec, EdgeIndex};
use crate::contour::{Contour, ContourData};
use crate::{half, half::Half, hull::Hull, HullIndex};

const TERMINAL_LOWER_LEFT: PointIndex = PointIndex { val: 0 };
const TERMINAL_LOWER_RIGHT: PointIndex = PointIndex { val: 1 };

enum Walk {
    Outside(HullIndex),
    Inside(EdgeIndex, HullIndex),
    Done(EdgeIndex),
}

pub struct Triangulation {
    pub points: PointVec<Point>,    // Sorted in the constructor
    remap: PointVec<usize>,         // self.points[i] = input[self.remap[i]]
    next: PointIndex,               // Progress of the triangulation

    // If a point p terminates fixed edges, then endings[p] will be a tuple
    // range into ending_data containing the starting points of those edges.
    endings: PointVec<(usize, usize)>,
    ending_data: Vec<PointIndex>,

    // This stores the start of an edge (as a pseudoangle) as an index into
    // the edges array
    pub hull: Hull,
    pub half: Half,
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
        let y_bounds = (y_bounds.0 - dy / 8.0, y_bounds.1 + dy / 8.0);
        sorted_points.push((x_bounds.0, y_bounds.0));
        sorted_points.push((x_bounds.1, y_bounds.0));
        map_reverse.push(usize::MAX); // Dummy values
        map_reverse.push(usize::MAX);

        // Then, copy the rest of the sorted points into sorted_points and
        // store the full maps.
        for p in scratch.into_iter() {
            sorted_points.push(points[p.0]);
            map_forward[p.0] = map_reverse.push(p.0);
        }

        ////////////////////////////////////////////////////////////////////////
        let mut out = Triangulation {
            hull: Hull::new(x_bounds.0, x_bounds.1),
            half: Half::new(sorted_points.len()),

            remap: map_reverse,
            next: PointIndex::new(3), // we've already built a, b, c

            // Endings are assigned later
            endings: PointVec{ vec: vec![(0,0); sorted_points.len()] },
            ending_data: vec![],

            points: sorted_points, // moved out here
        };

        let pa = TERMINAL_LOWER_LEFT;
        let pb = TERMINAL_LOWER_RIGHT;
        let pc = PointIndex::new(2);

        let e_ab = out.half.insert(pa, pb, pc,
                                   half::EMPTY, half::EMPTY, half::EMPTY);
        assert!(e_ab == EdgeIndex::new(0));
        let e_bc = out.half.next(e_ab);
        let e_ca = out.half.prev(e_ab);

        let h_lower = out.hull.insert_lower_edge(e_ca);
        out.hull.insert(h_lower, out.points[pc].0, e_bc);

        ////////////////////////////////////////////////////////////////////////
        // Iterate over edges, counting which points have a termination
        let mut termination_count = PointVec { vec: vec![0; out.points.len()] };
        let edge_iter = || edges.clone()
            .into_iter()
            .map(|&(src, dst)| {
                let src = map_forward[src];
                let dst = map_forward[dst];
                assert!(src != usize::MAX);
                assert!(dst != usize::MAX);

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

    pub fn orient2d(&self, pa: PointIndex, pb: PointIndex, pc: PointIndex) -> f64 {
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
        let h_ab = self.hull.get(self.points[p].0);
        let e_ab = self.hull.edge(h_ab);

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
        let o = self.orient2d(b, a, p);
        assert!(o != 0.0);
        assert!(o > 0.0);

        let f = self.half.insert(b, a, p, half::EMPTY, half::EMPTY, e_ab);

        // Replaces the previous item in the hull
        self.hull.update(h_ab, self.half.prev(f));

        // Insert the new edge into the hull, using the previous HullIndex
        // as a hint to avoid searching for its position.
        let h_p = self.hull.insert(h_ab, self.points[p].0, self.half.next(f));

        self.legalize(f);

        // Check and fill acute angles
        self.check_acute_left(p, h_p);
        self.check_acute_right(p, h_p);

        // Finally, we check whether this point terminates any edges that are
        // locked in the triangulation (the "constrainted" part of Constrained
        // Delaunay Triangulation).
        let (start, end) = self.endings[p];
        for i in start..end {
            self.handle_fixed_edge(h_p, p, self.ending_data[i]);
        }

        true
    }

    fn check_acute_left(&mut self, p: PointIndex, h_p: HullIndex) {
        /* Search for sharp angles on the left side.
         *
         *      q       p [new point]
         *     / ^    e/ ^
         *    /   \   /   \
         *   /     \ V     \
         *          b------->
         */
        let mut h_b = h_p;
        loop {
            // Move one edge to the left.  In the first iteration of the loop,
            // h_b will be pointing at the b->p edge.
            h_b = self.hull.left_hull(h_b);
            let e_pb = self.hull.edge(h_b);
            let edge_pb = self.half.edge(e_pb);
            let b = edge_pb.dst;
            if b == TERMINAL_LOWER_LEFT {
                break;
            }

            // Pick out the next item in the list
            let h_q = self.hull.left_hull(h_b);
            let e_bq = self.hull.edge(h_q);
            let edge_bq = self.half.edge(e_bq);
            let q = edge_bq.dst;

            // Check that the inner angle is less that pi/2, skipping out
            // of the loop if that's not true.
            if self.acute(p, b, q) <= 0.0 {
                break;
            }
            // Sanity-check that the p-b-q triangle is correctly wound, which
            // should be guaranteed by construction
            assert!(self.orient2d(p, b, q) < 0.0);

            // Friendship ended with q-b-p
            self.hull.erase(h_b);

            // Now p-q is my new friend
            let e_pq = self.half.insert(p, q, b, e_bq, e_pb, half::EMPTY);
            self.hull.update(h_q, e_pq);
            h_b = h_p;

            // Then legalize from the two new triangle edges (bp and qb)
            self.legalize(self.half.next(e_pq));
            self.legalize(self.half.prev(e_pq));
        }
    }

    fn check_acute_right(&mut self, p: PointIndex, h_p: HullIndex) {
        /*  Rightward equivalent of check_acute_left
         *         p        q
         *        / ^      / \
         *       /   \e   /   \
         *      V     \  v     \
         *     -------->a       \
         */
        let mut h_a = h_p;
        loop {
            // Move one edge to the left.  In the first iteration of the loop,
            // h_a will be pointing at the p->a edge.
            let e_ap = self.hull.edge(h_a);
            let edge_ap = self.half.edge(e_ap);
            let a = edge_ap.src;
            if a == TERMINAL_LOWER_RIGHT {
                break;
            }

            // Scoot over by one to look at the a-q edge
            h_a = self.hull.right_hull(h_a);
            let e_qa = self.hull.edge(h_a);
            let edge_qa = self.half.edge(e_qa);
            let q = edge_qa.src;

            // Check the inner angle against pi/2
            if self.acute(p, a, q) <= 0.0 {
                break;
            }
            assert!(self.orient2d(p, a, q) > 0.0);

            self.hull.erase(h_a);
            let edge_qp = self.half.insert(q, p, a, e_ap, e_qa, half::EMPTY);
            self.hull.update(h_p, edge_qp);
            h_a = h_p;

            // Then legalize from the two new triangle edges (bp and qb)
            self.legalize(self.half.next(edge_qp));
            self.legalize(self.half.prev(edge_qp));
        }
    }

    /// Finds which mode to begin walking through the triangulation when
    /// inserting a fixed edge.  h is a HullIndex equivalent to the src point,
    /// and dst is the destination of the new fixed edge.
    fn find_hull_walk_mode(&self, h: HullIndex, src: PointIndex, dst: PointIndex)
        -> Walk {
        /*  We've just built a triangle that contains a fixed edge, and need
            to walk through the triangulation and implement that edge.

            The only thing we know going in is that point src is on the hull of
            the triangulation with HullIndex h.

            We start by finding the triangle a->src->b which contains the edge
            src->dst, e.g.

                     src
                     / :^
                    / :  \
                   /  :   \h
                  /  :     \
                 V   :      \
                b---:------->a
                    :
                   dst

            This triangle may not exist!  For example, if the p->src edge
            remains outside the hull on the left, then we start in Walk::Outside
        */
        let e_right = self.hull.edge(h);
        let h_left = self.hull.left_hull(h);
        let e_left = self.hull.edge(h_left);

        // Note that e_right-e_left may be a wedge that contains multiple
        // triangles (for example, this would be the case if there was an edge
        // flip of b->a)
        let wedge_left = self.half.edge(e_left).dst;
        let wedge_right = self.half.edge(e_right).src;

        // If the fixed edge is directly attached to src, then we can declare
        // that we're done right away (and the caller will lock the edge)
        if dst == wedge_left {
            return Walk::Done(e_left);
        } else if dst == wedge_right {
            return Walk::Done(e_right);
        }

        // Otherwise, check the winding to see which side we're on.
        let o_left = self.orient2d(src, wedge_left, dst);
        let o_right = self.orient2d(src, dst, wedge_right);

        // For now, we don't handle cases where fixed edges have coincident
        // points that are not the start/end of the fixed edge.
        assert!(o_left != 0.0);
        assert!(o_right != 0.0);

        // Easy cases: we're outside the wedge on one side or the other
        if o_left < 0.0 {
            return Walk::Outside(h_left);
        } else if o_right < 0.0 {
            return Walk::Outside(h);
        }

        // Walk the inside of the wedge until we find the
        // subtriangle which captures the p-src line.
        let mut index_a_src = self.half.edge(e_left).prev;

        loop {
            let edge_a_src = self.half.edge(index_a_src);
            let a = edge_a_src.src;
            if a == dst {
                /* Lucky break: the src point is one of the edges directly
                   within the wedge, e.g.:
                          src
                         / ^\
                        /  | \
                       /   |  \
                      /    |   \
                     V     |    \
                    ------>a------ (a == dst)
                */
                return Walk::Done(index_a_src);
            }

            // Keep walking through the fan
            let intersected_index = edge_a_src.prev;

            let o = self.orient2d(src, dst, a);
            assert!(o != 0.0);
            // If we've found the intersection point, then we return the new
            // (inner) edge.  The walking loop will transition to Left or Right
            // if this edge doesn't have a buddy.
            if o > 0.0 {
                /*
                          src
                         /:^\
                        / :| \
                       /  :|  \
                      /  : |   \
                     V   : |    \
                    ----:->a------
                        :
                       dst
                */
                // We may exit either into another interior triangle or
                // leave the triangulation and walk the hull, but we don't
                // need to decide that right now.
                return Walk::Inside(intersected_index, h);
            } else {
                /*  Sorry, Mario; your src-dst line is in another triangle

                          src
                         / ^:\
                        /  |: \
                       /   | : \
                      /    | :  \
                     V     |  :  \
                    ------>a--:---
                              :
                              dst

                    (so keep going through the triangle)
                */
                let buddy = edge_a_src.buddy;

                // We can't have walked out of the wedge, because otherwise
                // o_right would be < 0.0 and we wouldn't be in this branch
                assert!(buddy != half::EMPTY);
                index_a_src = self.half.edge(buddy).prev;
            }
        }
    }

    fn walk_fill_left(&mut self, src: PointIndex, dst: PointIndex, mut m: Walk) {
        let mut steps_above = Contour::new_pos(src, ContourData::None);
        let mut steps_below = Contour::new_neg(src, ContourData::None);

        // If we start inside a triangle, then escape it right away, because
        // Walk::Inside typically means means we've _entered_ through edge
        // `e`.
        if let Walk::Inside(e_ba, h) = m {
            /*
                         src
                         / :^
                        / :  \
                     hl/  :   \h
                      /  :     \
                     V   :  e   \
                    b---:------->a
                        :
                       dst
             */
            let edge_ba = self.half.edge(e_ba);
            let e_ac = edge_ba.next;
            let e_cb = edge_ba.prev;
            let edge_ac = self.half.edge(e_ac);
            let edge_cb = self.half.edge(e_cb);

            // Delete this triangle from the triangulation; it will be
            // reconstructed later in a more perfect form.
            self.half.erase(e_ba);

            steps_above.push(self, edge_ba.src,
                if edge_cb.buddy != half::EMPTY {
                    ContourData::Buddy(edge_cb.buddy)
                } else {
                    let hl = self.hull.left_hull(h);
                    ContourData::Hull(hl)
                });
            steps_below.push(self, edge_ba.dst,
                if edge_ac.buddy != half::EMPTY {
                    ContourData::Buddy(edge_ac.buddy)
                } else {
                    ContourData::Hull(h)
                });

            // Exit this triangle, either onto the hull or continuing inside
            // the triangulation.
            if edge_ba.buddy == half::EMPTY {
                m = Walk::Outside(self.hull.search_left(h, e_ba));
            } else {
                m = Walk::Inside(edge_ba.buddy, h);
            }
        }

        loop {
            match m {
                Walk::Outside(h) => {
                    /*
                                     /src
                                   / /  ^
                                 /  /    \
                               /   /h     \
                              /   /        \
                            /    V          \
                          /     ------------>\
                        dst

                        (as the loop runs, e may not start at src, but it
                        will be the most recent hull edge)
                     */
                    // Check the next hull edge to see if it either intersects
                    // the new line or terminates it.
                    let edge_index = self.hull.edge(h);
                    let edge = self.half.edge(edge_index);

                    // If we've reached the target point, then rejoice; the
                    // last point pushed to the contour should terminate the
                    // hull.
                    if edge.dst == dst {
                        let e_src_dst = steps_below.push(
                                self, dst, ContourData::Buddy(edge_index))
                            .expect("Failed to push last edge");

                        // This should have terminated the lower contour
                        assert!(self.half.edge(e_src_dst).src == src);
                        assert!(self.half.edge(e_src_dst).dst == dst);

                        // If we entered then exited the contour, then we'll
                        // triangulate an edge when pushing to steps_above;
                        // otherwise, we've only got two points in steps_above,
                        // and our newest edge is on the hull.
                        if let Some(e_dst_src) = steps_above.push(
                            self, dst, ContourData::Hull(h))
                        {
                            assert!(self.half.edge(e_dst_src).src == dst);
                            assert!(self.half.edge(e_dst_src).dst == src);
                            self.half.link(e_src_dst, e_dst_src);
                            self.half.lock(e_src_dst);
                        } else {
                            self.hull.update(h, e_src_dst);
                        }

                        break;
                    }
                    assert!(src != edge.dst);

                    // If we're intersecting this edge, then things get tricky
                    if self.orient2d(src, dst, edge.dst) <= 0.0 {
                        steps_above.push(self, edge.dst,
                                         ContourData::Hull(h));
                        m = Walk::Inside(edge_index, h);
                        // We leave this hull intact, because it will be updated
                        // once the triangulation reaches it.
                    } else {
                        // If we're still outside the triangulation, then keep
                        // walking along the hull
                        steps_below.push(self, edge.dst,
                                         ContourData::Buddy(edge_index));
                        m = Walk::Outside(self.hull.left_hull(h));
                        self.hull.erase(h);
                    }
                }
                Walk::Inside(e_ab, h) => {
                        /*            src
                                     :
                               b<--:-------a
                                \ :  e     ^
                                 :\      /
                                :   v  /
                               :     c
                              dst
                         */
                    let edge_ab = self.half.edge(e_ab);
                    let a = edge_ab.src;
                    let b = edge_ab.dst;
                    let e_bc = edge_ab.next;
                    let e_ca = edge_ab.prev;
                    let edge_bc = self.half.edge(e_bc);
                    let edge_ca = self.half.edge(e_ca);
                    let c = edge_bc.dst;

                    // Erase this triangle from the triangulation before
                    // pushing vertices to the contours, which could create
                    // new triangles.  At this point, you're not allowed to use
                    // self.half for any of the triangle edges, which is why
                    // we stored them all above.
                    self.half.erase(e_ab);

                    // Handle the termination case, if c is the destination
                    if c == dst {
                        // The left (above) contour is either on the hull
                        // (if no buddy is present) or inside the triangulation
                        let e_dst_src = steps_above.push(self, c,
                            if edge_bc.buddy == half::EMPTY {
                                ContourData::Hull(
                                    self.hull.search_left(h, e_bc))
                            } else {
                                ContourData::Buddy(edge_bc.buddy)
                            }).expect("Failed to create fixed edge");

                        // This better have terminated the triangulation of
                        // the upper contour with a dst-src edge
                        assert!(self.half.edge(e_dst_src).dst == src);
                        assert!(self.half.edge(e_dst_src).src == dst);

                        // The other contour will finish up with the other
                        // half of the fixed edge as its buddy.  This edge
                        // could also be on the hull, so we do the same check
                        // as above.
                        let e_src_dst = steps_below.push(self, c,
                            if edge_ca.buddy == half::EMPTY {
                                ContourData::Hull(
                                    self.hull.search_left(h, e_ca))
                            } else {
                                ContourData::Buddy(edge_ca.buddy)
                            })
                            .expect("Failed to create second fixed edge");

                        // Similarly, this better have terminated the
                        // triangulation of the lower contour.
                        assert!(self.half.edge(e_src_dst).src == src);
                        assert!(self.half.edge(e_src_dst).dst == dst);

                        self.half.link(e_src_dst, e_dst_src);
                        self.half.lock(e_src_dst); // locks both sides

                        break;
                    }

                    let o_psc = self.orient2d(src, dst, c);
                    if o_psc > 0.0 {
                        // Store the c-a edge as our buddy, and exit via b-c
                        assert!(edge_ca.buddy != half::EMPTY);
                        steps_below.push(self, c,
                                         ContourData::Buddy(edge_ca.buddy));

                        // Exit the triangle, either onto the hull or staying
                        // in the triangulation
                        m = if edge_bc.buddy == half::EMPTY {
                            let h = self.hull.search_left(h, e_bc);
                            let hl = self.hull.left_hull(h);
                            self.hull.erase(h);
                            Walk::Outside(hl)
                        } else {
                            Walk::Inside(edge_bc.buddy, h)
                        };
                    } else if o_psc < 0.0 {
                        /*         src
                                    :
                               b<-- :-a
                                |  : ^
                                |  :/
                                | :/
                                | :
                                V/:
                                c dst
                         */
                        // Store the b-c edge as our buddy and exit via c-a,
                        // which is guaranteed to stay in the triangulation
                        //
                        // (c-b may be a hull edge, so we check for that)
                        let mut h = h;
                        steps_above.push(self, c,
                            if edge_bc.buddy == half::EMPTY {
                                // Notice that this modifies h (which is stored
                                // into m below), so that we don't walk as far
                                // next time.
                                let h_c = self.hull.search_left(h, e_bc);
                                h = self.hull.left_hull(h_c);
                                ContourData::Hull(h_c)
                            } else {
                                ContourData::Buddy(edge_bc.buddy)
                            });

                        assert!(edge_ca.buddy != half::EMPTY);
                        m = Walk::Inside(edge_ca.buddy, h);
                    } else {
                        assert!(false);
                    }
                }
                _ => panic!("Invalid walk mode"),
            }
        }
    }

    fn walk_fill_right(&mut self, src: PointIndex, dst: PointIndex, m: Walk) {
        /*
        let mut steps_below: Vec<HullIndex> = Vec::new();
        let mut h = start;
        loop {
            /*
                         src
                        /  ^ \
                       /    \  \
                      /      \   \
                     /       h\    \
                    V          \     \
                   ------------->      \
                                         \
                                         dst
             */
            let index = self.hull.edge(h);
            let edge = self.half.edge(index);
            steps_below.push(h);

            if edge.src == dst {
                break;
            }
            assert!(src != edge.src);

            if self.orient2d(src, edge.src, dst) <= 0.0 {
                return false;
            }
            h = self.hull.right_hull(h);
        }
        let pts: Vec<(PointIndex, EdgeIndex)> = steps_below.iter()
            .map(|h| {
                let e = self.hull.edge(*h);
                let edge = self.half.edge(e);
                let pt = edge.dst;
                if pt != src {
                    self.hull.erase(*h);
                }
                (pt, e)
            })
            .chain(std::iter::once((dst, half::EMPTY)))
            .collect();

        // Fill this polygon, returning the new edge
        let new_edge = self.fill_monotone(&pts);
        assert!(self.half.edge(new_edge).src == dst);
        assert!(self.half.edge(new_edge).dst == src);

        self.hull.update(start, new_edge);
        self.half.lock(new_edge);
        */
    }

    fn handle_fixed_edge(&mut self, h: HullIndex, src: PointIndex, dst: PointIndex) {
        match self.find_hull_walk_mode(h, src, dst) {
            // Easy mode: the fixed edge is directly connected to the new
            // point, so we lock it and return immediately.
            Walk::Done(e) => self.half.lock(e),

            // Otherwise, walk either to the left or the right depending on
            // the positions of src and dst.
            m => if self.points[dst].0 < self.points[src].0 {
                self.walk_fill_left(src, dst, m)
            } else {
                self.walk_fill_right(src, dst, m)
            },
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
        if edge.fixed || edge.buddy == half::EMPTY {
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

    pub fn save_svg(&self, filename: &str) {
        std::fs::write(filename, self.to_svg())
            .expect("Failed to write file");
    }

    pub fn to_svg(&self) -> String {
        const SCALE: f64 = 250.0;
        let (x_bounds, y_bounds) = Self::bbox(&self.points);
        let line_width = (x_bounds.1 - x_bounds.0)
            .max(y_bounds.1 - y_bounds.0) / 250.0 * SCALE;
        let dx = |x| { SCALE * (x - x_bounds.0) + line_width};
        let dy = |y| { SCALE * (y_bounds.1 - y) + line_width};

         let mut out = String::new();
         // Put a dummy rectangle in the SVG so that rsvg-convert doesn't clip
         out.push_str(&format!(
            r#"<svg viewbox="auto" xmlns="http://www.w3.org/2000/svg">
    <rect x="0" y="0" width="{}" height="{}"
     style="fill:rgb(0,0,0)" />"#,
     dx(x_bounds.1) + line_width,
     dy(y_bounds.0) + line_width));

        // Draw endings in green (they will be overdrawn in white if they're
        // included in the triangulation).
        for (p, (start, end)) in self.endings.iter().enumerate() {
            for i in *start..*end {
                let dst = PointIndex::new(p);
                let src = self.ending_data[i];
                 out.push_str(&format!(
                    r#"
        <line x1="{}" y1="{}" x2="{}" y2="{}"
         style="stroke:rgb(0,255,0)"
         stroke-width="{}" stroke-linecap="round" />"#,
                    dx(self.points[src].0),
                    dy(self.points[src].1),
                    dx(self.points[dst].0),
                    dy(self.points[dst].1),
                    line_width));
            }
        }

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