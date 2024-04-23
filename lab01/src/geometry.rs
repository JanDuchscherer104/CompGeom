#[derive(Copy, Clone, Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn approx_eq(&self, other: &Self, epsilon: f64) -> bool {
        (self.x - other.x).abs() < epsilon && (other.x - other.y).abs() < epsilon
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl Line2D {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Line2D {
            start: Point2D { x: x1, y: y1 },
            end: Point2D { x: x2, y: y2 },
        }
    }

    pub fn get_length(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn intersects(&self, other: Line2D) -> bool {
        if self.get_length() == 0.0 && other.get_length() == 0.0 {
            let res = self.start.approx_eq(&other.start, 1e-6);
            return res;
        }

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
        (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y)
    }
}

impl PartialEq for Line2D {
    fn eq(&self, other: &Self) -> bool {
        self.start.approx_eq(&other.start, 1e-6) && self.end.approx_eq(&other.end, 1e-6)
    }
}

#[cfg(test)]
mod tests {
    use geo::{Intersects, LineString};
    use geo::line_string;

    use super::{Line2D, Point2D};

    fn intersect_using_external_library(native_line1: Line2D, native_line2: Line2D) -> bool {
        let geo_line1 = transform_to_geo_line(native_line1);
        let geo_line2 = transform_to_geo_line(native_line2);

        geo_line1.intersects(&geo_line2)
    }

    fn transform_to_geo_line(line: Line2D) -> LineString<f64> {
        line_string![(x: line.start.x, y: line.start.y), (x: line.end.x, y: line.end.y)]
    }

    #[test]
    fn lines_intersect_in_middle() {
        let line1 = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 2.0, y: 0.0 },
        };
        let line2 = Line2D {
            start: Point2D { x: 1.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 2.0 },
        };

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn lines_touch_at_edge() {
        let line1 = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 2.0, y: 0.0 },
        };
        let line2 = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 0.0, y: 2.0 },
        };

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn lines_dont_intersect() {
        let line1 = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 0.5 },
        };
        let line2 = Line2D {
            start: Point2D { x: 1.0, y: 2.0 },
            end: Point2D { x: 2.0, y: 1.5 },
        };

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(line1.intersects(line2), intersect_using_external_library(line1, line2))
    }

    #[test]
    fn lines_dont_intersect2() {
        let line1 = Line2D::new(0.0, 0.0, 1.0, 0.0);
        let line2 = Line2D::new(0.0, 1.0, 1.0, 2.0);

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn horizontal_lines_overlap() {
        let line1 = Line2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 0.0 },
        };
        let line2 = Line2D {
            start: Point2D { x: 0.5, y: 0.0 },
            end: Point2D { x: 2.0, y: 0.0 },
        };

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn vertical_lines_overlap() {
        let line1 = Line2D::new(0.0, 0.0, 0.0, 1.0);
        let line2 = Line2D::new(0.0, 0.5, 0.0, 2.0);

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn diagonal_lines_overlap() {
        let line1 = Line2D::new(0.0, 0.0, 1.0, 1.0);
        let line2 = Line2D::new(0.5, 0.5, 2.0, 2.0);

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
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

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2));
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

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn identical_lines() {
        let line1 = Line2D::new(0.0, 0.0, 1.0, 1.0);
        let line2 = Line2D::new(0.0, 0.0, 1.0, 1.0);

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn collinear_lines() {
        let line1 = Line2D::new(0.0, 0.0, 1.0, 1.0);
        let line2 = Line2D::new(1.5, 1.5, 2.5, 2.5);

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn collinear_lines_start_and_end_on_same_point() {
        let line1 = Line2D::new(0.0, 0.0, 1.0, 0.0);
        let line2 = Line2D::new(1.0, 0.0, 2.0, 0.0);

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn lines_with_zero_length_on_same_point() {
        let line1 = Line2D::new(0.0, 0.0, 0.0, 0.0);
        let line2 = Line2D::new(0.0, 0.0, 0.0, 0.0);

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn lines_with_zero_length_on_different_points() {
        let line1 = Line2D::new(0.0, 0.0, 0.0, 0.0);
        let line2 = Line2D::new(1.0, 1.0, 1.0, 1.0);

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

}
