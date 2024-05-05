use super::point::Point2D;

pub fn ccw(p: Point2D, q: Point2D, r: Point2D) -> f64 {
    (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y)
}

mod tests {
    use super::*;

    #[test]
    fn ccw_should_return_positive_when_points_form_triangle() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(4.0, 0.0);
        let r = Point2D::new(0.0, 3.0);
        assert_eq!(ccw(p, q, r), 12.0);
    }

    #[test]
    fn ccw_should_return_zero_when_points_on_line() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(4.0, 0.0);
        let r = Point2D::new(0.0, 0.0);
        assert_eq!(ccw(p, q, r), 0.0);
    }

    #[test]
    fn ccw_should_return_negative_when_points_form_triangle_in_reverse_order() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(0.0, 3.0);
        let r = Point2D::new(4.0, 0.0);
        assert_eq!(ccw(p, q, r), -12.0);
    }

    #[test]
    fn ccw_should_return_zero_when_all_all_on_same_point() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(0.0, 0.0);
        let r = Point2D::new(0.0, 0.0);
        assert_eq!(ccw(p, q, r), 0.0);
    }

    #[test]
    fn ccw_should_return_zero_when_points_are_collinear() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(1.0, 1.0);
        let r = Point2D::new(2.0, 2.0);
        assert_eq!(ccw(p, q, r), 0.0);
    }

}