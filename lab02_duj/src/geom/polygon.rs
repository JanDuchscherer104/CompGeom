use super::point::Point2D;

#[derive(Clone, Debug)]
pub struct Polygon2D {
    pub vertices: Vec<Point2D>,
}

impl Polygon2D {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, x: f64, y: f64) {
        self.vertices.push(Point2D::new(x, y));
    }

    pub fn push_relative(&mut self, dx: f64, dy: f64) {
        let last_point = match self.vertices.last() {
            Some(point) => *point,
            None => Point2D::new(0.0, 0.0),
        };
        self.vertices
            .push(Point2D::new_relative(&last_point, dx, dy));
    }

    /// Returns the bounding box of the polygon
    #[rustfmt::skip]
    pub fn get_bbox(&self) -> (Point2D, Point2D) {
        let (min_x, min_y, max_x, max_y) = self.vertices
            .iter()
            .fold(
                (f64::MAX, f64::MAX, f64::MIN, f64::MIN),
                |(min_x, min_y, max_x, max_y), point| {
                    (
                        min_x.min(point.x),
                        min_y.min(point.y),
                        max_x.max(point.x),
                        max_y.max(point.y),
                    )
                },
            );

        (Point2D::new(min_x, min_y), Point2D::new(max_x, max_y))
    }

    #[rustfmt::skip]
    pub fn is_ccw(&self) -> bool {
        let sum = self
            .vertices
            .iter()
            .enumerate()
            .fold(0.0, |acc, (i, p1)| {
                let p2 = &self.vertices.get(i + 1).unwrap_or(&self.vertices[0]);
                acc + (p2.x - p1.x) * (p2.y + p1.y)
        });
        sum > 0.0
    }

    pub fn contains(&self, point: &Point2D) -> bool {
        let (min, max) = self.get_bbox();

        // Check if the point is outside the bounding box
        if point.x < min.x || point.x > max.x || point.y < min.y || point.y > max.y {
            return false;
        }
        let epsilon = f64::EPSILON * 10.0;
        let point_outside_bbox = { Point2D::new_relative(&max, epsilon, epsilon) };

        // Retrieve index of a corner point that is not contained by (point_outside_bbox->point)
        let corner_idx = self
            .vertices
            .iter()
            .position(|&vertex| 0.0 != Point2D::ccw(&point_outside_bbox, &point, &vertex))
            .unwrap_or(0);

        let mut num_intersections = 0;
        let mut last_sign =
            Point2D::ccw(&point_outside_bbox, point, &self.vertices[corner_idx]).signum() as i8;

        for i in (corner_idx + 1)..(corner_idx + self.vertices.len() + 1) {
            let wrapped_idx = i % self.vertices.len();
            let current_sign = Point2D::ccw(&point_outside_bbox, point, &self.vertices[wrapped_idx])
                .signum() as i8;

            if (current_sign - last_sign).abs() == 2 {
                last_sign = current_sign;
                let previous_idx = if wrapped_idx == 0 {
                    self.vertices.len() - 1
                } else {
                    wrapped_idx - 1
                };
                if Point2D::ccw(
                    &self.vertices[previous_idx],
                    &self.vertices[wrapped_idx],
                    &point_outside_bbox,
                )
                .signum()
                    * Point2D::ccw(
                        &self.vertices[previous_idx],
                        &self.vertices[wrapped_idx],
                        point,
                    )
                    .signum()
                    <= 0.0
                {
                    // The point that is outside the polygon and the point of interest are on different sides of the edge
                    num_intersections += 1;
                }
            }
        }

        // The point is inside the polygon if the number of intersections is odd
        num_intersections % 2 == 1
    }

    pub fn contains_other(&self, other: &Polygon2D) -> bool {
        other.vertices.iter().all(|point| self.contains(point))
    }

    pub fn area(&self) -> f64 {
        let (area, _, _) = self
            .vertices
            .iter()
            .skip(1)
            .chain(self.vertices.first())
            .fold(
                // acc, current, next
                // (p0, p1, p2): (-1, 0, 1)->(0, 1, 2)-...->(n-2, n-1, 0)
                (
                    0.0,
                    *self.vertices.last().unwrap(),
                    *self.vertices.first().unwrap(),
                ),
                |(acc, p0, p1), &p2| (acc + (p0.x - p2.x) * p1.y, p1, p2),
            );
        area.abs() / 2.0
    }

    pub fn scale(&mut self, width_ratio: f64, height_ratio: f64) {
        for vertex in self.vertices.iter_mut() {
            vertex.x *= width_ratio;
            vertex.y *= height_ratio;
        }
    }
}

#[cfg(test)]
mod tests_contains {
    use super::*;

    #[test]
    fn test_point_inside_convex_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(0.0, 4.0);

        let point = Point2D::new(2.0, 2.0);
        assert!(polygon.contains(&point));
    }

    #[test]
    fn test_point_outside_convex_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(0.0, 4.0);

        let point = Point2D::new(5.0, 5.0);
        assert!(!polygon.contains(&point));
    }

    #[test]
    fn test_point_on_edge_of_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(0.0, 4.0);

        let point = Point2D::new(4.0, 2.0);
        assert!(polygon.contains(&point)); // Depending on definition, might be true or false
    }

    #[test]
    fn test_point_at_vertex_of_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(0.0, 4.0);

        let point = Point2D::new(4.0, 4.0);
        assert!(polygon.contains(&point)); // Depending on definition, might be true or false
    }

    #[test]
    fn test_point_inside_concave_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(2.0, 2.0);
        polygon.push(0.0, 4.0);

        let point = Point2D::new(1.0, 1.0);
        assert!(polygon.contains(&point));
    }

    #[test]
    fn test_point_outside_concave_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(2.0, 2.0);
        polygon.push(0.0, 4.0);

        let point = Point2D::new(3.0, 3.0);
        assert!(!polygon.contains(&point));
    }

    #[test]
    fn test_point_on_edge_of_concave_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(2.0, 2.0);
        polygon.push(0.0, 4.0);

        let point = Point2D::new(2.0, 2.0);
        assert!(polygon.contains(&point)); // Depending on definition, might be true or false
    }

    #[test]
    fn test_point_at_vertex_of_concave_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(2.0, 2.0);
        polygon.push(0.0, 4.0);

        let point = Point2D::new(4.0, 4.0);
        assert!(polygon.contains(&point)); // Depending on definition, might be true or false
    }

    #[test]
    fn test_degenerate_polygon_line() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);

        let point = Point2D::new(2.0, 0.0);
        assert!(polygon.contains(&point));
    }

    #[test]
    fn test_degenerate_polygon_point() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);

        let point = Point2D::new(0.0, 0.0);
        assert!(!polygon.contains(&point));
    }
}

#[cfg(test)]
mod tests_area {
    use super::*;

    #[test]
    fn test_area_square() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(0.0, 4.0);

        let expected_area = 16.0;
        assert!((polygon.area() - expected_area).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_triangle() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(2.0, 4.0);

        let expected_area = 8.0;
        assert!((polygon.area() - expected_area).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_concave_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(2.0, 2.0);
        polygon.push(4.0, 4.0);
        polygon.push(0.0, 4.0);

        let expected_area = 12.0;
        assert!((polygon.area() - expected_area).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_degenerate_polygon_line() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);

        let expected_area = 0.0;
        assert!((polygon.area() - expected_area).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_degenerate_polygon_point() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);

        let expected_area = 0.0;
        assert!((polygon.area() - expected_area).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_polygon_with_overlapping_vertices() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(4.0, 0.0);
        polygon.push(4.0, 4.0);
        polygon.push(4.0, 0.0); // Overlapping vertex
        polygon.push(0.0, 4.0);

        let expected_area = 8.0;
        assert_eq!(polygon.area(), expected_area, "Area: {}", polygon.area());
    }

    #[test]
    fn test_area_complex_polygon() {
        let mut polygon = Polygon2D::new();
        polygon.push(0.0, 0.0);
        polygon.push(6.0, 0.0);
        polygon.push(6.0, 6.0);
        polygon.push(3.0, 3.0);
        polygon.push(0.0, 6.0);

        let expected_area = 9.0 + 18.0;
        assert!((polygon.area() - expected_area).abs() < f64::EPSILON);
    }
}
