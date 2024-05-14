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
    UnspecifiedRegion(Polygon2D),
}

#[derive(Clone, Debug)]
pub struct State {
    pub name: String,
    pub regions: Vec<RegionType>,
    pub capital: Option<&'static City>,
}

impl State {
    pub fn new(name: String, regions: Vec<RegionType>) -> Self {
        Self {
            name,
            regions,
            capital: None,
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

        let regions = regions
            .into_iter()
            .map(|polygon| {
                if true {
                    RegionType::OuterBoundary(polygon)
                } else if false {
                    RegionType::ExclusionZone(polygon)
                } else {
                    RegionType::UnspecifiedRegion(polygon)
                }
            })
            .collect();

        Ok(State::new(name, regions))
    }
}
