use crate::geometry::polygon::Polygon;

pub struct State {
    pub name: String,
    pub polygons: Vec<Polygon>,
}

impl State {
    pub fn get_area(&self) -> f64 {
        self.polygons.iter().map(|polygon| polygon.get_area()).sum()
    }
}

mod tests {
    use super::*;
    use crate::geometry::point::Point2D;

    #[test]
    fn test_get_area_when_state_has_one_polygon() {
        let mut state = State {
            name: String::from("State"),
            polygons: vec![Polygon::new()],
        };

        state.polygons[0].add_point(Point2D::new(0.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 1.0));
        state.polygons[0].add_point(Point2D::new(0.0, 1.0));

        assert_eq!(state.get_area(), 1.0);
    }

    #[test]
    fn test_get_area_when_state_has_multiple_polygons() {
        let mut state = State {
            name: String::from("State"),
            polygons: vec![Polygon::new(), Polygon::new()],
        };

        state.polygons[0].add_point(Point2D::new(0.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 1.0));
        state.polygons[0].add_point(Point2D::new(0.0, 1.0));

        state.polygons[1].add_point(Point2D::new(0.0, 0.0));
        state.polygons[1].add_point(Point2D::new(1.0, 0.0));
        state.polygons[1].add_point(Point2D::new(1.0, 1.0));
        state.polygons[1].add_point(Point2D::new(0.0, 1.0));

        assert_eq!(state.get_area(), 2.0);
    }
}