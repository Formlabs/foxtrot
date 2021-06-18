use crate::{
    contour::{Contour, ContourData},
    Error, Point,
    half::Half, hull::Hull,
    indexes::{PointIndex, PointVec, EdgeIndex, HullIndex, EMPTY_EDGE},
    predicates::{acute, orient2d, in_circle, centroid, distance2, pseudo_angle},
};

#[derive(Debug)]
enum Walk {
    Inside(EdgeIndex),
    Done(EdgeIndex),
}

/// This `struct` contains all of the data needed to generate a (constrained)
/// Delaunay triangulation of a set of input points and edges.  It is a
/// **low-level** API; consider using the module-level functions if you don't
/// need total control.
pub struct Triangulation {
    pub(crate) points: PointVec<Point>,    // Sorted in the constructor
    angles: PointVec<f64>,          // pseudo-angles for each point
    remap: PointVec<usize>,         // self.points[i] = input[self.remap[i]]
    next: PointIndex,               // Progress of the triangulation
    constrained: bool,

    // If a point p terminates fixed edges, then endings[p] will be a tuple
    // range into ending_data containing the starting points of those edges.
    endings: PointVec<(usize, usize)>,
    ending_data: Vec<PointIndex>,

    // This stores the start of an edge (as a pseudoangle) as an index into
    // the edges array
    pub(crate) hull: Hull,
    pub(crate) half: Half,
}

impl Triangulation {
    /// Builds a complete triangulation from the given points
    ///
    /// # Errors
    /// This may return [`Error::EmptyInput`], [`Error::InvalidInput`], or
    /// [`Error::CannotInitialize`] if the input is invalid.
    pub fn build(points: & [Point]) -> Result<Triangulation, Error> {
        let mut t = Self::new(points)?;
        t.run()?;
        Ok(t)
    }

    /// Builds a complete triangulation from the given points and edges.
    /// The points are a flat array of positions in 2D spaces; edges are
    /// undirected and expressed as indexes into the points list.
    ///
    /// # Errors
    /// This may return [`Error::EmptyInput`], [`Error::InvalidInput`],
    /// [`Error::InvalidEdge`], or [`Error::CannotInitialize`] if the input is
    /// invalid.
    pub fn build_with_edges<'a, E>(points: &[Point], edges: E)
        -> Result<Triangulation, Error>
        where E: IntoIterator<Item=&'a (usize, usize)> + Copy
    {
        let mut t = Self::new_with_edges(points, edges)?;
        t.run()?;
        Ok(t)
    }

    /// Builds a complete triangulation from the given points and contours
    /// (which are represented as indexes into the points array).
    ///
    /// # Errors
    /// This may return [`Error::EmptyInput`], [`Error::InvalidInput`],
    /// [`Error::InvalidEdge`], [`Error::OpenContour`] or
    /// [`Error::CannotInitialize`] if the input is invalid.
    pub fn build_from_contours<V>(points: &[Point], contours: &[V])
        -> Result<Triangulation, Error>
        where for<'b> &'b V: IntoIterator<Item=&'b usize>
    {
        let mut t = Self::new_from_contours(points, contours)?;
        t.run()?;
        Ok(t)
    }

    fn validate_input<'a, E>(points: &[Point], edges: E)
        -> Result<(), Error>
        where E: IntoIterator<Item=&'a (usize, usize)> + Copy
    {
        if points.is_empty() {
            Err(Error::EmptyInput)
        } else if points.iter().any(|p| p.0.is_nan() || p.0.is_infinite() ||
                                        p.1.is_nan() || p.1.is_infinite()) {
            Err(Error::InvalidInput)
        } else if edges.into_iter().any(|e| e.0 >= points.len() ||
                                            e.1 >= points.len() ||
                                            e.0 == e.1) {
            Err(Error::InvalidEdge)
        } else if points.len() < 3 {
            Err(Error::TooFewPoints)
        } else {
            Ok(())
        }
    }

    /// Constructs a new triangulation of the given points.  The points are a
    /// flat array of positions in 2D spaces; edges are undirected and expressed
    /// as indexes into the `points` list.
    ///
    /// The triangulation is not actually run in this constructor; use
    /// [`Triangulation::step`] or [`Triangulation::run`] to triangulate,
    /// or [`Triangulation::build_with_edges`] to get a complete triangulation
    /// right away.
    ///
    /// # Errors
    /// This may return [`Error::EmptyInput`], [`Error::InvalidInput`],
    /// [`Error::InvalidEdge`], or [`Error::CannotInitialize`] if the input is
    /// invalid.
    pub fn new_with_edges<'a, E>(points: &[Point], edges: E)
        -> Result<Triangulation, Error>
        where E: IntoIterator<Item=&'a (usize, usize)> + Copy
    {
        Self::validate_input(points, edges)?;

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
        // relative to the current center.  We leave distance unpopulated
        // because it's calculated at the beginning of the loop below.
        let mut scratch: Vec<(usize, f64)> = (0..points.len())
            .map(|j| (j, distance2(center, points[j])))
            .collect();

        // Find the three closest points
        let arr = min3(&scratch, &points);

        // Pick out the triangle points, ensuring that they're clockwise
        let pa = arr[0];
        let mut pb = arr[1];
        let mut pc = arr[2];
        if orient2d(points[pa], points[pb], points[pc]) < 0.0 {
            std::mem::swap(&mut pb, &mut pc);
        }

        // Pick this triangle's centroid as our starting point
        center = centroid(points[pa], points[pb], points[pc]);

        // Sort with a special comparison function that puts the first
        // three keys at the start of the list, and uses partial_cmp
        // otherwise.  The order of the first three keys is not
        // guaranteed, which we fix up below.
        scratch.sort_unstable_by(|k, r|
            if k.0 == pa || k.0 == pb || k.0 == pc {
                std::cmp::Ordering::Less
            } else if r.0 == pa || r.0 == pb || r.0 == pc {
                std::cmp::Ordering::Greater
            } else {
                // Compare by radius first, then break ties with pseudoangle
                // This should be reproducible, i.e. two identical points should
                // end up next to each other in the list, although with
                // floating-point values, you _never know_.
                match k.1.partial_cmp(&r.1).unwrap() {
                    std::cmp::Ordering::Equal => {
                        let pk = points[k.0];
                        let pr = points[r.0];
                        let ak = pseudo_angle((pk.0 - center.0, pk.1 - center.1));
                        let ar = pseudo_angle((pr.0 - center.0, pr.1 - center.1));
                        ak.partial_cmp(&ar).unwrap()
                    },
                    e => e,
                }
            });

        // Sanity-check that our three target points are at the head of the
        // list, as expected.
        assert!((scratch[0].0 == pa) as u8 +
                (scratch[1].0 == pa) as u8 +
                (scratch[2].0 == pa) as u8 == 1);
        assert!((scratch[0].0 == pb) as u8 +
                (scratch[1].0 == pb) as u8 +
                (scratch[2].0 == pb) as u8 == 1);
        assert!((scratch[0].0 == pc) as u8 +
                (scratch[1].0 == pc) as u8 +
                (scratch[2].0 == pc) as u8 == 1);

        // Apply sorting to initial three points, ignoring distance
        // values at this point because they're unused.
        scratch[0].0 = pa;
        scratch[1].0 = pb;
        scratch[2].0 = pc;

        // These are the points used in the Triangulation struct
        let mut sorted_points = PointVec::with_capacity(points.len());

        // usize in original array -> PointIndex in sorted array
        let mut map_forward = vec![PointIndex::empty(); points.len()];

        // PointIndex in sorted array -> usize in original array
        let mut map_reverse = PointVec::with_capacity(points.len());

        for i in 0..scratch.len() {
            // The first three points are guaranteed to be unique by the
            // min3 selection function, so they have no dupe
            let mut dupe = None;
            let p = scratch[i];
            if i >= 3 {
                // Check each point against its nearest neighbor and the
                // three original points, since they could be duplicates
                // and may not be adjacent
                for j in &[i - 1, 0, 1, 2] {
                    let pa = points[scratch[*j].0];
                    let pb = points[p.0];
                    if (pa.0 - pb.0).abs() < f64::EPSILON &&
                       (pa.1 - pb.1).abs() < f64::EPSILON
                    {
                        dupe = Some(scratch[*j].0);
                        break;
                    }
                }
            };
            map_forward[p.0] = match dupe {
                None => {
                    sorted_points.push(points[p.0]);
                    map_reverse.push(p.0)
                },
                Some(d) => {
                    assert!(map_forward[d] != PointIndex::empty());
                    map_forward[d]
                },
            };
        }

        ////////////////////////////////////////////////////////////////////////
        let has_edges = edges.into_iter().count() > 0;
        let mut out = Triangulation {
            hull: Hull::new(sorted_points.len(), has_edges),
            half: Half::new(sorted_points.len()),
            constrained: has_edges,

            remap: map_reverse,
            next: PointIndex::new(0),
            angles: PointVec::of(sorted_points.iter()
                .map(|p| pseudo_angle((p.0 - center.0, p.1 - center.1)))
                .collect()),

            // Endings are assigned later
            endings: PointVec::of(vec![(0,0); sorted_points.len()]),
            ending_data: vec![],

            points: sorted_points, // moved out here
        };

        let pa = out.next;
        let pb = out.next + 1;
        let pc = out.next + 2;
        out.next += 3;
        let e_ab = out.half.insert(pa, pb, pc,
                                   EMPTY_EDGE, EMPTY_EDGE, EMPTY_EDGE);
        assert!(e_ab == EdgeIndex::new(0));
        let e_bc = out.half.next(e_ab);
        let e_ca = out.half.prev(e_ab);

        /*
         *              a
         *             / ^
         *            /   \
         *           V  f  \
         *          b-------> c
         */
        out.hull.initialize(pa, out.angles[pa], e_ca);
        out.hull.insert_bare(out.angles[pb], pb, e_ab);
        out.hull.insert_bare(out.angles[pc], pc, e_bc);

        ////////////////////////////////////////////////////////////////////////
        // Iterate over edges, counting which points have a termination
        let mut termination_count = PointVec::of(vec![0; out.points.len()]);
        let edge_iter = || edges
            .into_iter()
            .map(|&(src, dst)| {
                let src = map_forward[src];
                let dst = map_forward[dst];
                assert!(src != PointIndex::empty());
                assert!(dst != PointIndex::empty());

                if src > dst { (dst, src) } else { (src, dst) }
            });
        for (src, dst) in edge_iter() {
            // Lock any edges that appear in the seed triangle.  Because the
            // (src, dst) tuple is sorted, there are only three possible
            // matches here.
            if (src, dst) == (pa, pb) {
                out.half.toggle_lock_sign(e_ab);
            } else if (src, dst) == (pa, pc) {
                out.half.toggle_lock_sign(e_ca);
            } else if (src, dst) == (pb, pc) {
                out.half.toggle_lock_sign(e_bc);
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
        Ok(out)
    }

    /// Constructs a new unconstrained triangulation
    ///
    /// The triangulation is not actually run in this constructor; use
    /// [`Triangulation::step`] or [`Triangulation::run`] to triangulate,
    /// or [`Triangulation::build`] to get a complete triangulation right away.
    ///
    /// # Errors
    /// This may return [`Error::EmptyInput`], [`Error::InvalidInput`], or
    /// [`Error::CannotInitialize`] if the input is invalid.
    pub fn new(points: &[Point]) -> Result<Triangulation, Error> {
        let edges: [(usize, usize); 0] = [];
        Self::new_with_edges(points, &edges)
    }

    /// Triangulates a set of contours, given as indexed paths into the point
    /// list.  Each contour must be closed (i.e. the last point in the contour
    /// must equal the first point), otherwise [`Error::OpenContour`] will be
    /// returned.
    ///
    /// The triangulation is not actually run in this constructor; use
    /// [`Triangulation::step`] or [`Triangulation::run`] to triangulate,
    /// or [`Triangulation::build_from_contours`] to get a complete
    /// triangulation right away.
    ///
    /// # Errors
    /// This may return [`Error::EmptyInput`], [`Error::InvalidInput`],
    /// [`Error::InvalidEdge`], [`Error::OpenContour`] or
    /// [`Error::CannotInitialize`] if the input is invalid.
    pub fn new_from_contours<'a, V>(pts: &[Point], contours: &[V])
        -> Result<Triangulation, Error>
        where for<'b> &'b V: IntoIterator<Item=&'b usize>
    {
        let mut edges = Vec::new();
        for c in contours {
            let next = edges.len();
            for (a, b) in c.into_iter().zip(c.into_iter().skip(1)) {
                edges.push((*a, *b));
            }
            if let Some(start) = edges.get(next) {
                if start.0 != edges.last().unwrap().1 {
                    return Err(Error::OpenContour);
                }
            }
        }
        Self::new_with_edges(&pts, &edges)
    }

    /// Runs the triangulation algorithm until completion
    ///
    /// # Errors
    /// This may return [`Error::PointOnFixedEdge`], [`Error::NoMorePoints`],
    /// or [`Error::CrossingFixedEdge`] if those error conditions are met.
    pub fn run(&mut self) -> Result<(), Error> {
        while !self.done() {
            self.step()?;
        }
        Ok(())
    }

    pub(crate) fn orient2d(&self, pa: PointIndex, pb: PointIndex, pc: PointIndex) -> f64 {
        orient2d(self.points[pa], self.points[pb], self.points[pc])
    }

    fn acute(&self, pa: PointIndex, pb: PointIndex, pc: PointIndex) -> f64 {
        acute(self.points[pa], self.points[pb], self.points[pc])
    }

    /// Checks whether the triangulation is done
    pub fn done(&self) -> bool {
        self.next == self.points.len() + 1
    }

    /// Walks the upper hull, making it convex.
    /// This should only be called once from `finalize()`.
    fn make_outer_hull_convex(&mut self) {
        // Walk the hull from left to right, flattening any convex regions
        assert!(self.next == self.points.len());
        let mut start = self.hull.start();
        let mut hl = start;
        let mut hr = self.hull.right_hull(hl);
        loop {
            /*
                ^
                 \
                  \el/hl
                   \
                    <-------------
                        er/hr
            */
            let el = self.hull.edge(hl);
            let er = self.hull.edge(hr);

            let edge_l = self.half.edge(el);
            let edge_r = self.half.edge(er);
            assert!(edge_r.dst == edge_l.src);

            // If this triangle on the hull is strictly convex, fill it
            if self.orient2d(edge_l.dst, edge_l.src, edge_r.src) > 0.0 {
                self.hull.erase(hr);
                let new_edge = self.half.insert(
                    edge_r.src, edge_l.dst, edge_l.src,
                    el, er, EMPTY_EDGE);
                self.hull.update(hl, new_edge);
                self.legalize(self.half.next(new_edge));
                self.legalize(self.half.prev(new_edge));

                // Try stepping back in case this reveals another convex tri
                hr = hl;
                hl = self.hull.left_hull(hl);

                // Record this as the start of the convex region
                start = hl;
            } else {
                // Continue walking along the hull
                let next = self.hull.right_hull(hr);
                hl = hr;
                hr = next;
                if hl == start {
                    break;
                }
            }
        }
    }

    /// Finalizes the triangulation by making the outer hull convex (in the case
    /// of unconstrained triangulation), or removing unattached triangles (for
    /// CDT).
    fn finalize(&mut self) {
        assert!(self.next == self.points.len());

        if self.constrained {
            // For a constrained triangulation, flood fill and erase triangles
            // that are outside the shape boundaries.
            let h = self.hull.start();
            let e = self.hull.edge(h);
            self.half.flood_erase_from(e);
        } else {
            // For an unconstrained triangulation, make the outer hull convex
            self.make_outer_hull_convex();
        }

        self.next += 1usize;
    }

    /// Checks that invariants of the algorithm are maintained. This is a slow
    /// operation and should only be used for debugging.
    ///
    /// # Panics
    /// Panics if invariants are not correct
    pub fn check(&self) {
        self.hull.check();
        self.half.check();
    }

    /// Advances the triangulation by one step.
    ///
    /// # Errors
    /// This may return [`Error::PointOnFixedEdge`], [`Error::NoMorePoints`],
    /// or [`Error::CrossingFixedEdge`] if those error conditions are met.
    pub fn step(&mut self) -> Result<(), Error> {
        if self.done() {
            return Err(Error::NoMorePoints);
        } else if self.next == self.points.len() {
            self.finalize();
            return Ok(());
        }

        // Pick the next point in our pre-sorted array
        let p = self.next;
        self.next += 1usize;

        // Find the hull edge which will be split by this point
        let h_ab = self.hull.get(self.angles[p]);
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
        assert!(edge.next != EMPTY_EDGE);
        assert!(edge.prev != EMPTY_EDGE);
        assert!(edge.buddy == EMPTY_EDGE);

        assert!(a != b);
        assert!(a != p);
        assert!(b != p);

        let o = self.orient2d(b, a, p);
        let h_p = if o <= 0.0 {
            /*
                    b<-------p<------a
                     \      ^|      ^
                      \      |     /
                  next \     |    / prev
                        \    |   /
                         \   |  /
                          \  |v/
                           V |/
                            c

                Special case: if p is exactly on the line (or inside), then we
                split the line instead of inserting a new triangle.
            */
            if edge.fixed() {
                // TODO: this should only be checked if o == 0.0; otherwise,
                // we should re-insert a-b with a-b-p being a third triangle
                return Err(Error::PointOnFixedEdge(self.remap[p]));
            }
            assert!(edge.buddy == EMPTY_EDGE);
            let edge_bc = self.half.edge(edge.next);
            let edge_ca = self.half.edge(edge.prev);
            let c = edge_bc.dst;
            assert!(c == edge_ca.src);

            let hull_right = self.hull.right_hull(h_ab);
            let hull_left = self.hull.left_hull(h_ab);

            self.half.erase(e_ab);

            let e_pc = self.half.insert(p, c, a, edge_ca.buddy, EMPTY_EDGE, EMPTY_EDGE);
            let e_cp = self.half.insert(c, p, b, EMPTY_EDGE, edge_bc.buddy, e_pc);

            // Update the hull point at b to point to the new split edge
            self.hull.update(h_ab, self.half.next(e_cp));

            // Split the edge in the hull
            let h_ap = self.hull.insert(
                h_ab, self.angles[p], p, self.half.prev(e_pc));

            // If either of the other triangle edges (in the now-deleted
            // triangle) were attached to the hull, then patch them up.
            if self.hull.edge(hull_right) == edge.prev {
                self.hull.update(hull_right, self.half.next(e_pc));
            }
            if self.hull.edge(hull_left) == edge.next {
                self.hull.update(hull_left, self.half.prev(e_cp));
            }

            self.legalize(self.half.prev(e_cp));
            self.legalize(self.half.next(e_pc));
            h_ap
        } else {
            let f = self.half.insert(b, a, p, EMPTY_EDGE, EMPTY_EDGE, e_ab);
            assert!(o != 0.0);
            assert!(o > 0.0);

            // Replaces the previous item in the hull
            self.hull.update(h_ab, self.half.prev(f));

            let h_p = if self.angles[a] != self.angles[p] {
                // Insert the new edge into the hull, using the previous
                // HullIndex as a hint to avoid searching for its position.
                let h_ap = self.hull.insert(
                    h_ab, self.angles[p], p, self.half.next(f));
                self.legalize(f);
                h_ap
            } else {
                /*  Rare case when p and a are in a perfect vertical line:
                 *
                 *  We already inserted the left triangle and attached p-b to
                 *  the hull index.  We insert a bonus right triangle here and
                 *  attach c-p to to p's hull index, rather than splitting a-b
                 *  in the hull.
                 *
                 *                 /p [new point]
                 *               /  | ^
                 *             /    |   \
                 *           V  f   |  g  \
                 *          -------->------>\
                 *          b<------a<------c [previous hull edge]
                 *              e
                 */
                let h_ca = self.hull.right_hull(h_ab);
                let e_ca = self.hull.edge(h_ca);
                let edge_ca = self.half.edge(e_ca);
                assert!(a == edge_ca.dst);
                let c = edge_ca.src;
                let g = self.half.insert(a, c, p,
                    EMPTY_EDGE, self.half.next(f), e_ca);

                // h_ca has the same X position as c-p, so we update the same
                // slot in the hull, then move the point in the look-up table.
                self.hull.update(h_ca, self.half.next(g));
                self.hull.move_point(a, p);

                // Legalize the two new triangle edges
                self.legalize(f);
                self.legalize(g);
                h_ca
            };

            // Check and fill acute angles
            self.check_acute_left(p, h_p);
            self.check_acute_right(p, h_p);
            h_p
        };

        // Finally, we check whether this point terminates any edges that are
        // locked in the triangulation (the "constrainted" part of Constrained
        // Delaunay Triangulation).
        let (start, end) = self.endings[p];
        for i in start..end {
            self.handle_fixed_edge(h_p, p, self.ending_data[i])?;
        }

        Ok(())
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

            // Pick out the next item in the list
            let h_q = self.hull.left_hull(h_b);
            let e_bq = self.hull.edge(h_q);
            let edge_bq = self.half.edge(e_bq);
            let q = edge_bq.dst;

            // If we're building a constrained triangulation, then we force the
            // outer hull to be convex, so each point-to-point connection
            // is guaranteed to stay within the triangulation.  This is slightly
            // less efficient than the acute check, but dramatically simplifies
            // the code for fixing edges.
            //
            // For unconstrained triangulations, we check that the inner angle
            // is less that pi/2, per Zalik '05.
            if (!self.constrained && self.acute(p, b, q) <= 0.0) ||
                self.orient2d(p, b, q) >= 0.0
            {
                break;
            }

            // Friendship ended with q-b-p
            self.hull.erase(h_b);

            // Now p-q is my new friend
            let e_pq = self.half.insert(p, q, b, e_bq, e_pb, EMPTY_EDGE);
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
            assert!(a != p);

            // Scoot over by one to look at the a-q edge
            h_a = self.hull.right_hull(h_a);
            let e_qa = self.hull.edge(h_a);
            let edge_qa = self.half.edge(e_qa);
            let q = edge_qa.src;

            // Same check as above
            if (!self.constrained && self.acute(p, a, q) <= 0.0)  ||
                self.orient2d(p, a, q) <= 0.0
            {
                break;
            }

            self.hull.erase(h_a);
            let edge_qp = self.half.insert(q, p, a, e_ap, e_qa, EMPTY_EDGE);
            self.hull.update(h_p, edge_qp);
            h_a = h_p;

            // Then legalize from the two new triangle edges (bp and qb)
            self.legalize(self.half.next(edge_qp));
            self.legalize(self.half.prev(edge_qp));
        }
    }

    /// Finds which mode to begin walking through the triangulation when
    /// inserting a fixed edge.  h is a [`HullIndex`] equivalent to the `src`
    /// point, and `dst` is the destination of the new fixed edge.
    fn find_hull_walk_mode(&self, h: HullIndex, src: PointIndex, dst: PointIndex)
        -> Result<Walk, Error> {
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

            Because the triangulation is convex, we know that this triangle
            exists; we can't escape the triangulation.
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
            return Ok(Walk::Done(e_left));
        } else if dst == wedge_right {
            return Ok(Walk::Done(e_right));
        }

        // Otherwise, check the winding to see which side we're on.
        let o_left = self.orient2d(src, wedge_left, dst);
        let o_right = self.orient2d(src, dst, wedge_right);

        // For now, we don't handle cases where fixed edges have coincident
        // points that are not the start/end of the fixed edge.
        if o_left == 0.0 {
            return Err(Error::PointOnFixedEdge(self.remap[wedge_left]));
        } else if o_right == 0.0 {
            return Err(Error::PointOnFixedEdge(self.remap[wedge_right]));
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
                return Ok(Walk::Done(index_a_src));
            }

            // Keep walking through the fan
            let intersected_index = edge_a_src.prev;

            let o = self.orient2d(src, dst, a);
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
                return Ok(Walk::Inside(intersected_index));
            } else if o < 0.0 {
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
                if buddy == EMPTY_EDGE {
                    return Err(Error::WedgeEscape);
                }
                index_a_src = self.half.edge(buddy).prev;
            } else {
                // If we hit a vertex, exactly, then return an error
                return Err(Error::PointOnFixedEdge(self.remap[a]));
            }
        }
    }

    fn walk_fill(&mut self, src: PointIndex, dst: PointIndex, mut e: EdgeIndex) -> Result<(), Error> {
        let mut steps_left = Contour::new_pos(src, ContourData::None);
        let mut steps_right = Contour::new_neg(src, ContourData::None);

        /*
         * We start inside a triangle, then escape it right away:
                     src
                     / :^
                    / :  \
                 hl/  :   \hr
                  /  :     \
                 V   :  e   \
                b---:------->a
                    :
                   dst
         */
        let edge_ba = self.half.edge(e);
        let e_ac = edge_ba.next;
        let e_cb = edge_ba.prev;
        let edge_ac = self.half.edge(e_ac);
        let edge_cb = self.half.edge(e_cb);

        // Delete this triangle from the triangulation; it will be
        // reconstructed later in a more perfect form.
        self.half.erase(e);

        steps_left.push(self, edge_ba.src,
            if edge_cb.buddy != EMPTY_EDGE {
                ContourData::Buddy(edge_cb.buddy)
            } else {
                let hl = self.hull.index_of(edge_cb.dst);
                assert!(self.hull.edge(hl) == e_cb);
                ContourData::Hull(hl, edge_cb.sign)
            });
        steps_right.push(self, edge_ba.dst,
            if edge_ac.buddy != EMPTY_EDGE {
                ContourData::Buddy(edge_ac.buddy)
            } else {
                let hr = self.hull.index_of(edge_ac.dst);
                assert!(self.hull.edge(hr) == e_ac);
                ContourData::Hull(hr, edge_ac.sign)
            });

        // Exit this triangle, either onto the hull or continuing inside
        // the triangulation.
        if edge_ba.fixed() {
            return Err(Error::CrossingFixedEdge);
        }
        assert!(edge_ba.buddy != EMPTY_EDGE);
        e = edge_ba.buddy;

        loop {
            /*            src
                         :
                   b<--:-------a
                    \ :  e     ^
                     :\      /
                    :   v  /
                   :     c
                  dst
             */
            let edge_ab = self.half.edge(e);
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
            self.half.erase(e);

            // Handle the termination case, if c is the destination
            if c == dst {
                // The left (above) contour is either on the hull
                // (if no buddy is present) or inside the triangulation
                let e_dst_src = steps_left.push(self, c,
                    if edge_bc.buddy == EMPTY_EDGE {
                        let h = self.hull.index_of(edge_bc.dst);
                        assert!(self.hull.edge(h) == e_bc);
                        ContourData::Hull(h, edge_bc.sign)
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
                let e_src_dst = steps_right.push(self, c,
                    if edge_ca.buddy == EMPTY_EDGE {
                        let h = self.hull.index_of(edge_ca.dst);
                        assert!(self.hull.edge(h) == e_ca);
                        ContourData::Hull(h, edge_ca.sign)
                    } else {
                        ContourData::Buddy(edge_ca.buddy)
                    })
                    .expect("Failed to create second fixed edge");

                // Similarly, this better have terminated the
                // triangulation of the lower contour.
                assert!(self.half.edge(e_src_dst).src == src);
                assert!(self.half.edge(e_src_dst).dst == dst);

                self.half.link(e_src_dst, e_dst_src);
                self.half.toggle_lock_sign(e_src_dst); // locks both sides

                break;
            }

            let o_psc = self.orient2d(src, dst, c);
            e = if o_psc > 0.0 {
                // Store the c-a edge as our buddy, and exit via b-c
                // (unless c-a is the 0th edge, which has no buddy)
                steps_right.push(self, c,
                    if edge_ca.buddy == EMPTY_EDGE {
                        let h = self.hull.index_of(edge_ca.dst);
                        assert!(self.hull.edge(h) == e_ca);
                        ContourData::Hull(h, edge_ca.sign)
                    } else {
                        ContourData::Buddy(edge_ca.buddy)
                    });

                // Exit the triangle, either onto the hull or staying
                // in the triangulation
                if edge_bc.fixed() {
                    return Err(Error::CrossingFixedEdge);
                }
                assert!(edge_bc.buddy != EMPTY_EDGE);
                edge_bc.buddy
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
                //
                // (c-b may be a hull edge, so we check for that)
                steps_left.push(self, c,
                    if edge_bc.buddy == EMPTY_EDGE {
                        let h = self.hull.index_of(edge_bc.dst);
                        assert!(self.hull.edge(h) == e_bc);
                        ContourData::Hull(h, edge_bc.sign)
                    } else {
                        ContourData::Buddy(edge_bc.buddy)
                    });

                if edge_ca.fixed() {
                    return Err(Error::CrossingFixedEdge);
                }
                assert!(edge_ca.buddy != EMPTY_EDGE);
                edge_ca.buddy
            } else {
                return Err(Error::PointOnFixedEdge(self.remap[c]));
            }
        }
        Ok(())
    }

    fn handle_fixed_edge(&mut self, h: HullIndex, src: PointIndex, dst: PointIndex) -> Result<(), Error> {
        match self.find_hull_walk_mode(h, src, dst)? {
            // Easy mode: the fixed edge is directly connected to the new
            // point, so we lock it and return immediately.
            Walk::Done(e) => { self.half.toggle_lock_sign(e); Ok(()) },

            // Otherwise, we're guaranteed to be inside the triangulation,
            // because the hull is convex by construction.
            Walk::Inside(e) => self.walk_fill(src, dst, e),
        }
    }

    pub(crate) fn legalize(&mut self, e_ab: EdgeIndex) {
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
        if edge.fixed() || edge.buddy == EMPTY_EDGE {
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

    /// Calculates a bounding box, returning `((xmin, xmax), (ymin, ymax))`
    pub(crate) fn bbox(points: &[Point]) -> ((f64, f64), (f64, f64)) {
        let (mut xmin, mut xmax) = (std::f64::INFINITY, -std::f64::INFINITY);
        let (mut ymin, mut ymax) = (std::f64::INFINITY, -std::f64::INFINITY);
        for (px, py) in points.iter() {
            xmin = px.min(xmin);
            ymin = py.min(ymin);
            xmax = px.max(xmax);
            ymax = py.max(ymax);
        }
        ((xmin, xmax), (ymin, ymax))
    }

    /// Returns all of the resulting triangles, as indexes into the original
    /// `points` array from the constructor.
    pub fn triangles(&self) -> impl Iterator<Item=(usize, usize, usize)> + '_ {
        self.half.iter_triangles()
            .map(move |(a, b, c)|
                (self.remap[a], self.remap[b], self.remap[c]))
    }

    /// Checks whether the given point is inside or outside the triangulation.
    /// This is extremely inefficient, and should only be used for debugging
    /// or unit tests.
    pub fn inside(&self, p: Point) -> bool {
        self.half.iter_triangles()
            .any(|(a, b, c)| {
                orient2d(self.points[a], self.points[b], p) >= 0.0 &&
                orient2d(self.points[b], self.points[c], p) >= 0.0 &&
                orient2d(self.points[c], self.points[a], p) >= 0.0
            })
    }

    /// Writes the current state of the triangulation to an SVG file,
    /// without debug visualizations.
    pub fn save_svg(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.to_svg(false))
    }

    /// Writes the current state of the triangulation to an SVG file,
    /// including the upper hull as a debugging visualization.
    pub fn save_debug_svg(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.to_svg(true))
    }

    /// Converts the current state of the triangulation to an SVG.  When `debug`
    /// is true, includes the upper hull and to-be-fixed edges; otherwise, just
    /// shows points, triangles, and fixed edges from the half-edge graph.
    pub fn to_svg(&self, debug: bool) -> String {
        let (x_bounds, y_bounds) = Self::bbox(&self.points);
        let scale = 800.0 /
            (x_bounds.1 - x_bounds.0).max(y_bounds.1 - y_bounds.0);
        let line_width = 2.0;
        let dx = |x| { scale * (x - x_bounds.0) + line_width};
        let dy = |y| { scale * (y_bounds.1 - y) + line_width};

         let mut out = String::new();
         // Put a dummy rectangle in the SVG so that rsvg-convert doesn't clip
         out.push_str(&format!(
            r#"<svg viewbox="auto" xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">
    <rect x="0" y="0" width="{}" height="{}"
     style="fill:rgb(0,0,0)" />"#,
            scale * (x_bounds.1 - x_bounds.0) + line_width*2.0,
            scale * (y_bounds.1 - y_bounds.0) + line_width*2.0,
            dx(x_bounds.1) + line_width,
            dy(y_bounds.0) + line_width));

        // Draw endings in green (they will be overdrawn in white if they're
        // included in the triangulation).
        if debug {
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

         if debug {
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
         }

         for p in self.points.iter() {
             out.push_str(&format!(
                r#"
        <circle cx="{}" cy="{}" r="{}" style="fill:rgb(255,128,128)" />"#,
                dx(p.0), dy(p.1), line_width));
        }

        out.push_str("\n</svg>");
        out
    }
}

// Finds the three points in the given buffer with the lowest score, returning
// then in order (so that out[0] is closest)
//
// This is faster than sorting an entire array each time.
fn min3(buf: &[(usize, f64)], points: &[(f64, f64)]) -> [usize; 3] {
    let mut array = [(0, std::f64::INFINITY); 3];
    for &(p, score) in buf.iter() {
        if score < array[0].1 {
            array[0] = (p, score);
        }
    }
    for &(p, score) in buf.iter() {
        if score < array[1].1 {
            // If there is one point picked already, then don't
            // pick it again, since that will be doomed to be colinear.
            let p0 = points[array[0].0];
            if (p0.0 - points[p].0).abs() >= std::f64::EPSILON ||
               (p0.1 - points[p].1).abs() >= std::f64::EPSILON
            {
                array[1] = (p, score);
            }
        }
    }
    for &(p, score) in buf.iter() {
        if score < array[2].1 {
            let p0 = points[array[0].0];
            let p1 = points[array[1].0];
            if orient2d(p0, p1, points[p]).abs() > std::f64::EPSILON {
                array[2] = (p, score);
            }
        }
    }

    let mut out = [0usize; 3];
    for (i, a) in array.iter().enumerate() {
        out[i] = a.0;
    }
    // TODO: return a reasonable error if all inputs are colinear or duplicates
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_triangle() {
        let pts = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let t = Triangulation::build(&pts[0..]).expect("Could not construct");
        assert!(t.inside((0.5, 0.5)));
    }

    #[test]
    fn duplicate_point() {
        let points = vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.1, 1.1),
            (1.1, 1.1),
            (0.0, 1.0),
        ];
        let edges = vec![
            (0, 1),
            (1, 2),
            (3, 4),
            (4, 0),
        ];
        let t = Triangulation::build_with_edges(&points, &edges);
        assert!(!t.is_err());
        assert!(t.unwrap().inside((0.5, 0.5)));
    }

    #[test]
    fn simple_circle() {
        let mut edges = Vec::new();
        let mut points = Vec::new();
        const N: usize = 22;
        for i in 0..N {
            let a = (i as f64) / (N as f64) * core::f64::consts::PI * 2.0;
            let x = a.cos();
            let y = a.sin();
            points.push((x, y));
            edges.push((i, (i + 1) % N));
        }
        let t = Triangulation::build_with_edges(&points, &edges)
            .expect("Could not build triangulation");
        assert!(t.inside((0.0, 0.0)));
        assert!(!t.inside((1.01, 0.0)));
    }

    #[test]
    fn dupe_start() {
        let points = vec![
            // Duplicate nearest points
            (0.5, 0.5),
            (0.5, 0.5),
            (0.5, 0.6),
            (0.6, 0.5),
            (0.5, 0.4),

            // Force the center to be at 0.5, 0.5
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
        ];
        let edges = vec![
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 0),
        ];
        let t = Triangulation::build_with_edges(&points, &edges)
            .expect("Could not build triangulation");
        assert!(t.inside((0.55, 0.5)));
        assert!(!t.inside((0.45, 0.5)));
    }

    #[test]
    fn colinear_start() {
        let points = vec![
            // Force the center to be at 0.5, 0.5
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),

            // Threee colinear points
            (0.5, 0.4),
            (0.5, 0.5),
            (0.5, 0.6),
            (0.6, 0.5),
        ];
        let edges = vec![
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
        ];
        let t = Triangulation::build_with_edges(&points, &edges)
            .expect("Could not build triangulation");
        assert!(t.inside((0.55, 0.5)));
        assert!(!t.inside((0.45, 0.5)));
    }

    #[test]
    fn fuzzy_circle() {
        let mut edges = Vec::new();
        let mut points = Vec::new();
        const N: usize = 32;
        for i in 0..N {
            let a = (i as f64) / (N as f64) * core::f64::consts::PI * 2.0;
            let x = a.cos();
            let y = a.sin();
            points.push((x, y));
            edges.push((i, (i + 1) % N));
        }
        const M: usize = 32;

        use std::iter::repeat_with;
        use rand::{Rng, SeedableRng};
        use itertools::Itertools;

        // Use a ChaCha RNG to be reproducible across platforms
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(12345);
        points.extend(repeat_with(|| rng.gen_range(-1.0..1.0))
            .tuple_windows()
            .filter(|(x, y): &(f64, f64)| (x*x + y*y).sqrt() < 0.95)
            .take(M));

        let t = Triangulation::build_with_edges(&points, &edges)
            .expect("Could not build triangulation");
        assert!(t.inside((0.0, 0.0)));
        assert!(!t.inside((1.01, 0.0)));
    }

    #[test]
    fn spiral_circle() {
        let mut edges = Vec::new();
        let mut points = Vec::new();
        const N: usize = 16;
        for i in 0..N {
            let a = (i as f64) / (N as f64) * core::f64::consts::PI * 2.0;
            let x = a.cos();
            let y = a.sin();
            points.push((x, y));
            edges.push((i, (i + 1) % N));
        }
        const M: usize = 32;
        for i in 0..(2*M) {
            let a = (i as f64) / (M as f64) * core::f64::consts::PI * 2.0;
            let scale = (i as f64 + 1.1).powf(0.2);
            let x = a.cos() / scale;
            let y = a.sin() / scale;
            points.push((x, y));
        }

        let t = Triangulation::build_with_edges(&points, &edges)
            .expect("Could not build triangulation");
        assert!(t.inside((0.0, 0.0)));
        assert!(!t.inside((1.01, 0.0)));
    }

    #[test]
    fn nested_circles() {
        let mut edges = Vec::new();
        let mut points = Vec::new();
        const N: usize = 32;
        for i in 0..N {
            let a = (i as f64) / (N as f64) * core::f64::consts::PI * 2.0;
            let x = a.cos();
            let y = a.sin();
            points.push((x, y));
            edges.push((i, (i + 1) % N));
        }
        for i in 0..N {
            let a = (i as f64) / (N as f64) * core::f64::consts::PI * 2.0;
            let x = a.cos() / 2.0;
            let y = a.sin() / 2.0;
            points.push((x, y));
            edges.push((N + i, N + (i + 1) % N));
        }

        let t = Triangulation::build_with_edges(&points, &edges)
            .expect("Could not build triangulation");
        assert!(!t.inside((0.0, 0.0)));
        assert!(!t.inside((1.01, 0.0)));
        assert!(t.inside((0.75, 0.0)));
        assert!(t.inside((0.0, 0.8)));
    }

    #[test]
    fn grid() {
        let mut points = Vec::new();
        const N: usize = 32;
        for i in 0..N {
            for j in 0..N {
                points.push((i as f64, j as f64));
            }
        }
        let t = Triangulation::build(&points)
            .expect("Could not build triangulation");
        t.check();
    }

    #[test]
    fn grid_with_fixed_circle() {
        let mut edges = Vec::new();
        let mut points = Vec::new();
        const N: usize = 32;
        for i in 0..N {
            let a = (i as f64) / (N as f64) * core::f64::consts::PI * 2.0;
            let x = a.cos() * 0.9;
            let y = a.sin() * 0.9;
            points.push((x, y));
            edges.push((i, (i + 1) % N));
        }
        const M: usize = 32;
        for i in 0..M {
            for j in 0..M {
                points.push((i as f64 / M as f64 * 2.0 - 1.0,
                             j as f64 / M as f64 * 2.0 - 1.0));
            }
        }
        let t = Triangulation::build_with_edges(&points, &edges)
            .expect("Could not build triangulation");
        t.check();
    }

    #[test]
    fn new_from_contours() {
        let t = Triangulation::build_from_contours::<Vec<usize>>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], &vec![]);
        assert!(t.is_ok());

        let t = Triangulation::build_from_contours(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], &[vec![]]);
        assert!(t.is_ok());

        let t = Triangulation::build_from_contours(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], &[vec![0]]);
        assert!(t.is_ok());

        let t = Triangulation::build_from_contours(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], &[vec![0, 1]]);
        assert!(t.is_err());
        if let Err(e) = t {
            assert!(e == Error::OpenContour);
        }
    }
}
