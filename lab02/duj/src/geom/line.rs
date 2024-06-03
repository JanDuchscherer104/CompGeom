use super::point::Point2D;

#[derive(Copy, Clone, Debug)]
pub struct Line2D {
    pub p: Point2D,
    pub q: Point2D,
}

impl Line2D {
    pub fn new(px: f64, py: f64, qx: f64, qy: f64) -> Line2D {
        Line2D {
            p: Point2D::new(px, py),
            q: Point2D::new(qx, qy),
        }
    }

    pub fn length(&self) -> f64 {
        self.p.distance(&self.q)
    }

    pub fn almost_eq(&self, other: &Self) -> bool {
        self.p.almost_eq(&other.p) && self.q.almost_eq(&other.q)
            || self.p.almost_eq(&other.q) && self.q.almost_eq(&other.p)
    }

    pub fn is_vertical(&self) -> bool {
        self.p.x == self.q.x
    }

    pub fn is_almost_vertical(&self) -> bool {
        (self.p.x - self.q.x).abs() < f64::EPSILON
    }

    pub fn slope(&self) -> f64 {
        let dx = self.q.x - self.p.x;
        let dy = self.q.y - self.p.y;
        if dx.abs() < f64::EPSILON {
            if dy > 0.0 {
                f64::INFINITY
            } else {
                f64::NEG_INFINITY
            }
        } else {
            dy / dx
        }
    }

    pub fn contains(&self, point: &Point2D) -> bool {
        if point.x < self.p.x.min(self.q.x) || point.x > self.p.x.max(self.q.x) {
            return false;
        }

        if point.y < self.p.y.min(self.q.y) || point.y > self.p.y.max(self.q.y) {
            return false;
        }
        // cross product needs to be zero
        Point2D::ccw(&self.p, &self.q, point) == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let l = Line2D::new(0.0, 0.0, 3.0, 4.0);
        assert_eq!(l.length(), 5.0);
    }

    #[test]
    fn test_almost_eq() {
        let line1 = Line2D::new(0.0, 0.0, 1.0, 3.141592653589793238);
        let line2 = Line2D::new(0.0, 0.0, 1.0, 3.141592653589793238 + f64::EPSILON * 0.99);
        let line3 = Line2D::new(0.0, 0.0, 1.0, 3.141592653589793238 + f64::EPSILON * 1.01);

        assert!(line1.almost_eq(&line2));
        assert!(!line1.almost_eq(&line3));
    }

    #[test]
    fn test_is_vertical() {
        let vertical_line = Line2D::new(0.0, 0.0, 0.0, 1.0);
        let non_vertical_line = Line2D::new(0.0, 0.0, 1.0, 1.0);
        assert!(vertical_line.is_vertical());
        assert!(!non_vertical_line.is_vertical());
    }

    #[test]
    fn test_is_almost_vertical() {
        let almost_vertical_line = Line2D::new(0.0, 0.0, f64::EPSILON * 0.99, 1.0);
        let non_almost_vertical_line = Line2D::new(0.0, 0.0, 1.0, 1.0);
        assert!(almost_vertical_line.is_almost_vertical());
        assert!(!non_almost_vertical_line.is_almost_vertical());
    }

    #[test]
    fn test_slope() {
        let horizontal_line = Line2D::new(0.0, 0.0, 1.0, 0.0);
        let diagonal_line = Line2D::new(0.0, 0.0, 1.0, 1.0);
        assert_eq!(horizontal_line.slope(), 0.0);
        assert_eq!(diagonal_line.slope(), 1.0);

        let diagonal_line = Line2D::new(0.0, 0.0, -1.0, -1.0);
        assert_eq!(diagonal_line.slope(), 1.0);
    }

    #[test]
    fn test_slope_vertical() {
        let vertical_line = Line2D::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(vertical_line.slope(), f64::INFINITY);
        let vertical_line = Line2D::new(0.0, 0.0, 0.0, -1.0);
        assert_eq!(vertical_line.slope(), f64::NEG_INFINITY);
        let vertical_line = Line2D::new(0.0, -1.0, 0.0, 0.0);
        assert_eq!(vertical_line.slope(), f64::INFINITY);
    }

    #[test]
    fn test_contains() {
        let line = Line2D::new(0.0, 0.0, 2.0, 2.0);
        let point_on_line = Point2D::new(1.0, 1.0);
        let point_off_line = Point2D::new(1.0, 2.0);
        let point_outside_line_segment = Point2D::new(3.0, 3.0);

        // Test for a point that lies on the line
        assert!(line.contains(&point_on_line));

        // Test for a point that does not lie on the line
        assert!(!line.contains(&point_off_line));

        // Test for a point that lies on the line but outside the line segment
        assert!(!line.contains(&point_outside_line_segment));
    }
}
