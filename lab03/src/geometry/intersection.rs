use std::hash::{Hash, Hasher};
use crate::geometry::line::Line2D;
use crate::geometry::point::Point2D;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub line1: Line2D,
    pub line2: Line2D,
    pub point: Point2D,
}

impl Eq for Intersection {}


impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        (self.line1 == other.line1 && self.line2 == other.line2 && self.point == other.point) ||
            (self.line1 == other.line2 && self.line2 == other.line1 && self.point == other.point)
    }
}

impl Hash for Intersection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Order lines in a consistent way before hashing
        if self.line1 < self.line2 {
            self.line1.hash(state);
            self.line2.hash(state);
        } else {
            self.line2.hash(state);
            self.line1.hash(state);
        }
        self.point.hash(state);
    }
}