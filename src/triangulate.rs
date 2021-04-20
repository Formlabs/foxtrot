use itertools::Itertools;

use crate::predicates::{acute, orient2d, in_circle};
use crate::{Point, PointIndex, PointVec, EdgeIndex};
use crate::{half, half::Half, hull::Hull, HullIndex};

const TERMINAL_LOWER_LEFT: PointIndex = PointIndex { val: 0 };
const TERMINAL_LOWER_RIGHT: PointIndex = PointIndex { val: 1 };

enum WalkMode {
    Left(HullIndex),
    Right(HullIndex),
    Inside(EdgeIndex),
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

    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
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

        // If we have fixed edges, then add *another* two phantom points to
        // the top of the model, so that we can fully close it off and
        // guarantee that it's convex.
        if edges.clone().into_iter().next().is_some() {
            sorted_points.push((x_bounds.0 + dx / 16.0, y_bounds.1));
            sorted_points.push((x_bounds.1 - dx / 16.0, y_bounds.1));
            map_reverse.push(usize::MAX); // Dummy value
            map_reverse.push(usize::MAX);
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
            x_bounds, y_bounds,
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

    pub fn finalize_edges(&mut self) {
        // Put a roof over the hull, to ensure that it's convex and all edges
        // will walk completely inside of the triangulation.
        let h = self.hull.right_hull(self.hull.start());
        self.walk_fill_right(
            PointIndex::new(self.points.len() - 2),
            PointIndex::new(self.points.len() - 1),
            h);
        self.half.unlock(self.hull.edge(h));

        // Now, let's add bonus points so that every edge that the triangulation
        // may try to delete has a buddy.  This is basically injecting a diamond
        // shape around the square of the bounding box
        let dx = (self.x_bounds.1 - self.x_bounds.0) / 8.0;
        let x_mid = (self.x_bounds.0 + self.x_bounds.1) / 2.0;
        let dy = (self.y_bounds.1 - self.y_bounds.0) / 8.0;
        let y_mid = (self.y_bounds.0 + self.y_bounds.1) / 2.0;
        {
            let e_lower = EdgeIndex::new(0);
            let edge_lower = self.half.edge(e_lower);
            let pt_lower = self.points.push((x_mid, self.y_bounds.0 - dy));
            self.half.insert(edge_lower.dst, edge_lower.src, pt_lower,
                             half::EMPTY, half::EMPTY, EdgeIndex::new(0));
        }
        {
            let e_left = self.hull.edge(self.hull.start());
            let edge_left = self.half.edge(e_left);
            let pt_left = self.points.push((self.x_bounds.0 - dx, y_mid));
            self.half.insert(edge_left.dst, edge_left.src, pt_left,
                             half::EMPTY, half::EMPTY, e_left);
        }
        {
            let e_top = self.hull.edge(h);
            let edge_top = self.half.edge(e_top);
            let pt_top = self.points.push((x_mid, self.y_bounds.1 + dy));
            self.half.insert(edge_top.dst, edge_top.src, pt_top,
                             half::EMPTY, half::EMPTY, e_top);
        }
        {
            let e_right = self.hull.edge(self.hull.right_hull(h));
            let edge_right = self.half.edge(e_right);
            let pt_right = self.points.push((self.x_bounds.1 + dx, y_mid));
            self.half.insert(edge_right.dst, edge_right.src, pt_right,
                             half::EMPTY, half::EMPTY, e_right);
        }

        // Steal the PointVec of endings from &self, so we can call mutable
        // functions later on inside the loop.  Don't worry, we'll give it back.
        let mut endings = PointVec::new();
        std::mem::swap(&mut endings, &mut self.endings);

        for (pt, (start, end)) in endings.iter_mut().enumerate() {
            let pt = PointIndex::new(pt);
            for i in (*start..*end).rev() {
                let e = self.half.edge_index(pt);
                assert!(e != half::EMPTY);
                let edge = self.half.edge(e);
                assert!(edge.src == pt);
                let r = self.finalize_fixed_edge(pt, self.ending_data[i], edge.next);
                assert!(r);
            }
            *end = *start;
        }
        std::mem::swap(&mut endings, &mut self.endings);
    }

    /// Finalizes a fixed edge in a fully connected (convex) model
    /// e is an edge opposite src; dst is the end of the fixed edge
    pub fn finalize_fixed_edge(&mut self, src: PointIndex, dst: PointIndex, mut e: EdgeIndex) -> bool {
        /*
                     src
                     / :^
                    / :  \
                   /  :   \
                  /  :     \
                 V   : e    \
                a---:------->b
                    :
                   dst
        */
        // Find the edge which contains the src-dst line, by walking around
        // the triangulation
        loop {
            let edge = self.half.edge(e);
            let a = edge.src;
            let b = edge.dst;

            // Easy mode: the triangle is directly connected
            if a == dst {
                self.half.lock(edge.prev);
                return true;
            } else if b == dst {
                self.half.lock(edge.next);
                return true;
            }

            let oa = self.orient2d(src, a, dst);
            let ob = self.orient2d(src, dst, b);
            assert!(oa != 0.0);
            assert!(ob != 0.0);
            if oa > 0.0 && ob >= 0.0 {
                return self.walk_fill_inside(src, dst, e);
            } else {
                // Keep walking arorund the triangle fan
               let b = self.half.edge(edge.next).buddy;
               assert!(b != half::EMPTY);
               e = self.half.edge(b).next;
               assert!(e != half::EMPTY);
            }
        }
    }

    pub fn orient2d(&self, pa: PointIndex, pb: PointIndex, pc: PointIndex) -> f64 {
        orient2d(self.points[pa], self.points[pb], self.points[pc])
    }

    fn acute(&self, pa: PointIndex, pb: PointIndex, pc: PointIndex) -> f64 {
        acute(self.points[pa], self.points[pb], self.points[pc])
    }

    pub fn step(&mut self) -> bool {
        if self.next == self.points.len() {
            self.finalize_edges();
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
        // Delaunay Triangulation).  If this is an easy point (i.e. handled
        // by handle_fixed_edge), then we swap it to the back of the
        // list and reduce the number of edges associated with this point;
        // otherwise, we'll deal with it later.
        let (start, mut end) = self.endings[p];
        let mut i = start;
        while i != end {
            if self.handle_fixed_edge(h_p, p, self.ending_data[i]) {
                end -= 1;
                self.ending_data.swap(i, end);
            } else {
                i += 1;
            }
        }
        self.endings[p].1 = end;

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
        -> WalkMode {
        /*  We've just built a triangle that contains a fixed edge, and need
            to walk through the triangulation and implement that edge.

            The only thing we know going in is that point src is on the hull of
            the triangulation with HullIndex h.

            We start by finding the triangle a->src->b which contains the edge
            src->dst, e.g.

                     src
                     / :^
                    / :  \
                   /  :   \
                  /  :     \
                 V   :      \
                b---:------->a
                    :
                   dst

            This triangle may not exist!  For example, if the p->src edge
            remains outside the hull on the left, then we start in Mode::Left
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
            return WalkMode::Done(e_left);
        } else if dst == wedge_right {
            return WalkMode::Done(e_right);
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
            return WalkMode::Left(h_left);
        } else if o_right < 0.0 {
            return WalkMode::Right(h);
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
                return WalkMode::Done(index_a_src);
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
                return WalkMode::Inside(intersected_index);
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

    fn walk_fill_left(&mut self, src: PointIndex, dst: PointIndex, start: HullIndex) -> bool {
        let mut steps_below: Vec<HullIndex> = Vec::new();
        let mut h = start;
        loop {
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
            let index = self.hull.edge(h);
            let edge = self.half.edge(index);
            steps_below.push(h);

            // If we've reached the target point, then rejoice!
            if edge.dst == dst {
                break;
            }
            assert!(src != edge.dst);

            // If we're intersecting this edge, then it's too hard
            // to handle now, and we'll do it later.
            if self.orient2d(src, dst, edge.dst) <= 0.0 {
                return false;
            }

            // If we're still outside the triangulation, then keep
            // walking along the hull
            h = self.hull.left_hull(h);
        }

        let pts: Vec<(PointIndex, EdgeIndex)> = steps_below.iter()
            .rev()
            .map(|h| {
                let e = self.hull.edge(*h);
                let edge = self.half.edge(e);
                let pt = edge.dst;
                if pt != dst {
                    self.hull.erase(*h);
                }
                (pt, e)
            })
            .chain(std::iter::once((src, half::EMPTY)))
            .collect();

        // Fill this polygon, returning the new edge
        let new_edge = self.fill_monotone(&pts);
        assert!(self.half.edge(new_edge).src == src);
        assert!(self.half.edge(new_edge).dst == dst);

        self.hull.update(h, new_edge);
        self.half.lock(new_edge);
        true
    }

    fn walk_fill_right(&mut self, src: PointIndex, dst: PointIndex, start: HullIndex) -> bool {
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
        true
    }

    fn walk_fill_inside(&mut self, src: PointIndex, dst: PointIndex, mut e: EdgeIndex) -> bool {
        #[derive(Debug, Copy, Clone)]
        enum Step {
            Start,
            Left,
            Right,
            End,
        }

        /*
                     src
                     / :^
                    / :  \
                   /  :   \
                  /  :     \
                 V   :  e   \
                b---:------->a
                    :
                   dst
         */
        let edge = self.half.edge(e);
        if self.half.edge(edge.prev).buddy == half::EMPTY {
            return false;
        }
        if self.half.edge(edge.next).buddy == half::EMPTY {
            return false;
        }

        let mut steps: Vec<(Step, EdgeIndex)> = vec![(Step::Start, e)];
        e = edge.buddy; // If we exit the triangle, then we'll catch it below

        loop {
            if e == half::EMPTY {
                return false;
            }
            /*            src
                         :
                        :
                   b<--:-------a
                    \ :    e   ^
                     :\      /
                    :   v  /
                   :     c
                  dst
             */
            // There has been a src->dst intersection with edge e,
            // and we're now within a particular triangle.  We
            // check which edge we exit along; delegating the decision
            // whether we stay in the triangulation or not to
            // WalkMode::Exit.
            let edge_ab = self.half.edge(e);
            let e_bc = edge_ab.next;
            let e_ca = edge_ab.prev;

            let c = self.half.edge(e_bc).dst;

            if c == dst {
                let e_bc_buddy = self.half.edge(e_bc).buddy;
                if e_bc_buddy == half::EMPTY {
                    return false;
                }

                let e_ca_buddy = self.half.edge(e_ca).buddy;
                if e_ca_buddy == half::EMPTY {
                    return false;
                }
                steps.push((Step::End, e));
                break; // rejoice!
            }

            let o_psc = self.orient2d(src, dst, c);
            if o_psc > 0.0 {
                // Store the c-a edge as our buddy
                if self.half.edge(e_ca).buddy == half::EMPTY {
                    return false;
                }
                steps.push((Step::Right, e_ca));

                // Exiting the triangle via b-c
                e = self.half.edge(e_bc).buddy;
            } else if o_psc < 0.0 {
                // Store the b-c edge as our buddy
                if self.half.edge(e_bc).buddy == half::EMPTY {
                    return false;
                }
                steps.push((Step::Left, e_bc));

                // Exit the triangle via c-a
                e = self.half.edge(e_ca).buddy;
            } else {
                return false; // Direct hit on c, deal with it later
            }
        }

        //////////////////////////////////////////////////////////////////////
        let mut pts_left: Vec<(PointIndex, EdgeIndex)> = Vec::new();
        let mut pts_right: Vec<(PointIndex, EdgeIndex)> = Vec::new();
        let mut i = 0;
        println!("{}", self.to_svg());
        while i < steps.len() {
            let (s, e) = steps[i].clone();
            match s {
                Step::Start => {
                    let edge = self.half.edge(e);
                    let left_edge = self.half.edge(edge.prev);
                    let right_edge = self.half.edge(edge.next);
                    pts_left.push((left_edge.src, left_edge.buddy));
                    pts_right.push((right_edge.src, right_edge.buddy));
                },
                Step::End => {
                    let edge = self.half.edge(e);
                    let left_edge = self.half.edge(edge.next);
                    let right_edge = self.half.edge(edge.prev);
                    pts_left.push((left_edge.src, left_edge.buddy));
                    pts_right.push((right_edge.src, right_edge.buddy));
                },
                Step::Right => {
                    let right_edge = self.half.edge(e);
                    if right_edge.buddy == half::EMPTY {
                        // In certain rare cases, deleting interior triangles
                        // during this walk step can leave a point orphaned.
                        //
                        // We handle this by constructing a triangle that
                        // reattaches that point to the wall of the pseudo-
                        // polygon, then adjusting the wall accordingly.
                        pts_right.pop().expect("Could not pop orphan edge");
                        let (_, e_wall) = pts_right.pop()
                            .expect("Failed to get wall edge");
                        let wall_edge = self.half.edge(e_wall);
                        let new_edge_index = self.half.insert(
                            wall_edge.dst,
                            wall_edge.src,
                            right_edge.dst,
                            half::EMPTY,
                            half::EMPTY,
                            e_wall);
                        let new_edge = self.half.edge(new_edge_index);
                        pts_right.push((right_edge.dst, new_edge.next));
                        self.half.link(e, new_edge.prev);
                        self.legalize(new_edge_index);

                        // Revisit this triangle now that its edge has a buddy
                        continue;
                    } else {
                        pts_right.push((right_edge.src, right_edge.buddy));
                    }
                }
                Step::Left => {
                    let left_edge = self.half.edge(e);
                    if left_edge.buddy == half::EMPTY {
                        pts_left.pop().expect("Could not pop orphan edge");
                        let (_, e_wall) = pts_left.pop()
                            .expect("Failed to get wall edge");
                        let wall_edge = self.half.edge(e_wall);
                        let new_edge_index = self.half.insert(
                            wall_edge.dst,
                            wall_edge.src,
                            left_edge.src,
                            half::EMPTY,
                            half::EMPTY,
                            e_wall);
                        let new_edge = self.half.edge(new_edge_index);
                        pts_left.push((wall_edge.dst, new_edge.prev));
                        self.half.link(e, new_edge.next);
                        self.legalize(new_edge_index);
                        continue;
                    } else {
                        pts_left.push((left_edge.src, left_edge.buddy));
                    }
                }
            }
            self.half.erase(e);
            i += 1;
        }
        pts_left.push((dst, half::EMPTY));
        pts_right.reverse();
        pts_right.push((src, half::EMPTY));

        // Triangulate the left and right pseudopolygons
        let new_edge_left = self.fill_monotone(&pts_left);
        assert!(self.half.edge(new_edge_left).src == dst);
        assert!(self.half.edge(new_edge_left).dst == src);
        self.half.lock(new_edge_left);

        println!("{}", self.to_svg());
        let new_edge_right = self.fill_monotone(&pts_right);
        assert!(self.half.edge(new_edge_right).src == src);
        assert!(self.half.edge(new_edge_right).dst == dst);
        self.half.lock(new_edge_right);

        // Set them as each other's buddies
        self.half.link(new_edge_left, new_edge_right);

        true
    }

    fn handle_fixed_edge(&mut self, h: HullIndex, src: PointIndex, dst: PointIndex) -> bool {
        match self.find_hull_walk_mode(h, src, dst) {
            // Easy mode: the fixed edge is directly connected to the new
            // point, so we lock it and return immediately.
            WalkMode::Done(e) => {
                self.half.lock(e);
                true
            },

            // Otherwise, record the direction and continue
            WalkMode::Left(h) => self.walk_fill_left(src, dst, h),
            WalkMode::Right(h) => self.walk_fill_right(src, dst, h),
            WalkMode::Inside(e) => self.walk_fill_inside(src, dst, e),
        }
    }

    fn fill_monotone(&mut self, pts: &[(PointIndex, EdgeIndex)]) -> EdgeIndex {

        /*  Based on "Triangulating Monotone Mountains",
            http://www.ams.sunysb.edu/~jsbm/courses/345/13/triangulating-monotone-mountains.pdf

            pts should be a left-to-right set of Y-monotone points, such that
            the edge pts[0]->pts[pts.len() - 1] is above all points in the list.

            0----------------------------end
             \                          /
            b0\      ^x                /
               \    /  \b2            /
                v  /b1  vx---------->x
                 x/

            Alternatively, this will also work on a right-to-left mountain

                 x^         b1
                /  \    /x<----------x
               /    \  /b2            ^
              /      xv                \b0
             v                          \
          end--------------------------->0

            Each point is associated with a buddy edge, shown as b0... in the
            sketches above.  This edge is used to link us into the half-edge
            data structure.
        */

        // Build a tiny flat pseudo-linked list representing the contour
        #[derive(Debug)]
        struct Node {
            prev: usize,
            next: usize,
            pt: PointIndex,
            buddy: EdgeIndex,
        }
        let mut pts: Vec<Node> = pts.iter().enumerate()
            .map(|(i, p)| {
                // "Do you have your exit buddy?"
                //  (Finding Nemo, 2003)
                if i != pts.len() - 1 {
                    assert!(p.1 != half::EMPTY);
                }

                // Build a node in the pseudo-linked list
                Node {
                    prev: if i == 0 { usize::MAX } else { i - 1 },
                    next: if i == pts.len() - 1 { usize::MAX } else { i + 1 },
                    pt: p.0,
                    buddy: p.1,
                }
            }).collect();

        let mut i = 1;
        // Run until the last triangle is flattened out
        while pts[0].next != pts.len() - 1 {
            let prev = pts[i].prev;
            let next = pts[i].next;
            /*
              prev
                 \       next
                  \      ^
                   \    /
                    \  /
                     v/
                     i

                If this ear is strictly convex, then clip it!
            */
            if self.orient2d(pts[prev].pt, pts[i].pt, pts[next].pt) > 0.0 {
                // Write a new triangle and record its inside edge as a new
                // buddy for the earliest point in the ear, overwriting the
                // previous buddy (which may have been an external edge)
                let e = self.half.insert(
                    pts[next].pt, pts[prev].pt, pts[i].pt,
                    pts[prev].buddy, pts[i].buddy, half::EMPTY);
                pts[prev].buddy = e;

                // Remove point i out of the linked list
                pts[prev].next = next;
                pts[next].prev = prev;

                // Backtrack, unless the previous point is the head of the
                // list.  It's not guaranteed that we've unlocked another
                // convex point, but this is innocuous, since if it's not
                // convex, then we'll walk forward in the next iteration.
                i = if prev != 0 { prev } else { next };
            } else {
                i = next;
            }
        }
        assert!(pts[0].buddy != half::EMPTY);
        pts[0].buddy
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

        // Draw remaining endings in green
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
