use itertools::Itertools;

use crate::predicates::{acute, orient2d, in_circle};
use crate::{Point, PointIndex, PointVec, EdgeIndex};
use crate::{half, half::Half, sweepline::hull::Hull, HullIndex};

const TERMINAL_LEFT: PointIndex = PointIndex { val: 0 };
const TERMINAL_RIGHT: PointIndex = PointIndex { val: 1 };

enum WalkMode {
    Left(HullIndex),
    Right(HullIndex),
    Inside(EdgeIndex),
    Done(EdgeIndex),
    Nope,
}

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
            hull: Hull::new(x_bounds.0, x_bounds.1),
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

        let h_lower = out.hull.insert_lower_edge(e_ca);
        out.hull.insert(h_lower, out.points[pc].0, e_bc);

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
        let (start, end) = self.endings[p];
        let mut i = start;
        while i != end {
            if self.handle_fixed_edge(h_p, p, self.ending_data[i]) {
                self.ending_data.swap(i, end - 1);
                self.endings[p].1 -= 1;
            } else {
                i += 1;
            }
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
            if b == TERMINAL_LEFT {
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
            if a == TERMINAL_RIGHT {
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
    fn find_walk_mode(&self, h: HullIndex, src: PointIndex, dst: PointIndex)
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
                let intersected_edge = self.half.edge(intersected_index);
                if intersected_edge.buddy == half::EMPTY {
                    return WalkMode::Nope;
                } else {
                    return WalkMode::Inside(intersected_edge.buddy);
                }
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

    fn walk_fill_left(&mut self, src: PointIndex, dst: PointIndex, mut h: HullIndex) -> bool {
        let mut steps_below: Vec<HullIndex> = Vec::new();
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
            steps_below.push(h);

            // Check the next hull edge to see if it either intersects
            // the new line or terminates it.
            let index = self.hull.edge(h);
            let edge = self.half.edge(index);

            // If we've reached the target point, then rejoice!
            if edge.dst == dst {
                break;
            }
            assert!(src != edge.dst);

            let o = self.orient2d(src, dst, edge.dst);
            if o > 0.0 {
                // If we're still outside the triangulation, then keep
                // walking along the hull
                h = self.hull.left_hull(h);
            } else {
                // If we're intersecting this edge, then it's too hard
                // to handle now, and we'll do it later.
                return false;
            }
        }
        // TODO: triangulate
        true
    }

    fn walk_fill_right(&mut self, src: PointIndex, dst: PointIndex, mut h: HullIndex) -> bool {
        let mut steps_below: Vec<HullIndex> = Vec::new();
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
            steps_below.push(h);

            let index = self.hull.edge(h);
            let edge = self.half.edge(index);

            if edge.src == dst {
                break;
            }
            assert!(src != edge.src);

            let o = self.orient2d(src, edge.src, dst);
            if o > 0.0 {
                h = self.hull.right_hull(h);
            } else {
                return false;
            }
        }
        // TODO: triangulate
        true
    }

    fn walk_fill_inside(&mut self, src: PointIndex, dst: PointIndex, mut e: EdgeIndex) -> bool {
        let mut steps_left: Vec<EdgeIndex> = Vec::new();
        let mut steps_right: Vec<EdgeIndex> = Vec::new();
        loop {
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
                break; // rejoice!
            }

            let o_psc = self.orient2d(src, dst, c);
            if o_psc > 0.0 {
                // Exiting the triangle via b-c
                let buddy = self.half.edge(e_bc).buddy;
                if buddy == half::EMPTY {
                    return false;
                }
                e = buddy;
                steps_right.push(e_ca);
            } else if o_psc < 0.0 {
                let buddy = self.half.edge(e_ca).buddy;
                if buddy == half::EMPTY {
                    return false;
                }
                e = buddy;
                steps_left.push(e_bc);
            } else {
                return false; // Direct hit on c, deal with it later
            }
        }
        // TODO: triangulate
        true
    }

    fn handle_fixed_edge(&mut self, h: HullIndex, src: PointIndex, dst: PointIndex) -> bool {
        match self.find_walk_mode(h, src, dst) {
            // Easy mode: the fixed edge is directly connected to the new
            // point, so we lock it and return imemdiately.
            WalkMode::Done(e) => {
                self.half.lock(e);
                true
            },
            // Hard mode: the fxed edge emerges from a mid-wedge triangle,
            // and we don't want to deal with that right now.
            WalkMode::Nope => false,

            // Otherwise, record the direction and continue
            WalkMode::Left(h) => self.walk_fill_left(src, dst, h),
            WalkMode::Right(h) => self.walk_fill_right(src, dst, h),
            WalkMode::Inside(e) => self.walk_fill_inside(src, dst, e),
        }
    }

    fn fill_monotone_mountain(&mut self, pts: &[PointIndex]) -> EdgeIndex {
        // Based on "Triangulating Monotone Mountains",
        // http://www.ams.sunysb.edu/~jsbm/courses/345/13/triangulating-monotone-mountains.pdf
        //
        // pts sould be a left-to-right set of Y-monotone points

        // Build a tiny flat pseudo-linked list representing the contour
        struct Node {
            prev: usize,
            next: usize,
            pt: PointIndex,
            buddy: EdgeIndex,
        }
        let mut pts: Vec<Node> = pts.iter().enumerate()
            .map(|(i, p)| Node {
                prev: if i == 0 { usize::MAX } else { i - 1 },
                next: if i == pts.len() - 1 { i + 1 } else { usize::MAX },
                pt: *p,
                buddy: half::EMPTY,
            })
            .collect();

        let mut i = 1;
        // Run until the last triangle is flattened out
        while pts[0].next != pts.len() - 1 {
            let prev = pts[i].prev;
            let next = pts[i].next;
            // If this ear is strictly convex, then clip it!
            if self.orient2d(pts[prev].pt, pts[next].pt, pts[i].pt) > 0.0 {
                // Write a new triangle and record its inside edge as a new
                // buddy for the earliest point in the ear, overwriting the
                // previous buddy (which may have been an external edge)
                let prev = pts[i].prev;
                let next = pts[i].next;
                let e = self.half.insert(
                    pts[prev].pt, pts[next].pt, pts[i].pt,
                    pts[i].buddy, pts[prev].buddy, half::EMPTY);
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
