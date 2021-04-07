use crate::{PointIndex, EdgeIndex, EdgeVec};

pub const EMPTY: EdgeIndex = EdgeIndex { val: std::usize::MAX };

#[derive(Copy, Clone, Debug, Default)]
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
impl Default for Half {
    fn default() -> Self { Half::new(0) }
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

    pub fn edge_mut(&mut self, e: EdgeIndex) -> &mut Edge {
        &mut self.edges[e]
    }

    /// Inserts a new triangle into the edge map, based on three points
    /// and optional paired edges.  Returns the new edge index a->b
    pub fn insert(&mut self, a: PointIndex, b: PointIndex, c: PointIndex,
                  e_bc: EdgeIndex, e_ca: EdgeIndex, e_ab: EdgeIndex) -> EdgeIndex
    {
        let i = self.edges.len();
        let o_ab = EdgeIndex::new(i);
        let o_bc = EdgeIndex::new(i + 1);
        let o_ca = EdgeIndex::new(i + 2);
        self.edges.push(Edge {
            src: a, dst: b,
            prev: o_ca, next: o_bc,
            buddy: e_ab
        });
        self.edges.push(Edge {
            src: b, dst: c,
            prev: o_ab, next: o_ca,
            buddy: e_bc
        });
        self.edges.push(Edge {
            src: c, dst: a,
            prev: o_bc, next: o_ab,
            buddy: e_ca
        });

        self.set_buddy(e_ab, o_ab);
        self.set_buddy(e_bc, o_bc);
        self.set_buddy(e_ca, o_ca);

        o_ab
    }

    fn set_buddy(&mut self, target: EdgeIndex, buddy: EdgeIndex) {
        if target != EMPTY {
            let b = &mut self.edge_mut(target).buddy;
            assert!(*b == EMPTY);
            *b = buddy;
        }
    }

    /// Returns an iterator over the edges in the data structure
    pub fn iter_edges(&self) -> impl Iterator<Item=(PointIndex, PointIndex)> + '_ {
        return self.edges.iter().map(|p| (p.src, p.dst))
    }

    /// Swaps the target edge, which must be have a matched pair.
    /// Returns Ok(()) on success, Err(()) if the edge has no pair.
    pub fn swap(&mut self, e: EdgeIndex) {
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
         *         \ |V /
         *          V||/
         *           b
         */
        let edge = self.edge(e);
        assert!(edge.buddy != EMPTY);

        let e_ac = self.next(e);
        let e_cb = self.prev(e);
        let c = self.edge(e_ac).dst;

        let f = edge.buddy;
        let e_bd = self.next(f);
        let e_da = self.prev(f);
        let d = self.edge(self.next(f)).dst;

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
        *self.edge_mut(e) = Edge {
            src: c,
            dst: d,
            prev: e_ac,
            next: e_da,
            buddy: f,
        };
        *self.edge_mut(f) = Edge {
            src: d,
            dst: c,
            prev: e_cb,
            next: e_bd,
            buddy: e,
        };
    }
}
