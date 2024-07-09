use std::cmp::{Ordering, PartialEq};
use std::fmt::Display;
use ordered_float::OrderedFloat;
use crate::geometry::intersection::Intersection;
use crate::geometry::point::Point2D;

#[derive(Copy, Clone, Debug, Hash)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl Line2D {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Line2D {
            start: Point2D { x: OrderedFloat::from(x1), y: OrderedFloat::from(y1) },
            end: Point2D { x: OrderedFloat::from(x2), y: OrderedFloat::from(y2) },
        }
    }

    pub fn get_length(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn is_zero_length(&self) -> bool {
        self.start  == self.end
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn contains(&self, point: Point2D) -> bool {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        // Handle the special case of a zero-length line (both dx and dy are zero)
        if dx.abs() <= f64::EPSILON && dy.abs() <= f64::EPSILON {
            return point.x == self.start.x && point.y == self.start.y;
        }

        // Check if the point lies on the line defined by the segment
        let is_on_line = dx == OrderedFloat(0.0) || (point.x - self.start.x) * dy == (point.y - self.start.y) * dx;

        if !is_on_line {
            return false;
        }

        // Check if the point is within the segment bounds
        let within_x_bounds = if dx.abs() > f64::EPSILON {
            (self.start.x <= point.x && point.x <= self.end.x) || (self.end.x <= point.x && point.x <= self.start.x)
        } else {
            true
        };

        let within_y_bounds = if dy.abs() > f64::EPSILON {
            (self.start.y <= point.y && point.y <= self.end.y) || (self.end.y <= point.y && point.y <= self.start.y)
        } else {
            true
        };

        within_x_bounds && within_y_bounds
    }



    /// Finds the intersection point between two line segments.
    /// If overlapping, it will return one of the overlapping endpoints.
    pub fn find_intersection(&self, other: Line2D) -> Option<Intersection> {
        let (x1, y1) = (self.start.x.0, self.start.y.0);
        let (x2, y2) = (self.end.x.0, self.end.y.0);
        let (x3, y3) = (other.start.x.0, other.start.y.0);
        let (x4, y4) = (other.end.x.0, other.end.y.0);

        let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

        // collinear
        if denominator.abs() < f64::EPSILON {
            // Check if lines are identical
            if *self == other {
                return Some(Intersection::IdenticalOverlap { line1: *self, line2: other, overlap: *self })
            }

            // Check if one line is completely inside the other one
            if self.contains(other.start) && self.contains(other.end) {
                return Some(Intersection::ContainedOverlap { line1: *self, line2: other, overlap: other });
            }
            if other.contains(self.start) && other.contains(self.end) {
                return Some(Intersection::ContainedOverlap { line1: *self, line2: other, overlap: *self });
            }

            // Check for partial overlap
            if self.contains(other.start) || self.contains(other.end) || other.contains(self.start) || other.contains(self.end) {
                let overlap_start = if self.contains(other.start) { other.start } else { self.start };
                let overlap_end = if self.contains(other.end) { other.end } else { self.end };
                let overlap = Line2D { start: overlap_start, end: overlap_end };
                return Some(Intersection::PartialOverlap { line1: *self, line2: other, overlap });
            }
            return None
        }

        let ua_numerator = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
        let ub_numerator = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);

        let ua = ua_numerator / denominator;
        let ub = ub_numerator / denominator;

        let tolerance = f64::EPSILON;

        // Check if the intersection point is within both line segments or at their endpoints
        if (ua >= -tolerance && ua <= 1.0 + tolerance) && (ub >= -tolerance && ub <= 1.0 + tolerance) {
            let intersection_x = x1 + ua * (x2 - x1);
            let intersection_y = y1 + ua * (y2 - y1);
            let point = Point2D { x: OrderedFloat(intersection_x), y: OrderedFloat(intersection_y) };

            let is_touching = (ua.abs() < tolerance || (ua - 1.0).abs() < tolerance) ||
                (ub.abs() < tolerance || (ub - 1.0).abs() < tolerance);

            if is_touching {
                Some(Intersection::Touching { line1: *self, line2: other, point })
            } else {
                Some(Intersection::Crossing { line1: *self, line2: other, point })
            }
        } else {
            None
        }
    }


    pub fn intersects(&self, other: Line2D) -> bool {
        // if self.is_zero_length() && other.is_zero_length() {
        //     let res = self.start.approx_eq(&other.start, 1e-6);
        //     return res;
        // }

        let ccw1 = Line2D::ccw(self.start, self.end, other.start);
        let ccw2 = Line2D::ccw(self.start, self.end, other.end);
        let ccw3 = Line2D::ccw(other.start, other.end, self.start);
        let ccw4 = Line2D::ccw(other.start, other.end, self.end);

        if ccw1 == 0.0 && ccw2 == 0.0 {
            // They are colinear -> check whether they overlap
            // quickly check if bounding boxes overlap, then check parametric overlap
            if self.bbox_overlap(&other) {
                // script says parametric_overlap, but shouldn't bbox overlap be enough?
                self.parametric_overlap(&other)
            } else {
                false
            }
        } else {
            // If line segments straddle each other, they intersect
            ccw1 * ccw2 <= OrderedFloat::from(0.0) && ccw3 * ccw4 <= OrderedFloat::from(0.0)
        }
    }

    pub fn y_at(&self, x: f64) -> Option<f64> {
        if self.start.x == self.end.x {
            // Vertical line, return y1 if x matches
            if x == *self.start.x {
                Some(*self.start.y)
            } else {
                None
            }
        } else {
            let t = (x - *self.start.x) / (*self.end.x - *self.start.x);
            if t >= 0.0 && t <= 1.0 {
                Some(*self.start.y + t * (*self.end.y - *self.start.y))
            } else {
                None
            }
        }
    }

    /// Calculates whether two colinear lines overlap.
    fn parametric_overlap(&self, other: &Line2D) -> bool {
        if self.is_zero_length() {
            return other.contains(self.start);
        } else if other.is_zero_length() {
            return self.contains(other.start);
        }

        // calculate direction. Take care if one line is a point
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

        lambda1.min(lambda2) <= OrderedFloat::from(1.0) && lambda1.max(lambda2) >= OrderedFloat::from(0.0)
    }

    fn bbox_overlap(&self, other: &Line2D) -> bool {
        // Determine the bounding boxes of both line segments
        let min_x1 = self.start.x.min(self.end.x);
        let max_x1 = self.start.x.max(self.end.x);
        let min_y1 = self.start.y.min(self.end.y);
        let max_y1 = self.start.y.max(self.end.y);

        let min_x2 = other.start.x.min(other.end.x);
        let max_x2 = other.start.x.max(other.end.x);
        let min_y2 = other.start.y.min(other.end.y);
        let max_y2 = other.start.y.max(other.end.y);

        // Check for overlap in both x and y dimensions
        max_x1 >= min_x2 && min_x1 <= max_x2 && max_y1 >= min_y2 && min_y1 <= max_y2
    }

    /// Calculates whether a triangle is clockwise or counterclockwise oriented.
    /// That means whether r is on the left or on the right of a line between p and q.
    ///
    /// <0 if clockwise
    /// =0 if collinear
    /// >0 if counterclockwise
    fn ccw(p: Point2D, q: Point2D, r: Point2D) -> OrderedFloat<f64> {
        (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y)
    }
}

impl PartialEq for Line2D {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Line2D {}

impl PartialOrd for Line2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line2D {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.y.cmp(&other.start.y)
            .then_with(|| self.start.x.cmp(&other.start.x))
            .then_with(|| self.end.y.cmp(&other.end.y))
            .then_with(|| self.end.x.cmp(&other.end.x))
    }
}

impl Display for Line2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("({}, {}) -> ({}, {})", self.start.x.0, self.start.y.0, self.end.x.0, self.end.y.0))
    }
}

#[cfg(test)]
mod tests {
    use geo::line_string;
    use geo::{Intersects, LineString};
    use ordered_float::OrderedFloat;
    use crate::geometry::intersection::Intersection;
    use crate::geometry::point::Point2D;

    use super::Line2D;

    fn intersect_using_external_library(native_line1: Line2D, native_line2: Line2D) -> bool {
        let geo_line1 = transform_to_geo_line(native_line1);
        let geo_line2 = transform_to_geo_line(native_line2);

        geo_line1.intersects(&geo_line2)
    }

    fn transform_to_geo_line(line: Line2D) -> LineString<f64> {
        line_string![(x: line.start.x.0, y: line.start.y.0), (x: line.end.x.0, y: line.end.y.0)]
    }

    #[test]
    fn test_contains_point_on_horizontal_line() {
        let line = Line2D::new(0.0, 0.0, 5.0, 0.0);
        let point = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(0.0) };
        assert!(line.contains(point));
    }

    #[test]
    fn test_contains_point_on_vertical_line() {
        let line = Line2D::new(0.0, 0.0, 0.0, 5.0);
        let point = Point2D { x: OrderedFloat(0.0), y: OrderedFloat(2.0) };
        assert!(line.contains(point));
    }

    #[test]
    fn test_contains_point_on_diagonal_line() {
        let line = Line2D::new(0.0, 0.0, 5.0, 5.0);
        let point = Point2D { x: OrderedFloat(3.0), y: OrderedFloat(3.0) };
        assert!(line.contains(point));
    }

    #[test]
    fn test_contains_point_outside_segment_bounds() {
        let line = Line2D::new(0.0, 0.0, 5.0, 5.0);
        let point = Point2D { x: OrderedFloat(6.0), y: OrderedFloat(6.0) };
        assert!(!line.contains(point));
    }

    #[test]
    fn test_contains_point_not_on_line() {
        let line = Line2D::new(0.0, 0.0, 5.0, 5.0);
        let point = Point2D { x: OrderedFloat(3.0), y: OrderedFloat(4.0) };
        assert!(!line.contains(point));
    }

    #[test]
    fn test_contains_point_on_endpoint_start() {
        let line = Line2D::new(0.0, 0.0, 5.0, 5.0);
        let point = Point2D { x: OrderedFloat(0.0), y: OrderedFloat(0.0) };
        assert!(line.contains(point));
    }

    #[test]
    fn test_contains_point_on_endpoint_end() {
        let line = Line2D::new(0.0, 0.0, 5.0, 5.0);
        let point = Point2D { x: OrderedFloat(5.0), y: OrderedFloat(5.0) };
        assert!(line.contains(point));
    }

    #[test]
    fn test_contains_point_on_zero_length_line() {
        let line = Line2D::new(2.0, 2.0, 2.0, 2.0);
        let point = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(2.0) };
        assert!(line.contains(point));
    }

    #[test]
    fn test_contains_point_off_zero_length_line() {
        let line = Line2D::new(2.0, 2.0, 2.0, 2.0);
        let point = Point2D { x: OrderedFloat(3.0), y: OrderedFloat(2.0) };
        assert!(!line.contains(point));
    }

    #[test]
    fn lines_intersect_in_middle() {
        let line1 = Line2D::new(0.0, 0.0, 2.0, 0.0);
        let line2 = Line2D::new(1.0, 0.0, 1.0, 2.0);

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn lines_touch_at_edge() {
        let line1 = Line2D::new(0.0, 0.0, 2.0, 0.0);
        let line2 = Line2D::new(0.0, 0.0, 0.0, 2.0);

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn lines_dont_intersect() {
        let line1 = Line2D::new(0.0, 0.0, 1.0, 0.5);
        let line2 = Line2D::new(1.0, 2.0, 2.0, 1.5);

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(
            line1.intersects(line2),
            intersect_using_external_library(line1, line2)
        )
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
        let line1 = Line2D::new(0.0, 0.0, 1.0, 0.0);
        let line2 = Line2D::new(0.5, 0.0, 2.0, 0.0);

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
        let line1 = Line2D::new(0.0, 0.0, 1.0, 0.0);
        let line2 = Line2D::new(2.0, 0.0, 3.0, 0.0);

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2));
    }

    #[test]
    fn lines_parallel() {
        let line1 = Line2D::new(0.0, 0.0, 1.0, 1.0);
        let line2 = Line2D::new(0.0, 1.0, 1.0, 1.0);

        let result: bool = line1.intersects(line2);

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

    #[test]
    fn zero_length_and_non_zero_length_that_dont_intersect() {
        let line1 = Line2D::new(0.0, 0.0, 0.0, 0.0);
        let line2 = Line2D::new(1.0, 1.0, 10.0, 9.0);

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn zero_length_and_non_zero_length_that_dont_intersect_2() {
        // bug from file s_1000_1.dat that was indicating a wrong intersection
        let line1 = Line2D::new(10.0, 10.0, 10.0, 10.0);
        let line2 = Line2D::new(31.498, 9.526, 31.8858, 10.3111);

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn parallel_lines_dont_intersect_with_overlapping_bbox() {
        let line1 = Line2D::new(0.0, 0.0, 10.0, 10.0);
        let line2 = Line2D::new(1.0, 0.0, 10.0, 9.0);

        let result: bool = line1.intersects(line2);

        assert!(!result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn zero_length_on_line() {
        let line1 = Line2D::new(0.0, 0.0, 0.0, 0.0);
        let line2 = Line2D::new(0.0, 0.0, 1.0, 1.0);

        let result: bool = line1.intersects(line2);

        assert!(result);
        assert_eq!(result, intersect_using_external_library(line1, line2))
    }

    #[test]
    fn equality() {
        let line1 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line3 = Line2D::new(1.0, 1.0, 3.0, 3.0);
        let line4 = Line2D::new(0.5, 0.5, 1.5, 1.5);

        // Test equality
        assert_eq!(line1, line2);
        assert_ne!(line1, line3);
        assert_ne!(line1, line4);

        // Test equality with self
        assert_eq!(line1, line1);
        assert_eq!(line2, line2);
    }

    #[test]
    fn ordering() {
        let line1 = Line2D::new(0.5, 0.5, 1.5, 1.5);
        let line2 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line3 = Line2D::new(1.0, 1.5, 3.0, 3.0);
        let line4 = Line2D::new(0.0, 2.0, 3.0, 3.0);
        let line5 = Line2D::new(2.0, 2.0, 1.0, 1.0);

        // Test ordering by start y-coordinate
        assert!(line1 < line2);
        assert!(line2 < line3);
        assert!(line3 < line4);
        // Test ordering with same y, different x
        assert!(line4 < line5);


        // Test sorting
        let mut lines = vec![line4, line1, line3, line2, line5];
        lines.sort();
        assert_eq!(lines, vec![line1, line2, line3, line4, line5]);
    }

    #[test]
    fn find_intersection_when_crossing_intersection() {
        let line1 = Line2D::new(1.0, 1.0, 3.0, 3.0);
        let line2 = Line2D::new(1.0, 3.0, 3.0, 1.0);

        let intersection = line1.find_intersection(line2).unwrap();

        match intersection {
            Intersection::Crossing { point, .. } => {
                assert_eq!(point, Point2D::new(2.0,2.0))
            },
            _ => panic!("Expected Crossing intersection, but got {:?}", intersection),
        }
    }

    #[test]
    fn find_intersection_when_touching_at_end() {
        let line1 = Line2D::new(0.0, 1.0, 3.0, 1.0);
        let line2 = Line2D::new(3.0, 0.0, 3.0, 1.0);

        let intersection = line1.find_intersection(line2).unwrap();

        match intersection {
            Intersection::Touching { point, .. } => {
                assert_eq!(point, Point2D::new(3.0,1.0))
            },
            _ => panic!("Expected Touching intersection, but got {:?}", intersection),
        }
    }

    #[test]
    fn find_intersection_when_touching_in_middle() {
        let line1 = Line2D::new(0.0, 1.0, 3.0, 1.0);
        let line2 = Line2D::new(2.0, 0.0, 2.0, 1.0);

        let intersection = line1.find_intersection(line2).unwrap();

        match intersection {
            Intersection::Touching { point, .. } => {
                assert_eq!(point, Point2D::new(2.0,1.0))
            },
            _ => panic!("Expected Touching intersection, but got {:?}", intersection),
        }
    }

    #[test]
    fn find_intersection_when_collinear_touching_at_end() {
        let line1 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(2.0, 2.0, 3.0, 0.0);

        let intersection = line1.find_intersection(line2).unwrap();

        match intersection {
            Intersection::Touching { point, .. } => assert_eq!(point, Point2D { x: OrderedFloat(2.0), y: OrderedFloat(2.0) }),
            _ => panic!("Expected Touching intersection, but got {:?}", intersection),
        }
    }

    #[test]
    fn find_intersection_when_identical() {
        let line1 = Line2D::new(0.0, 0.0, 5.0, 0.0);

        let intersection = line1.find_intersection(line1).unwrap();

        match intersection {
            Intersection::IdenticalOverlap { overlap, .. } => {
                assert_eq!(overlap, line1)
            },
            _ => panic!("Expected IdenticalOverlap intersection, but got {:?}", intersection),
        }
    }

    #[test]
    fn find_intersection_when_collinear_one_fully_contained() {
        let line1 = Line2D::new(0.0, 0.0, 5.0, 0.0);
        let line2 = Line2D::new(0.0, 0.0, 2.0, 0.0);

        let intersection = line1.find_intersection(line2).unwrap();

        match intersection {
            Intersection::ContainedOverlap { overlap, .. } => {
                assert_eq!(overlap, line2)
            },
            _ => panic!("Expected ContainedOverlap intersection, but got {:?}", intersection),
        }
    }

    #[test]
    fn find_intersection_when_collinear_partial_overlap() {
        let line1 = Line2D::new(0.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(1.0, 1.0, 3.0, 3.0);

        let intersection = line1.find_intersection(line2).unwrap();

        match intersection {
            Intersection::PartialOverlap { overlap, .. } => {
                let expected_overlap = Line2D::new(1.0, 1.0, 2.0, 2.0);
                assert_eq!(overlap, expected_overlap)
            },
            _ => panic!("Expected PartialOverlap intersection, but got {:?}", intersection),
        }
    }

    #[test]
    fn find_intersection_when_collinear_endpoint_overlap() {
        let line1 = Line2D::new(0.0, 0.0, 3.0, 3.0);
        let line2 = Line2D::new(3.0, 3.0, 5.0, 5.0);

        let intersection = line1.find_intersection(line2).unwrap();

        match intersection {
            Intersection::PartialOverlap { overlap, .. } => {
                let expected_overlap = Line2D::new(3.0, 3.0, 3.0, 3.0);
                assert_eq!(overlap, expected_overlap)
            },
            _ => panic!("Expected PartialOverlap intersection, but got {:?}", intersection),
        }
    }

    #[test]
    fn find_intersection_when_parallel_lines() {
        let line1 = Line2D::new(1.0, 1.0, 3.0, 3.0);
        let line2 = Line2D::new(1.0, 2.0, 3.0, 4.0);

        let intersection = line1.find_intersection(line2);
        assert!(intersection.is_none());
    }

    #[test]
    fn find_intersection_when_collinear_no_overlap() {
        let line1 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(3.0, 3.0, 4.0, 4.0);

        let intersection = line1.find_intersection(line2);
        assert!(intersection.is_none());
    }

    #[test]
    fn find_intersection_when_no_intersection() {
        let line1 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(1.0, 4.0, 2.0, 3.0);

        let intersection = line1.find_intersection(line2);
        assert!(intersection.is_none());
    }
}
