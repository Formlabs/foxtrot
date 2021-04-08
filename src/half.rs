use crate::{PointIndex, EdgeIndex, EdgeVec, CHECK_INVARIANTS};

pub const EMPTY: EdgeIndex = EdgeIndex { val: std::usize::MAX };

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub src: PointIndex,
    pub dst: PointIndex,
    prev: EdgeIndex,
    next: EdgeIndex,
    pub buddy: EdgeIndex, // EMPTY if empty
}

/// Half is a half-edge graph structure, implicitly storing triangles.
/// It is agnostic to actual point locations, using abstract PointIndex
/// values instead.
pub struct Half {
    edges: EdgeVec<Edge>,
}

impl Half {
    pub fn new(max_triangles: usize) -> Half {
        Half { edges: EdgeVec::with_capacity(max_triangles * 3) }
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

    /// Inserts a new triangle into the edge map, based on three points
    /// and optional paired edges.  Returns the new edge index a->b
    pub fn insert(&mut self, a: PointIndex, b: PointIndex, c: PointIndex,
                  e_cb: EdgeIndex, e_ac: EdgeIndex, e_ba: EdgeIndex) -> EdgeIndex
    {
        let i = self.edges.len();
        let e_ab = EdgeIndex::new(i);
        let e_bc = EdgeIndex::new(i + 1);
        let e_ca = EdgeIndex::new(i + 2);
        self.edges.push(Edge {
            src: a, dst: b,
            prev: e_ca, next: e_bc,
            buddy: e_ba
        });
        self.edges.push(Edge {
            src: b, dst: c,
            prev: e_ab, next: e_ca,
            buddy: e_cb
        });
        self.edges.push(Edge {
            src: c, dst: a,
            prev: e_bc, next: e_ab,
            buddy: e_ac
        });

        self.set_buddy(e_ba, e_ab);
        self.set_buddy(e_cb, e_bc);
        self.set_buddy(e_ac, e_ca);

        self.check();
        e_ab
    }

    fn set_buddy(&mut self, target: EdgeIndex, mut buddy: EdgeIndex) {
        if target != EMPTY {
            std::mem::swap(&mut self.edges[target].buddy, &mut buddy);
            assert!(buddy == EMPTY);
        }
    }

    /// Returns an iterator over the edges in the data structure
    pub fn iter_edges(&self) -> impl Iterator<Item=(PointIndex, PointIndex)> + '_ {
        return self.edges.iter().map(|p| (p.src, p.dst))
    }

    /// Sanity-checks the structure's invariants, raising an assertion if
    /// any invariants are broken.
    pub fn check(&self) {
        if !CHECK_INVARIANTS {
            return;
        }
        for (index, edge) in self.edges.iter().enumerate() {
            let index = EdgeIndex::new(index);
            // Check that our relationship with our buddy is good
            let buddy_index = edge.buddy;
            if buddy_index != EMPTY {
                let buddy = self.edge(buddy_index);
                assert!(edge.src == buddy.dst);
                assert!(edge.dst == buddy.src);
                assert!(buddy.buddy == index);
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
    /// Returns Ok(()) on success, Err(()) if the edge has no pair.
    pub fn swap(&mut self, e_ba: EdgeIndex) {
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
        };
        self.edges[e_ab] = Edge {
            src: d,
            dst: c,
            prev: e_bd,
            next: e_cb,
            buddy: e_ba,
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

        self.check();
    }
}
