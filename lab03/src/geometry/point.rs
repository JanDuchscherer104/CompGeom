use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point2D {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: OrderedFloat(x), y: OrderedFloat(y) }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.partial_cmp(&other.x) {
            Some(std::cmp::Ordering::Equal) => self.y.partial_cmp(&other.y).unwrap_or(std::cmp::Ordering::Equal),
            Some(order) => order,
            None => std::cmp::Ordering::Equal,
        }
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