use super::point::Point2D;

pub struct Polygon {
    pub points: Vec<Point2D>,
}

impl Polygon {
    pub fn new() -> Self {
        Polygon { points: Vec::new() }
    }

    pub fn add_point(&mut self, point: Point2D) {
        self.points.push(point);
    }

    /// Calculate the area of the polygon.
    /// Implementation based on the shoelace formula.
    pub fn get_area(&self) -> f64 {
        let mut area = 0.0;

        for i in 0..self.points.len() {
            // module to wrap to the first point when we reach the last one
            let j: usize = (i + 1) % self.points.len();
            area += self.points[i].x * self.points[j].y;
            area -= self.points[j].x * self.points[i].y;
        }
        area = area.abs() / 2.0;

        area
    }
}

mod tests {
    use crate::geometry::{point::Point2D, polygon::Polygon};

    #[test]
    fn get_area_should_return_zero_when_polygon_has_no_points() {
        let polygon = Polygon::new();
        assert_eq!(polygon.get_area(), 0.0);
    }

    #[test]
    fn get_area_should_return_zero_when_polygon_has_one_point() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        
        let area = polygon.get_area();
        let expected = 0.0;

        assert!((area - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn get_area_should_return_zero_when_polygon_has_two_points() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(1.0, 1.0));
        
        let area = polygon.get_area();
        let expected = 0.0;

        assert!((area - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn get_area_should_return_correct_value_when_polygon_is_triangle() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(0.0, 3.0));

        
        let area = polygon.get_area();
        let expected = 6.0;

        assert!((area - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn get_area_should_return_correct_value_when_polygon_is_square() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(0.0, 4.0));

        
        let area = polygon.get_area();
        let expected = 16.0;

        assert!((area - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn get_area_should_return_correct_value_when_polygo_is_irregular() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(1.0, 1.0));
        polygon.add_point(Point2D::new(4.0, 1.0));
        polygon.add_point(Point2D::new(4.0, 5.0));
        polygon.add_point(Point2D::new(1.0, 5.0));
        polygon.add_point(Point2D::new(3.0, 3.0));

        let area = polygon.get_area();
        let expected = 8.0;

        assert!((area - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_polygon_with_negative_coordinates() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(-1.0, -1.0));
        polygon.add_point(Point2D::new(4.0, -1.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(-1.0, 4.0));

        assert!((polygon.get_area() - 25.0).abs() < f64::EPSILON);
    }

}
