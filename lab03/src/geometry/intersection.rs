use crate::geometry::line::Line2D;
use crate::geometry::point::Point2D;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub line1: Line2D,
    pub line2: Line2D,
    pub point: Point2D,
}
