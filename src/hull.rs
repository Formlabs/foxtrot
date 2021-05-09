use crate::{
    indexes::{PointVec, PointIndex, HullVec, HullIndex, EdgeIndex, EMPTY_HULL},
};

const N: usize = 1 << 10;

#[derive(Clone, Copy, Debug)]
struct Node {
    /// Pseudo-angle of the point
    angle: f64,

    /// `EdgeIndex` of the edge to the right of this point, i.e. having this
    /// point as its `dst` (since the hull is on top of the shape and triangle
    /// are wound counter-clockwise).
    edge: EdgeIndex,

    /// Neighbors, or `EMPTY_HULL`
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

    /// Random-access lookup of point->hull; this is only needed when doing
    /// *constrained* Delaunay triangulation and is empty otherwise
    points: PointVec<HullIndex>,

    /// Spare slots in the [`Hull::data`] array, to keep it small
    empty: Vec<HullIndex>,
}

impl Hull {
    pub fn new(num_points: usize, constrained: bool) -> Hull {
        Hull {
            data: HullVec::new(),
            buckets: [EMPTY_HULL; N],
            points: if constrained {
                PointVec::of(vec![EMPTY_HULL; num_points])
            } else {
                PointVec::new()
            },
            empty: Vec::new(),
        }
    }

    // Inserts the first point, along with its associated edge, tied into
    // a tiny loop with itself
    pub fn initialize(&mut self, p: PointIndex, angle: f64, edge: EdgeIndex) {
        let h = self.data.push(Node {
            angle,
            left: self.data.next_index(),
            right: self.data.next_index(),
            edge,
        });
        if !self.points.is_empty() {
            self.points[p] = h;
        }

        let b = self.bucket(angle);
        assert!(self.buckets[b] == EMPTY_HULL);
        self.buckets[b] = h;
    }

    pub fn update(&mut self, h: HullIndex, e: EdgeIndex) {
        self.data[h].edge = e;
    }

    /// For a given point, returns the HullIndex which will be split when this
    /// point is inserted.  Use `Hull::edge` to get the associated EdgeIndex.
    pub fn get(&self, angle: f64) -> HullIndex {
        let b = self.bucket(angle);

        // If the target bucket is empty, then we should search for the
        // next-highest point, then walk back one step to find the next-lowest
        // point.  This is better than searching for the next-lowest point,
        // which requires finding the next-lowest bucket then walking all
        // the way to the end of that bucket's chain.
        let mut h = self.buckets[b];
        if h == EMPTY_HULL {
            // Find the next filled bucket, which must exist somewhere
            let mut t = b;
            while self.buckets[t] == EMPTY_HULL {
                t = (t + 1) % N;
            }
            h = self.buckets[t];
        } else {
            // This bucket is already occupied, so we'll need to walk its
            // linked list until we find the right place to insert.

            // Loop until we find an item in the linked list which is less
            // that our new point, or we leave this bucket, or we're about
            // to wrap around in the same bucket.
            let start = h;
            while self.data[h].angle < angle && self.bucket_h(h) == b {
                h = self.data[h].right;
                // If we've looped around, it means all points are in the same
                // bucket *and* the new point is larger than all of them.  This
                // means it will be inserted at the end of the bucket, and will
                // link back to the first point in the bucket.
                if h == start {
                    break;
                }
            }
        }
        assert!(h != EMPTY_HULL);

        // Walk backwards one step to return the HullIndex which will be split
        // by this new point being inserted
        self.data[h].left
    }

    pub fn start(&self) -> HullIndex {
        self.buckets.iter()
            .filter(|b| **b != EMPTY_HULL)
            .copied()
            .next()
            .unwrap()
    }

    /// Sanity-checks invariants of the data structure, raising an assertion
    /// failure if an invariant is broken.  This is a slow operation and should
    /// only be run in a debugging context.
    ///
    /// # Panics
    /// Panics if the invariants are broken.
    pub fn check(&self) {
        // Find the first non-empty bucket to use as our starting point for
        // walking around the hull's linked list.
        let point = self.buckets.iter()
            .filter(|b| **b != EMPTY_HULL)
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
            assert!(index == self.data[next].left);

            // If this is the first item in a new bucket, it should be at the
            // head of the bucket's list.
            let my_bucket = self.bucket_h(index);
            let next_bucket = self.bucket_h(next);
            if next_bucket != my_bucket {
                assert!(self.buckets[next_bucket] == next);
            }

            if next == start {
                break;
            } else {
                // Assert that position are increasing in the list
                let my_position = self.data[index].angle;
                let next_position = self.data[next].angle;
                assert!(next_position >= my_position);
                index = next;
            }
        }
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

    /// Returns the hull index associated with the given point
    pub fn index_of(&self, p: PointIndex) -> HullIndex {
        assert!(!self.points.is_empty());
        let h = self.points[p];
        assert!(h != EMPTY_HULL);
        assert!(self.data[h].left != EMPTY_HULL ||
                self.data[h].right != EMPTY_HULL);
        h
    }

    /// Transitions the point -> hull random lookup from `old` to `new`.  This
    /// is required when two points have the exact same pseudoangle.
    pub fn move_point(&mut self, old: PointIndex, new: PointIndex) {
        if !self.points.is_empty() {
            self.points[new] = self.points[old];
            self.points[old] = EMPTY_HULL;
        }
    }

    /// Inserts a point without a hint
    pub fn insert_bare(&mut self, angle: f64, point: PointIndex, e: EdgeIndex)
        -> HullIndex
    {
        self.insert(self.get(angle), angle, point, e)
    }

    /// Insert a new Point-Edge pair into the hull, using a hint to save time
    /// searching for the new point's position.
    pub fn insert(&mut self, left: HullIndex, angle: f64,
                  point: PointIndex, edge: EdgeIndex) -> HullIndex {
        let right = self.right_hull(left);

        let h = if let Some(h) = self.empty.pop() {
            self.data[h] = Node {
                angle, edge, left, right
            };
            h
        } else {
            self.data.push(Node{
                angle, edge, left, right
            })
        };

        // If the target bucket is empty, or the given point is below the first
        // item in the target bucket, then it becomes the bucket's head
        let b = self.bucket(angle);
        if self.buckets[b] == EMPTY_HULL || (self.buckets[b] == right &&
                                             angle < self.data[right].angle)
        {
            self.buckets[b] = h;
        }

        // Stitch ourselves into the linked list
        self.data[right].left = h;
        self.data[left].right = h;

        if !self.points.is_empty() {
            self.points[point] = h;
        }

        h
    }

    /// Removes the given point from the hull
    pub fn erase(&mut self, h: HullIndex) {
        let next = self.data[h].right;
        let prev = self.data[h].left;

        // Cut this node out of the linked list
        self.data[next].left = prev;
        self.data[prev].right = next;
        self.data[h].right = EMPTY_HULL;
        self.data[h].left = EMPTY_HULL;

        // If this is the head of the bucket, then replace it with the next
        // item in this bucket chain (assuming it belongs in the same bucket),
        // or EMPTY_HULL if the bucket is now completely empty.
        let b = self.bucket_h(h);
        if self.buckets[b] == h {
            if self.bucket_h(next) == b {
                self.buckets[b] = next;
            } else {
                self.buckets[b] = EMPTY_HULL;
            }
        }

        // Store this hull index for reuse
        self.empty.push(h);
    }

    /// Iterates over all edges stored in the Hull, in order
    pub fn values(&self) -> impl Iterator<Item=EdgeIndex> + '_ {
        // Find the first non-empty bucket to use as our starting point for
        // walking around the hull's linked list.
        let mut point: HullIndex = self.buckets.iter()
            .filter(|b| **b != EMPTY_HULL)
            .copied()
            .next()
            .unwrap();
        // Then, walk the linked list until we hit the starting point again,
        // returning the associated edges at each point.
        let start = point;
        let mut started = false;
        std::iter::from_fn(move || {
            let out = self.data[point].edge;
            if point == start && started {
                None
            } else {
                point = self.data[point].right;
                started = true;
                Some(out)
            }
        })
    }

    pub fn bucket_h(&self, h: HullIndex) -> usize {
        self.bucket(self.data[h].angle)
    }

    /// Looks up what bucket a given pseudo-angle will fall into.
    pub fn bucket(&self, angle: f64) -> usize {
        (angle * (self.buckets.len() as f64 - 1.0)).round() as usize
    }
}
