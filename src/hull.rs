use crate::{PointVec, PointIndex, HullVec, HullIndex, EdgeIndex, half, CHECK_INVARIANTS};

const N: usize = 1 << 10;
pub const EMPTY: HullIndex = HullIndex { val: std::usize::MAX };

#[derive(Clone, Copy, Debug)]
struct Node {
    pos_norm: f64,
    edge: EdgeIndex,

    // prev is leftward, next is rightward
    left: HullIndex,
    right: HullIndex,
}

/// The Hull stores a set of points which form a left-to-right order
///
/// Each point is associated with an EdgeIndex into a half-edge data structure,
/// but the Hull does not concern itself with such things.
///
/// The Hull supports one kind of lookup: for a point P, find the point Q with
/// the highest X value that is below P.  When projecting P towards the
/// sweepline, it will intersect the edge beginning at Q; this edge is the one
/// which should be split.
///
/// In addition, the Hull stores a random-access map from PointIndex to
/// HullIndex (if present), for fast lookups without hash traversal.
#[derive(Debug)]
pub struct Hull {
    buckets: [HullIndex; N],
    data: HullVec<Node>,
    points: PointVec<HullIndex>,
    empty: Vec<HullIndex>, // spare slots

    xmin: f64,
    dx: f64,
}

impl Hull {
    pub fn new(num_points: usize, xmin: f64, xmax: f64) -> Hull {
        Hull {
            data: HullVec::new(),
            buckets: [EMPTY; N],
            points: PointVec { vec: vec![EMPTY; num_points] },
            empty: Vec::new(),
            dx: xmax - xmin,
            xmin,
        }
    }

    // Inserts the first point, along with its associated edge
    pub fn insert_lower_edge(&mut self, p: PointIndex, edge: EdgeIndex) -> HullIndex {
        self.buckets[0] = self.data.push( Node {
            pos_norm: 0.0,
            edge,
            left: EMPTY,
            right: HullIndex::new(1),
        });
        self.buckets[self.buckets.len() - 1] = self.data.push(Node {
            pos_norm: 1.0,
            edge: half::EMPTY,
            left: HullIndex::new(0),
            right: EMPTY,
        });
        self.points[p] = self.buckets[0];

        self.check();

        self.buckets[0]
    }

    pub fn update(&mut self, h: HullIndex, e: EdgeIndex) {
        self.data[h].edge = e;
    }

    /// For a given point, returns the HullIndex which will be split when this
    /// point is inserted.  Use `Hull::edge` to get the associated EdgeIndex.
    pub fn get(&self, p: f64) -> HullIndex {
        let p = self.normalize_pos(p);
        let b = self.bucket(p);

        // If the target bucket is empty, then we should search for the
        // next-highest point, then walk back one step to find the next-lowest
        // point.  This is better than searching for the next-lowest point,
        // which requires finding the next-lowest bucket then walking all
        // the way to the end of that bucket's chain.
        let mut pos = self.buckets[b];
        if pos == EMPTY {
            // Find the next filled bucket, which must exist somewhere
            let mut t = b;
            while self.buckets[t] == EMPTY {
                t = (t + 1) % N;
            }
            pos = self.buckets[t];
        } else {
            // This bucket is already occupied, so we'll need to walk its
            // linked list until we find the right place to insert.

            // Loop until we find an item in the linked list which is less
            // that our new point, or we leave this bucket, or we're about
            // to wrap around in the same bucket.
            while self.data[pos].pos_norm < p &&
                  self.bucket_h(pos) == b
            {
                pos = self.data[pos].right;
            }
        }
        assert!(pos != EMPTY);

        // Return the HullIndex which will be split by this point
        self.data[pos].left
    }

    /// Sanity-checks invariants of the data structure, raising an assertion
    /// failure if an invariant is broken.  This is a no-op if CHECK_INVARIANTS
    /// is set to false in lib.rs.
    pub fn check(&self) {
        if !CHECK_INVARIANTS {
            return;
        }
        // Find the first non-empty bucket to use as our starting point for
        // walking around the hull's linked list.
        let point = self.buckets.iter()
            .filter(|b| **b != EMPTY)
            .copied()
            .next();
        assert!(point.is_some());

        let start = point.unwrap();
        assert!(self.buckets[self.bucket_h(start)] == start);

        let mut index = start;
        // Walk around the hull, checking that position is strictly increasing,
        // edges are correctly stitched together, and buckets are correct.
        loop {
            // Assert that the list is correctly stitched together
            let next = self.data[index].right;
            if next == EMPTY {
                break;
            }
            assert!(index == self.data[next].left);

            // If this is the first item in a new bucket, it should be at the
            // head of the bucket's list.
            let my_bucket = self.bucket_h(index);
            let next_bucket = self.bucket_h(next);
            if next_bucket != my_bucket {
                assert!(self.buckets[next_bucket] == next);
            }

            // Assert that position are increasing in the list
            let my_position = self.data[index].pos_norm;
            let next_position = self.data[next].pos_norm;
            assert!(next_position >= my_position);
            index = next;
        }
        assert!(self.data[index].pos_norm == 1.0);
    }

    pub fn left_hull(&self, h: HullIndex) -> HullIndex {
        self.data[h].left
    }

    pub fn right_hull(&self, h: HullIndex) -> HullIndex {
        self.data[h].right
    }

    pub fn edge(&self, h: HullIndex) -> EdgeIndex {
        self.data[h].edge
    }

    /// Searches left along the hull for a hull point with the associated edge
    pub fn index_of(&self, p: PointIndex) -> HullIndex {
        let h = self.points[p];
        assert!(h != EMPTY);
        assert!(self.data[h].left != EMPTY || self.data[h].right != EMPTY);
        h
    }

    /// Insert a new Point-Edge pair into the hull, using a hint to save time
    /// searching for the new point's position.
    pub fn insert(&mut self, left: HullIndex, p: f64, point: PointIndex, e: EdgeIndex) -> HullIndex {
        let right = self.right_hull(left);

        let p = self.normalize_pos(p);
        let h = if let Some(h) = self.empty.pop() {
            self.data[h] = Node {
                pos_norm: p,
                edge: e,
                left, right
            };
            h
        } else {
            self.data.push(Node{
                pos_norm: p,
                edge: e,
                left, right
            })
        };

        // If the target bucket is empty, or the given point is below the first
        // item in the target bucket, then it becomes the bucket's head
        let b = self.bucket(p);
        if self.buckets[b] == EMPTY || (self.buckets[b] == right &&
            p < self.data[right].pos_norm)
        {
            self.buckets[b] = h;
        }

        // Stitch ourselves into the linked list
        self.data[right].left = h;
        self.data[left].right = h;

        self.points[point] = h;

        self.check();

        h
    }

    /// Removes the given point from the hull
    pub fn erase(&mut self, h: HullIndex) {
        let next = self.data[h].right;
        let prev = self.data[h].left;

        // Cut this node out of the linked list
        self.data[next].left = prev;
        self.data[prev].right = next;
        self.data[h].right = EMPTY;
        self.data[h].left = EMPTY;

        // If this is the head of the bucket, then replace it with the next
        // item in this bucket chain (assuming it belongs in the same bucket),
        // or EMPTY if the bucket is now completely empty.
        let b = self.bucket_h(h);
        if self.buckets[b] == h {
            if self.bucket_h(next) == b {
                self.buckets[b] = next;
            } else {
                self.buckets[b] = EMPTY;
            }
        }

        self.empty.push(h);
        self.check();
    }

    pub fn start(&self) -> HullIndex {
        self.buckets.iter()
            .filter(|b| **b != EMPTY)
            .copied()
            .next()
            .unwrap()
    }

    /// Iterates over all edges stored in the Hull, in order
    pub fn values(&self) -> impl Iterator<Item=EdgeIndex> + '_ {
        // Find the first non-empty bucket to use as our starting point for
        // walking around the hull's linked list.
        let mut point: HullIndex = self.buckets.iter()
            .filter(|b| **b != EMPTY)
            .copied()
            .next()
            .unwrap();
        // Then, walk the linked list until we hit the starting point again,
        // returning the associated edges at each point.
        std::iter::from_fn(move || {
            let out = self.data[point].edge;
            point = self.data[point].right;
            if point == EMPTY {
                None
            } else {
                Some(out)
            }
        })
    }

    fn normalize_pos(&self, p: f64) -> f64 {
        (p - self.xmin) / self.dx
    }

    /// Looks up what bucket a given normalized position will fall into
    /// Note that this doesn't work with point coordinates directly;
    /// call normalize_pos(p) to normalize it.
    fn bucket(&self, f: f64) -> usize {
        assert!(f >= 0.0);
        assert!(f <= 1.0);
        (f * (self.buckets.len() as f64 - 1.0)).round() as usize
    }

    pub fn bucket_h(&self, h: HullIndex) -> usize {
        self.bucket(self.data[h].pos_norm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use rand::seq::SliceRandom;

    #[test]
    fn circular_hull() {
        let mut pts = Vec::new();
        let num = 1_000_000;
        for i in 0..num {
            let angle = i as f64 * 2.0 * std::f64::consts::PI / (num as f64);
            pts.push((angle.cos(), angle.sin()));
        }
        pts.shuffle(&mut rand::thread_rng());

        let mut h = Hull::new((0.0, 0.0), &pts);
        h.insert_first(PointIndex(0), EdgeIndex(NonZeroUsize::new(1).unwrap()));
        for i in 1..num {
            if i % 1000 == 0 {
                eprintln!("{}", i);
            }
            h.insert(PointIndex(i), EdgeIndex(NonZeroUsize::new(2).unwrap()));
        }
    }
}
