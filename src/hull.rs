use ordered_float::OrderedFloat;

use crate::Point;
use crate::predicates::pseudo_angle;

const HASH_SIZE: usize = 128;

#[derive(Default)]
struct Key {
    key: usize,     // point index
    next: usize,    // index into data, or 0
    prev: usize,    // index into data, or 0
}

struct Hull {
    // hints[i] represents what bucket point[i] should be stored in.  We
    // precalculate this so that we can get an even distribution, even if
    // points are not evenly spaced
    hints: Vec<u8>,

    // angles[i] is a precalculated pseudoangle for point[i].  This is used
    // when deciding who gets into a bucket (smallest value wins, larger
    // value gets pushed into the linked list).
    angles: Vec<f64>,

    buckets: [usize; HASH_SIZE],
    data: Vec<Key>,
}

impl Hull {
    pub fn new(pts: &[Point]) -> Hull {
        // Build a list of [index, pseudoangle]
        let angles: Vec<f64> = pts.iter().map(|&p| pseudo_angle(p)).collect();

        let mut sorted_angles: Vec<usize> = (0..pts.len()).collect();
        sorted_angles.sort_by_key(|i| OrderedFloat(angles[*i]));

        // Each point gets a bucket hint, spaced out so that points are evenly
        // distributed into buckets.
        let chunk_size = HASH_SIZE / pts.len();
        let mut hints = vec![0; pts.len()];
        for (i, chunk) in sorted_angles[0..].chunks(chunk_size).enumerate() {
            for j in chunk {
                hints[*j] = i.max(HASH_SIZE - 1) as u8;
            }
        }

        Hull {
            hints,
            angles,
            buckets: [0; HASH_SIZE],
            data: vec![Key::default()],
        }
    }

    /// Inserts an initial (seed) triangle
    pub fn insert_initial(&mut self, tri: (usize, usize, usize)) {
        assert!(data.len() == 1);

        let a = data.len();

        data.push(Key {
            key: tri[0],
            next: a,
            prev: a,
        });
        self.buckets[self.hints[tri.0]] = 1;

        insert_ll(tri.1);
        insert_ll(tri.2);
    }

    fn insert_ll(&mut self, t: usize) {
        let target = self.hints[t];
        let prev = self.buckets[target];
        if prev != 0 {
            let prev_angle = self.angles[self.data[prev].key];

            if self.angles[t] < prev_angles {
                // Push it out of the bucket if this angle is lower
            } else {
                // Otherwise, attach it to the end of this bucket's chain
                let next = self.data[prev].next;
                while next != prev {
                    if self.angles[t] < self.data[next].angle {
                    }
                }
            }
        }
        data.push(Key {
            key: tri[1],
            next: b,
            prev: c,
        });
        self.buckets[self.hints[tri.1]] = 2;

        data.push(Key {
            key: tri[2],
            next: b,
            prev: c,
        });
        self.buckets[self.hints[tri.2]] = 2;
    }

    /// Inserts a new point into the hull, returning the indices of the edge
    /// which it intersected.
    pub fn insert(&mut self, i: usize) -> (usize, usize) {
    }
}
