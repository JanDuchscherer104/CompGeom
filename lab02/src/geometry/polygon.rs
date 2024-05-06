use super::point::Point2D;
use super::utils::{ccw, sign};

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

    /// Check if a point is inside the polygon.
    /// The algorithm is based on the ray casting method.
    pub fn contains(&self, point: Point2D) -> bool {
        let bounding_box = self.get_bounding_box();

        // Point outside the bounding box
        let outside: Point2D = Point2D::new(bounding_box.1.x + 1.0, bounding_box.1.y + 1.0);

        // search point that is not on the line [outside, point]
        let mut i = 0;
        while ccw(outside, point, self.points[i]) == 0.0 {
            i += 1;
            if i == self.points.len() {
                return false;
            }
        }

        let mut steps = 0;
        let mut last_result = sign(ccw(outside, point, self.points[i]));

        for j in i + 1 .. i + self.points.len() + 1 {
            let wrapped_index = j % self.points.len();
            let result = sign(ccw(outside, point, self.points[wrapped_index]));

            if (result - last_result).abs() == 2 {
                last_result = result;

                // wrapped_index-1
                let previous_index = if wrapped_index == 0 { self.points.len() - 1 } else { wrapped_index - 1 };
                if ccw(
                    self.points[previous_index],
                    self.points[wrapped_index],
                    outside,
                ) * ccw(
                    self.points[previous_index],
                    self.points[wrapped_index],
                    point,
                ) <= 0.0
                {
                    // outside and point are on different sides of the line [previous_index, wrapped_index]
                    steps += 1;
                }
            }
        }

        steps % 2 == 1
    }

    /// Return the bounding box of the polygon.
    /// The bounding box is a tuple of two points, the first one is the bottom left corner and the second one is the top right corner.
    fn get_bounding_box(&self) -> (Point2D, Point2D) {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for point in &self.points {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }

        (Point2D::new(min_x, min_y), Point2D::new(max_x, max_y))
    }

    /// Check if the polygon is clockwise.
    /// It should also work for non-simple polygons.
    /// The algorithm is from the following stackoverflow post.
    /// https://stackoverflow.com/questions/1165647/how-to-determine-if-a-list-of-polygon-points-are-in-clockwise-order
    pub fn is_clockwise(&self) -> bool {
        let mut sum = 0.0;
 
        for i in 0..self.points.len() {
            let j = (i + 1) % self.points.len();
            sum += (self.points[j].x - self.points[i].x) * (self.points[j].y + self.points[i].y);
        }

        sum > 0.0
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
    fn get_area_should_return_correct_value_when_polygon_is_cw() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(0.0, 4.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(4.0, 0.0));

        let area = polygon.get_area();
        let expected = 16.0;

        assert!((area - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn get_area_should_return_correct_value_when_polygon_is_ccw() {
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
    fn test_polygon_with_negative_coordinates() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(-1.0, -1.0));
        polygon.add_point(Point2D::new(4.0, -1.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(-1.0, 4.0));

        assert!((polygon.get_area() - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn contains_should_return_false_when_point_is_outside_polygon() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(0.0, 4.0));

        let point = Point2D::new(2.0, 6.0);
        assert!(!polygon.contains(point));
    }

    #[test]
    fn contains_should_return_true_when_point_is_inside_polygon() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(0.0, 4.0));

        let point = Point2D::new(2.0, 2.0);
        assert!(polygon.contains(point));
    }

    #[test]
    fn contains_should_return_true_when_point_is_on_edge() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(0.0, 4.0));

        let point = Point2D::new(4.0, 2.0);
        assert!(polygon.contains(point));
    }

    #[test]
    fn contains_should_return_true_when_point_is_on_vertex() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(0.0, 4.0));

        let point = Point2D::new(4.0, 4.0);
        assert!(polygon.contains(point));
    }

    #[test]
    fn contains_should_return_true_when_inside_but_ray_is_on_edge() {
        let mut polygon = Polygon::new();

        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(3.0, 0.0));
        polygon.add_point(Point2D::new(3.0, 3.0));
        polygon.add_point(Point2D::new(2.0, 2.0));
        polygon.add_point(Point2D::new(0.0, 2.0));


        let point = Point2D::new(1.0, 1.0);
        assert!(polygon.contains(point));
    }

    #[test]
    fn contains_should_return_false_when_point_is_equal_to_comparison_point() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(0.0, 4.0));

        let point = Point2D::new(5.0, 5.0);
        assert!(!polygon.contains(point));
    }

    #[test]
    fn is_clockwise_should_return_true_when_polygon_is_cw() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(0.0, 4.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(4.0, 0.0));

        assert!(polygon.is_clockwise());
    }

    #[test]
    fn is_clockwise_should_return_false_when_polygon_is_ccw() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(0.0, 4.0));

        assert!(!polygon.is_clockwise());
    }

    #[test]
    fn is_clockwise_should_return_true_when_polygon_is_cw_and_has_negative_coordinates() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(-1.0, -1.0));
        polygon.add_point(Point2D::new(-1.0, -5.0));
        polygon.add_point(Point2D::new(-5.0, -5.0));
        polygon.add_point(Point2D::new(-5.0, -1.0));

        assert!(polygon.is_clockwise());
    }

    #[test]
    fn is_clockwise_should_return_false_when_polygon_is_ccw_and_has_negative_coordinates() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(-1.0, -1.0));
        polygon.add_point(Point2D::new(-5.0, -1.0));
        polygon.add_point(Point2D::new(-5.0, -5.0));
        polygon.add_point(Point2D::new(-1.0, -5.0));

        assert!(!polygon.is_clockwise());
    }

    #[test]
    fn is_clockwise_should_return_false_when_polygon_is_ccw_and_concave() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(2.0, 2.0));
        polygon.add_point(Point2D::new(0.0, 4.0));
    
        assert!(!polygon.is_clockwise());
    }

    #[test]
    fn is_clockwise_should_return_true_when_polygon_is_cw_and_concave() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point2D::new(0.0, 0.0));
        polygon.add_point(Point2D::new(0.0, 4.0));
        polygon.add_point(Point2D::new(4.0, 4.0));
        polygon.add_point(Point2D::new(2.0, 2.0));
        polygon.add_point(Point2D::new(4.0, 0.0));
    
        assert!(polygon.is_clockwise());
    }
        
}
