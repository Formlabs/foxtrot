use std::num::NonZeroUsize;
use crate::{PointIndex, EdgeIndex};

#[derive(Copy, Clone, Debug, Default)]
pub struct Edge {
    pub src: PointIndex,
    pub dst: PointIndex,
    prev: EdgeIndex,
    next: EdgeIndex,
    pub buddy: Option<EdgeIndex>,
}

/// Half is a half-edge graph structure, implicitly storing triangles.
/// It is agnostic to actual point locations, using abstract PointIndex
/// values instead.
pub struct Half {
    edges: Vec<Edge>,
}
impl Default for Half {
    fn default() -> Self { Half::new(0) }
}

impl Half {
    pub fn new(max_triangles: usize) -> Half {
        // Store a default edge in slot 0, so that we can use an
        // Option<NonZeroUsize> for EdgeIndex, which has a type-safe None
        // value without using any extra storage space
        let mut edges = Vec::with_capacity(max_triangles * 3);
        edges.push(Edge::default());
        Half { edges }
    }

    pub fn next(&self, e: EdgeIndex) -> EdgeIndex {
        self.edges[e.0.get()].next
    }

    pub fn prev(&self, e: EdgeIndex) -> EdgeIndex {
        self.edges[e.0.get()].prev
    }

    pub fn edge(&self, e: EdgeIndex) -> Edge {
        self.edges[e.0.get()]
    }

    pub fn edge_mut(&mut self, e: EdgeIndex) -> &mut Edge {
        &mut self.edges[e.0.get()]
    }

    /// Inserts a new triangle into the edge map, based on three points
    /// and optional paired edges.  Returns the new edge index a->b
    pub fn insert(&mut self, a: PointIndex, b: PointIndex, c: PointIndex,
                  e_bc: Option<EdgeIndex>, e_ca: Option<EdgeIndex>,
                  e_ab: Option<EdgeIndex>) -> EdgeIndex
    {
        let i = self.edges.len();
        let o_ab = EdgeIndex(NonZeroUsize::new(i).unwrap());
        let o_bc = EdgeIndex(NonZeroUsize::new(i + 1).unwrap());
        let o_ca = EdgeIndex(NonZeroUsize::new(i + 2).unwrap());
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

        o_ab
    }

    /// Returns an iterator over the edges in the data structure
    pub fn iter_edges(&self) -> impl Iterator<Item=(PointIndex, PointIndex)> + '_ {
        return self.edges.iter().map(|p| (p.src, p.dst)).skip(1)
    }

    /// Swaps the target edge, which must be have a matched pair.
    /// Returns Ok(()) on success, Err(()) if the edge has no pair.
    pub fn swap(&mut self, e: EdgeIndex) -> Result<(), ()> {
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
        if edge.buddy.is_none() {
            return Err(());
        }

        let e_ac = self.next(e);
        let e_cb = self.prev(e);
        let c = self.edge(e_ac).dst;

        let f = edge.buddy.unwrap();
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
            buddy: Some(f),
        };
        *self.edge_mut(f) = Edge {
            src: d,
            dst: c,
            prev: e_cb,
            next: e_bd,
            buddy: Some(e),
        };

        Ok(())
    }
}
