use std::collections::HashMap;

use svg::node::{
    element::path::{Command, Data, Position},
    Value,
};

use super::{city::City, geom::Polygon2D};

#[derive(Clone, Debug)]
pub enum RegionType {
    OuterBoundary(Polygon2D),
    ExclusionZone(Polygon2D),
}

#[derive(Clone, Debug)]
pub struct State {
    pub name: String,
    pub regions: Vec<RegionType>,
    pub capital: Option<City>,
    pub area: Option<f64>,
}

impl State {
    pub fn new(name: String, regions: Vec<RegionType>) -> Self {
        Self {
            name,
            regions,
            capital: None,
            area: None,
        }
    }

    pub fn parse_svg(attributes: HashMap<String, Value>) -> Result<Self, &'static str> {
        let name = attributes.get("id").unwrap().to_string();

        let data = attributes.get("d").unwrap();
        let data = Data::parse(data).unwrap();

        let mut regions: Vec<Polygon2D> = Vec::new();

        for command in data.iter() {
            match command {
                Command::Move(Position::Absolute, params) if params.len() >= 2 => {
                    regions.push(Polygon2D::new());
                    regions
                        .last_mut()
                        .unwrap()
                        .push(params[0].into(), params[1].into());
                }
                Command::Line(position, params) if params.len() >= 2 => {
                    let x = params[0] as f64;
                    let y = params[1] as f64;

                    if *position == Position::Absolute {
                        regions.last_mut().unwrap().push(x, y);
                    } else {
                        regions.last_mut().unwrap().push_relative(x, y);
                    }
                }
                Command::Close => {
                    // remove the last point, since it is the same as the first point
                    regions.last_mut().unwrap().vertices.pop();
                }
                _ => {}
            }
        }

        let outer_perimeter = regions.remove(0);

        let mut classified_regions: Vec<RegionType> = regions
            .into_iter()
            .map(|polygon| {
                if outer_perimeter.contains_other(&polygon) {
                    RegionType::ExclusionZone(polygon)
                } else {
                    RegionType::OuterBoundary(polygon)
                }
            })
            .collect();

        classified_regions.insert(0, RegionType::OuterBoundary(outer_perimeter));

        Ok(State::new(name, classified_regions))
    }

    pub fn compute_area(&mut self) -> f64 {
        let area = self
            .regions
            .iter()
            .map(|region| match region {
                RegionType::OuterBoundary(polygon) => polygon.area(),
                RegionType::ExclusionZone(polygon) => -polygon.area(),
            })
            .sum();
        self.area = Some(area);
        area
    }

    pub fn contains(&self, city: &City) -> bool {
        let in_outer_boundary = self.regions.iter().any(|region| match region {
            RegionType::OuterBoundary(polygon) => polygon.contains(&city.location),
            _ => false,
        });

        let in_exclusion_zone = self.regions.iter().any(|region| match region {
            RegionType::ExclusionZone(polygon) => polygon.contains(&city.location),
            _ => false,
        });

        in_outer_boundary && !in_exclusion_zone
    }

    pub fn scale(&mut self, width_ratio: f64, height_ratio: f64) {
        for region in &mut self.regions {
            match region {
                RegionType::OuterBoundary(polygon) | RegionType::ExclusionZone(polygon) => {
                    polygon.scale(width_ratio, height_ratio)
                }
            }
        }
    }
}
