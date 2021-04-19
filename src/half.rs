use crate::{PointIndex, PointVec, EdgeIndex, EdgeVec, CHECK_INVARIANTS};

pub const EMPTY: EdgeIndex = EdgeIndex { val: std::usize::MAX };

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub src: PointIndex,
    pub dst: PointIndex,
    pub prev: EdgeIndex,
    pub next: EdgeIndex,
    pub buddy: EdgeIndex, // EMPTY if empty
    pub fixed: bool,
}

/// Half is a half-edge graph structure, implicitly storing triangles.
/// It is agnostic to actual point locations, using abstract PointIndex
/// values instead.
pub struct Half {
    /// The half-edge data structure is stored as a bunch of edges in a flat
    /// array, indexed by the type-safe EdgeIndex key
    edges: EdgeVec<Edge>,

    /// Each point stores an EdgeIndex for which it is the src, or EMPTY.
    /// This lets us do random look-up into the edges list by PointIndex.
    points: PointVec<EdgeIndex>,
}

impl Half {
    pub fn new(num_points: usize) -> Half {
        Half {
            edges: EdgeVec::with_capacity((num_points * 2 - 5) * 3),
            points: PointVec { vec: vec![EMPTY; num_points + 4] },
        }
    }

    /// Locks an edge and its buddy (if present)
    pub fn lock(&mut self, e: EdgeIndex) {
        self.set_lock(e, true);
    }

    pub fn unlock(&mut self, e: EdgeIndex) {
        self.set_lock(e, false);
    }

    fn set_lock(&mut self, e: EdgeIndex, v: bool) {
        self.edges[e].fixed = v;
        let buddy = self.edges[e].buddy;
        if buddy != EMPTY {
            self.edges[buddy].fixed = v;
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

        // Store the index of this edge in the point array
        if self.points[edge.src] == EMPTY {
            self.points[edge.src] = index;
        }

        // Link against a buddy if present, copying its fixed value
        if edge.buddy != EMPTY {
            self.edges[index].fixed = self.edges[edge.buddy].fixed;
            std::mem::swap(&mut self.edges[edge.buddy].buddy, &mut index);
            assert!(index == EMPTY);
        }
    }

    /// Inserts a new triangle into the edge map, based on three points
    /// and optional paired edges.  Returns the new edge index a->b
    pub fn insert(&mut self, a: PointIndex, b: PointIndex, c: PointIndex,
                  e_cb: EdgeIndex, e_ac: EdgeIndex, e_ba: EdgeIndex) -> EdgeIndex
    {
        let i = self.edges.len();
        let e_ab = EdgeIndex::new(i);
        let e_bc = EdgeIndex::new(i + 1);
        let e_ca = EdgeIndex::new(i + 2);
        self.push_edge(Edge {
            src: a, dst: b,
            prev: e_ca, next: e_bc,
            buddy: e_ba,
            fixed: false,
        });
        self.push_edge(Edge {
            src: b, dst: c,
            prev: e_ab, next: e_ca,
            buddy: e_cb,
            fixed: false,
        });
        self.push_edge(Edge {
            src: c, dst: a,
            prev: e_bc, next: e_ab,
            buddy: e_ac,
            fixed: false,
        });

        self.check();
        e_ab
    }

    pub fn iter_edges(&self) -> impl Iterator<Item=(PointIndex, PointIndex, bool)> + '_ {
        return self.edges.iter()
            .filter(|e| e.next != EMPTY)
            .map(|e| (e.src, e.dst, e.fixed))
    }

    pub fn iter_triangles(&self) -> impl Iterator<Item=(PointIndex, PointIndex, PointIndex)> + '_ {
        let mut seen = EdgeVec { vec: vec![false; self.edges.len()] };
        self.edges.iter()
            .filter(|e| e.next != EMPTY)
            .enumerate()
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

    pub fn edge_index(&self, p: PointIndex) -> EdgeIndex {
        self.points[p]
    }

    /// Sanity-checks the structure's invariants, raising an assertion if
    /// any invariants are broken.  This is a no-op if CHECK_INVARIANTS is set
    /// to false in lib.rs.
    pub fn check(&self) {
        if !CHECK_INVARIANTS {
            return;
        }
        for (point, e) in self.points.iter().enumerate() {
            if *e != EMPTY {
                assert!(self.edges[*e].src == point);
            }
        }
        for (index, edge) in self.edges.iter().enumerate() {
            if edge.next == EMPTY {
                assert!(edge.prev == EMPTY);
                assert!(edge.buddy == EMPTY);
                continue;
            }
            // Check that our relationship with our buddy is good
            let index = EdgeIndex::new(index);
            let buddy_index = edge.buddy;
            if buddy_index != EMPTY {
                let buddy = self.edge(buddy_index);
                assert!(edge.src == buddy.dst);
                assert!(edge.dst == buddy.src);
                assert!(buddy.buddy == index);
                assert!(edge.fixed == buddy.fixed);
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
        if self.edges[e_ba].fixed {
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
        assert!(edge.buddy != EMPTY);

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
            fixed: false,
        };
        self.edges[e_ab] = Edge {
            src: d,
            dst: c,
            prev: e_bd,
            next: e_cb,
            buddy: e_ba,
            fixed: false,
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

        // Reassign points which could have been pointing to the swapped edge
        self.points[edge.src] = e_bd;
        self.points[edge.dst] = e_ac;

        self.check();
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
        //
        // The point will be orphaned if this is EMPTY.
        for &e in &[e_ab, e_bc, e_ca] {
            let edge = self.edges[e];
            if self.points[edge.src] == e {
                self.points[edge.src] = self.edges[edge.prev].buddy;
            }
        }

        for &e in &[e_ab, e_bc, e_ca] {
            let buddy = self.edges[e].buddy;
            if buddy != EMPTY {
                self.edges[buddy].buddy = EMPTY;
            }
            self.edges[e].next = EMPTY;
            self.edges[e].prev = EMPTY;
            self.edges[e].buddy = EMPTY;
        }
        // TODO: reuse edges once they're erased
        self.check();
    }

    pub fn link(&mut self, a: EdgeIndex, b: EdgeIndex) {
        assert!(self.edges[a].buddy == EMPTY);
        assert!(self.edges[b].buddy == EMPTY);
        assert!(self.edges[a].fixed == self.edges[b].fixed);
        self.edges[a].buddy = b;
        self.edges[b].buddy = a;
    }
}
