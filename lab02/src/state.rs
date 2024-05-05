use crate::{city::City, geometry::polygon::Polygon};

pub struct State {
    pub name: String,
    pub polygons: Vec<Polygon>,
}

impl State {
    pub fn get_area(&self) -> f64 {
        self.polygons.iter().map(|polygon| polygon.get_area()).sum()
    }

    pub fn contains(&self, city: City) -> bool {
        self.polygons.iter().any(|polygon| polygon.contains(city.location))
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

    #[test]
    fn contains_should_return_true_when_city_is_inside_polygon() {
        let mut state = State {
            name: String::from("State"),
            polygons: vec![Polygon::new()],
        };

        state.polygons[0].add_point(Point2D::new(0.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 1.0));
        state.polygons[0].add_point(Point2D::new(0.0, 1.0));

        let city = City {
            name: "City".to_string(),
            location: Point2D::new(0.5, 0.5),};

        assert!(state.contains(city));
    }

    #[test]
    fn contains_should_return_true_when_city_is_in_second_polygon() {
        let mut state = State {
            name: String::from("State"),
            polygons: vec![Polygon::new(), Polygon::new()],
        };

        state.polygons[0].add_point(Point2D::new(0.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 1.0));
        state.polygons[0].add_point(Point2D::new(0.0, 1.0));

        state.polygons[1].add_point(Point2D::new(2.0, 2.0));
        state.polygons[1].add_point(Point2D::new(3.0, 2.0));
        state.polygons[1].add_point(Point2D::new(3.0, 3.0));
        state.polygons[1].add_point(Point2D::new(2.0, 3.0));

        let city = City {
            name: "City".to_string(),
            location: Point2D::new(2.5, 2.5),
        };

        assert!(state.contains(city));
    }

    #[test]
    fn contains_should_return_false_when_city_is_in_no_polygon() {
        let mut state = State {
            name: String::from("State"),
            polygons: vec![Polygon::new(), Polygon::new()],
        };

        state.polygons[0].add_point(Point2D::new(0.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 0.0));
        state.polygons[0].add_point(Point2D::new(1.0, 1.0));
        state.polygons[0].add_point(Point2D::new(0.0, 1.0));

        state.polygons[1].add_point(Point2D::new(2.0, 2.0));
        state.polygons[1].add_point(Point2D::new(3.0, 2.0));
        state.polygons[1].add_point(Point2D::new(3.0, 3.0));
        state.polygons[1].add_point(Point2D::new(2.0, 3.0));

        let city = City {
            name: "City".to_string(),
            location: Point2D::new(4.0, 4.0),
        };

        assert!(!state.contains(city));
    }

    #[test]
    fn contains_should_return_false_when_state_has_no_polygon() {
        let state = State {
            name: String::from("State"),
            polygons: vec![],
        };

        let city = City {
            name: "City".to_string(),
            location: Point2D::new(4.0, 4.0),
        };

        assert!(!state.contains(city));
    }

}