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