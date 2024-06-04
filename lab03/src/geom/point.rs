use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn new_relative(p: &Self, dx: f64, dy: f64) -> Self {
        Self {
            x: p.x + dx,
            y: p.y + dy,
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn almost_eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON && (self.y - other.y).abs() < f64::EPSILON
    }

    pub fn ccw(p: &Self, q: &Self, r: &Self) -> f64 {
        p.x * (q.y - r.y) + p.y * (q.x - r.x) + q.x * r.y - q.y * r.x
        // (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y)
    }
}

impl Eq for Point2D {}

/// Lexicographical ordering (first by x-coordinate, then by y-coordinate)
impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else if self.x > other.x {
            Ordering::Greater
        } else if self.y < other.y {
            Ordering::Less
        } else if self.y > other.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_relative() {
        let p1 = Point2D::new(1.0, 1.0);
        let p2 = Point2D::new_relative(&p1, 2.0, 3.0);
        assert_eq!(p2.x, 3.0);
        assert_eq!(p2.y, 4.0);
    }

    #[test]
    fn test_distance() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_almost_eq() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 0.0);
        assert!(p1.almost_eq(&p2));
    }

    #[test]
    fn test_almost_eq_epsilon() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(f64::EPSILON * 0.99, f64::EPSILON * 0.99);
        assert!(p1.almost_eq(&p2));
    }

    #[test]
    fn test_colinear() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(1.0, 1.0);
        let r = Point2D::new(2.0, 2.0);
        assert_eq!(Point2D::ccw(&p, &q, &r), 0.0);
        assert_eq!(Point2D::ccw(&p, &r, &q), 0.0);
    }

    #[test]
    fn test_clockwise() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(1.0, 1.0);
        let r = Point2D::new(2.0, 0.0);
        assert!(Point2D::ccw(&p, &q, &r) < 0.0);
        assert!(Point2D::ccw(&p, &r, &q) > 0.0);
    }
}
