use crate::indexes::{PointIndex, EdgeIndex, EdgeVec, EMPTY_EDGE};

/// Represents a directed edge in a triangle graph.
#[derive(Copy, Clone, Debug)]
pub struct Edge {
    /// Source of the directed edge
    pub src: PointIndex,
    /// Destination of the directed edge
    pub dst: PointIndex,
    /// Previous edge in the triangle
    pub prev: EdgeIndex,
    /// Next edge in the triangle
    pub next: EdgeIndex,
    /// Matched edge in the graph, [`EMPTY_EDGE`] if empty
    pub buddy: EdgeIndex,

    /// Marks whether this edge is fixed in the triangulation
    ///
    /// If it is `None`, then the edge is unfixed.  If it is `Some(_)`, then
    /// the edge is fixed; the flag marks whether the edge transition counts as
    /// an inside-outside transition.  For example, consider this donut with
    /// double horizontal edges (marked with `=`)
    ///
    /// ```text
    ///      /---------\
    ///     /           \
    ///     |  -------  |
    ///     |  |     |  |
    ///     |  |     |==|
    ///     |  |     |  |
    ///     |  -------  |
    ///     \           /
    ///      \---------/
    /// ```
    /// All of the area inside the donut should be filled, so crossing the
    /// doubled edge should not count as an inside-outside transition
    pub sign: Option<bool>,
}

impl Edge {
    pub fn fixed(&self) -> bool {
        self.sign.is_some()
    }
}

/// A half-edge graph structure, implicitly storing triangles.
/// It is agnostic to actual point locations, using abstract [`PointIndex`]
/// values instead.
pub struct Half {
    /// The half-edge data structure is stored as a bunch of edges in a flat
    /// array, indexed by the type-safe [`EdgeIndex`] key
    edges: EdgeVec<Edge>,
}

impl Half {
    pub fn new(num_points: usize) -> Half {
        Half {
            edges: EdgeVec::with_capacity((num_points * 2 - 5) * 3),
        }
    }

    pub fn set_sign(&mut self, e: EdgeIndex, v: Option<bool>) {
        self.edges[e].sign = v;
        let buddy = self.edges[e].buddy;
        if buddy != EMPTY_EDGE {
            self.edges[buddy].sign = self.edges[e].sign;
        }
    }

    /// Toggles the lock sign of an edge and its buddy (if present)
    ///
    /// See discussion of [`Edge::sign`] for why this is signed, rather than
    /// a simple flag.
    pub fn toggle_lock_sign(&mut self, e: EdgeIndex) {
        if let Some(b) = self.edges[e].sign {
            self.edges[e].sign = Some(!b);
        } else {
            self.edges[e].sign = Some(true);
        }
        let buddy = self.edges[e].buddy;
        if buddy != EMPTY_EDGE {
            self.edges[buddy].sign = self.edges[e].sign;
        }
    }

    pub fn next(&self, e: EdgeIndex) -> EdgeIndex {
        self.edges[e].next
    }

    pub fn prev(&self, e: EdgeIndex) -> EdgeIndex {
        self.edges[e].prev
    }

    pub fn edge(&self, e: EdgeIndex) -> Edge {
        self.edges[e]
    }

    fn push_edge(&mut self, edge: Edge) {
        let mut index = self.edges.push(edge);

        // Link against a buddy if present, copying its fixed value
        if edge.buddy != EMPTY_EDGE {
            self.edges[index].sign = self.edges[edge.buddy].sign;
            std::mem::swap(&mut self.edges[edge.buddy].buddy, &mut index);
            assert!(index == EMPTY_EDGE);
        }
    }

    /// Inserts a new triangle into the edge map, based on three points
    /// and optional paired edges.  Returns the new edge index `a → b`
    pub fn insert(&mut self, a: PointIndex, b: PointIndex, c: PointIndex,
                  e_cb: EdgeIndex, e_ac: EdgeIndex, e_ba: EdgeIndex) -> EdgeIndex
    {
        let e_ab = self.edges.next_index();
        let e_bc = e_ab + 1usize;
        let e_ca = e_ab + 2usize;
        self.push_edge(Edge {
            src: a, dst: b,
            prev: e_ca, next: e_bc,
            buddy: e_ba,
            sign: None,
        });
        self.push_edge(Edge {
            src: b, dst: c,
            prev: e_ab, next: e_ca,
            buddy: e_cb,
            sign: None,
        });
        self.push_edge(Edge {
            src: c, dst: a,
            prev: e_bc, next: e_ab,
            buddy: e_ac,
            sign: None,
        });

        e_ab
    }

    pub fn iter_edges(&self) -> impl Iterator<Item=(PointIndex, PointIndex, bool)> + '_ {
        return self.edges.iter()
            .filter(|e| e.next != EMPTY_EDGE)
            .map(|e| (e.src, e.dst, e.fixed()))
    }

    pub fn iter_triangles(&self) -> impl Iterator<Item=(PointIndex, PointIndex, PointIndex)> + '_ {
        let mut seen = EdgeVec::of(vec![false; self.edges.len()]);
        self.edges.iter()
            .enumerate()
            .filter(|(_i, e)| e.next != EMPTY_EDGE)
            .filter_map(move |(index, edge)| {
                let index = EdgeIndex::new(index);
                if seen[index] {
                    None
                } else {
                    seen[index] = true;
                    seen[edge.next] = true;
                    seen[edge.prev] = true;
                    Some((edge.src, edge.dst, self.edges[edge.next].dst))
                }
            })
    }

    /// Performs a flood fill from `e`, which is presumed to be outside the
    /// triangulation.  Every triangle outside the boundary is removed,
    /// using odd-even counting (i.e. we switch from outside to inside every
    /// time we cross a fixed edge).
    pub fn flood_erase_from(&mut self, e: EdgeIndex) {
        assert!(self.edge(e).buddy == EMPTY_EDGE);
        let mut seen = EdgeVec::of(vec![false; self.edges.len()]);
        let mut todo = vec![(e, self.edge(e).fixed())];
        while let Some((e, inside)) = todo.pop() {
            if e == EMPTY_EDGE || seen[e] {
                continue;
            }
            let edge = self.edge(e);

            assert!(!seen[edge.next]);
            assert!(!seen[edge.prev]);
            seen[e] = true;
            seen[edge.next] = true;
            seen[edge.prev] = true;

            let next = self.edge(edge.next);
            let prev = self.edge(edge.prev);
            todo.push((next.buddy, inside ^ (next.sign == Some(true))));
            todo.push((prev.buddy, inside ^ (prev.sign == Some(true))));
            if !inside {
                self.erase(e);
            }
        }
    }

    /// Sanity-checks the structure's invariants, raising an assertion if
    /// any invariants are broken.  This is a slow operation and should only
    /// be run in a debugging context.
    ///
    /// # Panics
    /// Panics if the invariants are broken.
    pub fn check(&self) {
        for (index, edge) in self.edges.iter().enumerate() {
            // Check that deleted edges are fully deleted
            if edge.next == EMPTY_EDGE {
                assert!(edge.prev == EMPTY_EDGE);
                assert!(edge.buddy == EMPTY_EDGE);
                continue;
            }
            // Check that our relationship with our buddy is good
            let index = EdgeIndex::new(index);
            let buddy_index = edge.buddy;
            if buddy_index != EMPTY_EDGE {
                let buddy = self.edge(buddy_index);
                assert!(edge.src == buddy.dst);
                assert!(edge.dst == buddy.src);
                assert!(buddy.buddy == index);
                assert!(edge.fixed() == buddy.fixed());
            }
            let next_index = edge.next;
            let next = self.edge(next_index);
            assert!(next.src == edge.dst);
            assert!(next.prev == index);

            let prev_index = edge.prev;
            let prev = self.edge(prev_index);
            assert!(prev.dst == edge.src);
            assert!(prev.next == index);

            // Check the third point in the triangle
            let far = next.dst;
            assert!(next.dst == far);
            assert!(prev.src == far);
            assert!(next.next == prev_index);
            assert!(prev.prev == next_index);
        }
    }

    /// Swaps the target edge, which must be have a matched pair.
    pub fn swap(&mut self, e_ba: EdgeIndex) {
        // We refuse to swap fixed edges, though the caller may ask for it
        if self.edges[e_ba].fixed() {
            return;
        }
        /* Before:
         *           a
         *          /^|^
         *         / || \
         *        /  ||  \
         *       /   ||   \
         *      V   e||    \
         *     c     ||f   d
         *      \    ||    ^
         *       \   ||   /
         *        \  ||  /
         *         \ || /
         *          V|V/
         *           b
         */
        let edge = self.edge(e_ba);
        assert!(edge.buddy != EMPTY_EDGE);

        let e_ac = self.next(e_ba);
        let e_cb = self.prev(e_ba);
        let c = self.edge(e_ac).dst;

        let e_ab = edge.buddy;

        let e_bd = self.next(e_ab);
        let d = self.edge(e_bd).dst;
        let e_da = self.prev(e_ab);

        /* After:
         *            a
         *          /  ^
         *         /    \
         *        /      \
         *       /        \
         *      V      e'  \
         *     c----------->\
         *     \<-----------d
         *      \   f'     ^
         *       \        /
         *        \      /
         *         \    /
         *          V  /
         *           b
         */
        self.edges[e_ba] = Edge {
            src: c,
            dst: d,
            prev: e_ac,
            next: e_da,
            buddy: e_ab,
            sign: None,
        };
        self.edges[e_ab] = Edge {
            src: d,
            dst: c,
            prev: e_bd,
            next: e_cb,
            buddy: e_ba,
            sign: None,
        };
        // Repair the other edges in the triangle
        self.edges[e_ac].prev = e_da;
        self.edges[e_ac].next = e_ba;
        self.edges[e_cb].prev = e_ab;
        self.edges[e_cb].next = e_bd;

        self.edges[e_bd].prev = e_cb;
        self.edges[e_bd].next = e_ab;
        self.edges[e_da].prev = e_ba;
        self.edges[e_da].next = e_ac;
    }

    /// Erases a triangle, clearing its neighbors buddies
    pub fn erase(&mut self, e_ab: EdgeIndex) {
        /*            a
         *          /  ^
         *         /    \
         *        /e     \
         *       /        \
         *      V          \
         *     b----------->c
         */
        let e_bc = self.edges[e_ab].next;
        let e_ca = self.edges[e_ab].prev;

        // We're about to delete this triangle.  If its edges were the canonical
        // edge for some point, then swap for the appropriate buddy.

        for &e in &[e_ab, e_bc, e_ca] {
            let buddy = self.edges[e].buddy;
            if buddy != EMPTY_EDGE {
                self.edges[buddy].buddy = EMPTY_EDGE;
            }
            self.edges[e].next = EMPTY_EDGE;
            self.edges[e].prev = EMPTY_EDGE;
            self.edges[e].buddy = EMPTY_EDGE;
        }
        // TODO: reuse edges once they're erased
    }

    /// Links a new edge in the triangulation, copying the value of `fixed`
    /// from the old edge which is its buddy.  This differs from [`Half::link`], which
    /// checks that the fixed-ness matches.
    ///
    /// The `old` and `new` edges must have compatible `src` and `dst` values
    /// and no pre-existing buddies.
    ///
    /// # Panics
    /// Panics if the edges are not compatible or already have buddies.
    pub fn link_new(&mut self, old: EdgeIndex, new: EdgeIndex) {
        self.edges[new].sign = self.edges[old].sign;
        self.link(old, new)
    }

    /// Sets a pair of edges as each others buddies.  They must have compatible
    /// `src`/`dst` values, no pre-existing buddies, and the same value for
    /// `fixed`; otherwise, it will panic.
    ///
    /// # Panics
    /// Panics if the edges are not compatible or already have buddies.
    pub fn link(&mut self, a: EdgeIndex, b: EdgeIndex) {
        assert!(self.edges[a].buddy == EMPTY_EDGE);
        assert!(self.edges[b].buddy == EMPTY_EDGE);
        assert!(self.edges[a].fixed() == self.edges[b].fixed());
        assert!(self.edges[a].src == self.edges[b].dst);
        assert!(self.edges[a].dst == self.edges[b].src);

        self.edges[a].buddy = b;
        self.edges[b].buddy = a;
    }
}
