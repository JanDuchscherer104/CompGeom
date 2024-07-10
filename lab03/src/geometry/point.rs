use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use ordered_float::OrderedFloat;

const EPSILON: f64 = 1e-9;

#[derive(Copy, Clone, Debug, Eq, Hash)]
pub struct Point2D {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: OrderedFloat(x), y: OrderedFloat(y) }
    }

    fn nearly_equals(&self, other: &Point2D) -> bool {
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.nearly_equals(other) {
            Ordering::Equal
        } else {
            self.x.partial_cmp(&other.x)
                .unwrap_or(Ordering::Equal)
                .then_with(|| self.y.partial_cmp(&other.y).unwrap_or(Ordering::Equal))
        }
    }
}


impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.nearly_equals(other)
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_cmp() {
        let p1 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(2.0) };
        let p2 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(2.0) };
        let p3 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(3.0) };
        let p4 = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(2.0) };
        let p5 = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(3.0) };

        assert_eq!(p1, p2);
        assert!(p1 < p3);
        assert!(p1 < p4);
        assert!(p1 < p5);
        assert!(p3 < p4);
        assert!(p3 < p5);
        assert!(p4 < p5);
    }

    #[test]
    fn test_point_order_should_order_by_x_then_y() {
        let p1 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(2.0) };
        let p2 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(2.0) };
        let p3 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(3.0) };
        let p4 = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(2.0) };
        let p5 = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(3.0) };

        let mut points = vec![p5, p1, p4, p3, p2];
        points.sort();

        assert_eq!(points, vec![p1, p2, p3, p4, p5]);
    }
}