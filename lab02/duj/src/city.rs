use std::collections::HashMap;
use svg::node::Value;

use super::geom::Point2D;
#[derive(Clone, Debug)]
pub struct City {
    pub name: String,
    pub location: Point2D,
}

impl City {
    pub fn parse_svg(attributes: HashMap<String, Value>) -> Result<Self, &'static str> {
        Ok(Self {
            name: attributes.get("id").unwrap().to_string(),
            location: Point2D::new(
                attributes
                    .get("sodipodi:cx")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
                attributes
                    .get("sodipodi:cy")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
            ),
        })
    }

    pub fn scale(&mut self, width_scale: f64, height_scale: f64) {
        self.location.x *= width_scale;
        self.location.y *= height_scale;
    }
}
