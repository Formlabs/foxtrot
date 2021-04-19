use crate::{Point, PointIndex, PointVec, EdgeIndex, HullIndex,
    half, half::Half, hull::Hull,
    predicates::orient2d};

safe_index::new! {
    ContourIndex,
    map: ContourVec with iter: ContourIter
}
const EMPTY: ContourIndex = ContourIndex { val: std::usize::MAX };

#[derive(Copy, Clone, Debug)]
pub enum ContourData {
    None,
    Buddy(EdgeIndex),
    Hull(HullIndex),
}

enum Direction { Left, Right }

#[derive(Copy, Clone, Debug)]
struct Node {
    point: PointIndex,
    data: ContourData,
    left: ContourIndex,
    right: ContourIndex,
}

pub struct Contour {
    pts: ContourVec<Node>,
    left: ContourIndex,
    right: ContourIndex,
    index: ContourIndex,
}

/// A contour marks a set of points that form the boundary of a pseudopolygon
/// during fixed edge insertion.  Each point is marked with an optional
/// HullIndex (if the point is on the hull) or EdgeIndex (if the point has
/// a buddy edge); when a point is inserted with that edge as dst, we update
/// the hull or half-edge structure accordingly.
///
/// The contour can be pushed on either side, which affects which direction
/// triangulation will happen.
///
/// Here's an example of a contour
///
/// ```text
///              x^         b1/h1
///             /  \    v1<----------v0
///            /    \  /              ^
///           /      xv                \b0/h0
///          v                          \
///        vl                            vr
/// ```
///
/// Points can be pushed on either the left or right side, and triangulation
/// will happen assuming the side is consistent (which should be the case).
///
/// Triangulation is based on ["Triangulating Monotone Mountains](http://www.ams.sunysb.edu/~jsbm/courses/345/13/triangulating-monotone-mountains.pdf)
impl Contour {
    pub fn new(point: PointIndex, data: ContourData) -> Self {
        let n = Node { point, data, left: EMPTY, right: EMPTY };
        Contour {
            pts: ContourVec { vec: vec![n] },
            left: ContourIndex::new(0),
            right: ContourIndex::new(0),
            index: ContourIndex::new(0),
        }
    }

    pub fn push_left(&mut self, point: PointIndex, data: ContourData) {
        let i = self.pts.push(Node {
            point, data, left: EMPTY, right: self.left
        });
        self.pts[self.left].left = i;
        self.left = i;
    }

    pub fn push_right(&mut self, point: PointIndex, data: ContourData) {
        let i = self.pts.push(Node {
            point, data, left: self.right, right: EMPTY
        });
        self.pts[self.right].right = i;
        self.right = i;
    }

    /// Attempts to clip the ear with tip self.index, advancing self.index
    /// either to the left or the right and returning true if successful.
    ///
    /// ```text
    ///
    ///            c
    ///          /  ^
    ///         /    \
    ///        /      \
    ///       /        \
    ///      V   e_ab   \
    ///     a - - - - - >b  (e_ab is a new edge inserted here)
    /// ```
    fn try_clip(&mut self, dir: Direction, points: &PointVec<Point>, half: &mut Half, hull: &mut Hull) -> bool {
        let c = self.pts[self.index];
        if c.left == EMPTY || c.right == EMPTY {
            return false;
        }
        let a = self.pts[c.left];
        let b = self.pts[c.right];

        // If the ear isn't strictly convex, then return immediately
        if orient2d(points[a.point], points[b.point], points[c.point]) <= 0.0 {
            return false;
        }

        // Insert the new triangle
        let e_ab = half.insert(a.point, b.point, c.point,
                               half::EMPTY, half::EMPTY, half::EMPTY);
        // Link the new triangle with buddies or hull edges
        let edge_ab = half.edge(e_ab);
        let e_ca = edge_ab.prev;
        let e_bc = edge_ab.next;
        match a.data {
            ContourData::None => (),
            ContourData::Hull(h) => hull.update(h, e_ca),
            ContourData::Buddy(b) => half.link(b, e_ca),
        };
        match c.data {
            ContourData::None => (),
            ContourData::Hull(h) => hull.update(h, e_bc),
            ContourData::Buddy(b) => half.link(b, e_bc),
        };

        // Any new triangles that include e_ba need to link it as their buddy
        self.pts[c.left].data = ContourData::Buddy(e_ab);

        // Stitch the removed node out of the list
        self.pts[self.index].left = EMPTY;
        self.pts[self.index].right = EMPTY;

        self.pts[c.left].right = c.right;
        self.pts[c.right].left = c.left;

        self.index = match dir {
            Direction::Left => c.left,
            Direction::Right => c.right,
        };
        true
    }
}
