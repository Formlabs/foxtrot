use std::num::NonZeroUsize;
use ordered_float::OrderedFloat;

use crate::{Point, PointIndex, EdgeIndex};
use crate::predicates::pseudo_angle;

const N: usize = 128;
const EMPTY: PointIndex = PointIndex(std::usize::MAX);

#[derive(Clone, Copy, Debug, Default)]
struct Node {
    // This is the points absolute ordering.  It is assigned into a bucket
    // based on this order and the total bucket count
    order: usize,

    edge: EdgeIndex,

    // prev and next refer to traveling counterclockwise around the hull
    prev: PointIndex,
    next: PointIndex,
}

/// The Hull stores a set of points which form a counterclockwise topological
/// circle about the center of the triangulation.
///
/// Each point is associated with an EdgeIndex into a half-edge data structure,
/// but the Hull does not concern itself with such things.
///
/// The Hull supports one kind of lookup: for a point P, find the point Q with
/// the highest psuedo-angle that is below P.  When projecting P towards the
/// triangulation center, it will intersect the edge beginning at Q; this
/// edge is the one which should be split.
#[derive(Debug)]
struct Hull {
    buckets: [PointIndex; N],
    data: Vec<Node>,
}

impl Hull {
    pub fn new(center: Point, pts: &[Point]) -> Hull {
        // Sort points by their pseudo-angle
        let mut scratch: Vec<(usize, f64)> = Vec::with_capacity(pts.len());
        scratch.extend(pts.iter()
            .enumerate()
            .map(|(j, p)|
                (j, pseudo_angle((p.0 - center.0, p.1 - center.1)))));
        scratch.sort_unstable_by_key(|k| OrderedFloat(k.1));

        // Record the ordering of points into the node data array
        let mut data = vec![Node::default(); pts.len()];
        for (j, (i, _)) in scratch.iter().enumerate() {
            data[*i].order = j;
        }

        Hull {
            buckets: [EMPTY; N],
            data: data,
        }
    }

    // Inserts the first point, along with its associated edge
    pub fn insert_first(&mut self, p: PointIndex, e: EdgeIndex) {
        let b = self.bucket(p);
        assert!(self.buckets[b] == EMPTY);
        self.buckets[b] = p;
        println!("Inserted into bucket {}", b);

        // Tie this point into a tiny loop
        self.data[p.0].next = p;
        self.data[p.0].prev = p;

        // Attach the edge index data to this point
        self.data[p.0].edge = e;
    }

    pub fn insert(&mut self, p: PointIndex, e: EdgeIndex) {
        let b = self.bucket(p);
        // If the target bucket is empty, then we should search for the
        // next-highest point, then walk back one step to find the next-lowest
        // point.  This is better than searching for the next-lowest point,
        // which requires finding the next-lowest bucket then walking all
        // the way to the end of that bucket's chain.
        let mut next = self.buckets[b];
        if next == EMPTY {
            // Find the next filled bucket, which must exist somewhere
            let mut t = b;
            while self.buckets[t] == EMPTY {
                t = (t + 1) % N;
            }
            next = self.buckets[t];
            self.buckets[b] = p;
        } else {
            // This bucket is already occupied, so we'll need to walk its
            // linked list until we find the right place to insert.

            // Loop until we find an item in the linked list which is less
            // that our new point, or we leave this bucket; the latter case
            // handles wrapping around.
            while self.data[next.0].order < self.data[p.0].order &&
                  self.bucket(next) == b
            {
                next = self.data[next.0].next;
            }
            // If the new node is the smallest (i.e. the loop didn't advance at
            // all), then it replaces the previous node in the bucket table.
            if next == self.buckets[b] {
                self.buckets[b] = p;
            }
        }

        // Walk backwards one step the list to find the previous node
        let prev = self.data[next.0].prev;

        // Write all of our new node data, leaving order fixed
        self.data[p.0].edge = e;
        self.data[p.0].next = next;
        self.data[p.0].prev = prev;

        // Stitch ourselves into the linked list
        self.data[next.0].prev = p;
        self.data[prev.0].next = p;
    }

    fn bucket(&self, p: PointIndex) -> usize {
        (self.data[p.0].order * self.buckets.len()) / self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circular_hull() {
        let mut pts = Vec::new();
        let num = 1024;
        for i in 0..num {
            let angle = i as f64 * 2.0 * std::f64::consts::PI / (num as f64);
            pts.push((angle.cos(), angle.sin()));
        }
        let mut h = Hull::new((0.0, 0.0), &pts);
        h.insert_first(PointIndex(0), EdgeIndex(NonZeroUsize::new(1).unwrap()));
        h.insert(PointIndex(1), EdgeIndex(NonZeroUsize::new(2).unwrap()));
        h.insert(PointIndex(2), EdgeIndex(NonZeroUsize::new(3).unwrap()));
    }
}
