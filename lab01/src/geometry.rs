#[derive(Copy, Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl Line2D {
    pub fn intersects(&self, other: Line2D) -> bool {
        let ccw1 = Line2D::ccw(self.start, self.end, other.start);
        let ccw2 = Line2D::ccw(self.start, self.end, other.end);
        let ccw3 = Line2D::ccw(other.start, other.end, self.start);
        let ccw4 = Line2D::ccw(other.start, other.end, self.end);

        if ccw1 == 0.0 && ccw2 == 0.0 {
            // They are colinear -> check whether they overlap
            self.parametric_overlap(&other)
        } else {
            // If line segments straddle each other, they intersect
            ccw1 * ccw2 <= 0.0 && ccw3 * ccw4 <= 0.0
        }
    }

    /// Calculates whether two colinear lines overlap.
    fn parametric_overlap(&self, other: &Line2D) -> bool {
        let direction = Point2D {
            x: self.end.x - self.start.x,
            y: self.end.y - self.start.y,
        };

        let to_param = |p: &Point2D| {
            if direction.x.abs() > f64::EPSILON {
                (p.x - self.start.x) / direction.x
            } else {
                (p.y - self.start.y) / direction.y
            }
        };

        let lambda1 = to_param(&other.start);
        let lambda2 = to_param(&other.end);

        lambda1.min(lambda2) <= 1.0 && lambda1.max(lambda2) >= 0.0
    }

    /// Calculates whether a triangle is clockwise or counterclockwise oriented.
    /// That means whether r is on the left or on the right of a line between p and q.
    ///
    /// <0 if clockwise
    /// =0 if collinear
    /// >0 if counterclockwise
    fn ccw(p: Point2D, q: Point2D, r: Point2D) -> f64 {
        (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x - r.y)
    }
}

#[cfg(test)]
mod tests {
    use super::{Line2D, Point2D};

    #[test]
    fn lines_intersect_in_middle() {
        let horizontal = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 2.0, y: 0.0 },
        };
        let vertical = Line2D {
            start: Point2D { x: 1.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 2.0 },
        };

        assert!(horizontal.intersects(vertical))
    }

    #[test]
    fn lines_touch_at_edge() {
        let horizontal = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 2.0, y: 0.0 },
        };
        let vertical = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 0.0, y: 2.0 },
        };

        assert!(horizontal.intersects(vertical))
    }

    #[test]
    fn lines_dont_intersect() {
        let horizontal = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 0.5 },
        };
        let vertical = Line2D {
            start: Point2D { x: 1.0, y: 2.0 },
            end: Point2D { x: 2.0, y: 1.5 },
        };

        assert!(!horizontal.intersects(vertical))
    }

    #[test]
    fn lines_overlaps() {
        let line1 = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 0.0 },
        };
        let line2 = Line2D {
            start: Point2D { x: 0.5, y: 0.0 },
            end: Point2D { x: 2.0, y: 0.0 },
        };

        assert!(line1.intersects(line2))
    }

    #[test]
    fn lines_dont_overlap() {
        let line1 = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 0.0 },
        };
        let line2 = Line2D {
            start: Point2D { x: 2.0, y: 0.0 },
            end: Point2D { x: 3.0, y: 0.0 },
        };

        assert!(!line1.intersects(line2))
    }

    #[test]
    fn lines_parallel() {
        let line1 = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 0.0 },
        };
        let line2 = Line2D {
            start: Point2D { x: 0.0, y: 1.0 },
            end: Point2D { x: 1.0, y: 1.0 },
        };
        assert!(!line1.intersects(line2))
    }

}
