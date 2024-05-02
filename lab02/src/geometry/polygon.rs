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
}
