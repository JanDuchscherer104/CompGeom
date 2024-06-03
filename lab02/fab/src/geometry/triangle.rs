use super::{line::Line2D, point::Point2D};

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    a: Point2D,
    b: Point2D,
    c: Point2D,
}

impl Triangle {
    pub fn new(a: Point2D, b: Point2D, c: Point2D) -> Self {
        Triangle { a, b, c }
    }

    pub fn get_area(&self) -> f64 {
        let ab = Line2D::new(self.a, self.b).get_length();
        let bc = Line2D::new(self.b, self.c).get_length();
        let ca = Line2D::new(self.c, self.a).get_length();
        let s = (ab + bc + ca) / 2.0;
        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }
    
    pub fn contains_point(&self, point: Point2D) -> bool {
        let signed_area = |a: Point2D, b: Point2D, c: Point2D| -> f64 {
            (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
        };

        let d1 = signed_area(point, self.a, self.b);
        let d2 = signed_area(point, self.b, self.c);
        let d3 = signed_area(point, self.c, self.a);

        let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
        let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;

        !(has_neg && has_pos)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_get_area_when_points_form_triangle() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(4.0, 0.0);
        let c = Point2D::new(0.0, 3.0);
        let triangle = Triangle::new(a, b, c);
        assert_eq!(triangle.get_area(), 6.0);
    }

    #[test]
    fn test_get_area_when_points_on_line() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(4.0, 0.0);
        let c = Point2D::new(0.0, 0.0);
        let triangle = Triangle::new(a, b, c);
        assert_eq!(triangle.get_area(), 0.0);
    }

    #[test]
    fn test_contains_point_when_contains_point() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(4.0, 0.0);
        let c = Point2D::new(0.0, 3.0);
        let triangle = Triangle::new(a, b, c);
        let p = Point2D::new(1.0, 1.0);
        assert!(triangle.contains_point(p));
    }

    #[test]
    fn test_contains_point_when_point_is_outside() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(4.0, 0.0);
        let c = Point2D::new(0.0, 3.0);
        let triangle = Triangle::new(a, b, c);
        let p = Point2D::new(5.0, 5.0);
        assert!(!triangle.contains_point(p));
    }

    #[test]
    fn test_contains_point_when_point_is_on_edge() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(4.0, 0.0);
        let c = Point2D::new(0.0, 3.0);
        let triangle = Triangle::new(a, b, c);
        let p = Point2D::new(2.0, 0.0);
        assert!(triangle.contains_point(p));
    }

    #[test]
    fn test_contains_point_when_point_is_vertex() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(4.0, 0.0);
        let c = Point2D::new(0.0, 3.0);
        let triangle = Triangle::new(a, b, c);
        assert!(triangle.contains_point(a));
        assert!(triangle.contains_point(b));
        assert!(triangle.contains_point(c));
    }

    #[test]
    fn test_contains_point_when_triangle_is_degenerate() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(4.0, 0.0);
        let c = Point2D::new(8.0, 0.0);
        let triangle = Triangle::new(a, b, c);
        let p = Point2D::new(5.0, 5.0);
        assert!(!triangle.contains_point(p));
    }

    #[test]
    fn test_contains_point_when_triangle_is_degenerate_and_point_is_on_line() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(4.0, 0.0);
        let c = Point2D::new(8.0, 0.0);
        let triangle = Triangle::new(a, b, c);
        let p = Point2D::new(5.0, 0.0);
        assert!(triangle.contains_point(p));
    }    
}
