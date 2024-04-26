use svg::{
    node::element::{
        path::{Command, Data, Position},
        tag::Type,
    },
    parser::Event,
};

use crate::{point::Point2D, polygon::Polygon, state::State};

pub fn parse_file_into_states(path: String) -> Vec<State> {
    let mut content = String::new();
    let mut states: Vec<State> = Vec::new();

    // the first group contains the states, the second group contains the cities.
    let mut is_state_group = true;

    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(name, tag_type, attributes) => {
                if name == "g" && tag_type == Type::End {
                    is_state_group = false;
                }
                if name == "path" {
                    if is_state_group {
                        states.push(parse_path_to_state(attributes));
                    }
                }
            }
            _ => {}
        }
    }

    states
}

fn parse_path_to_state(attributes: std::collections::HashMap<String, svg::node::Value>) -> State {
    let name = attributes.get("id").unwrap().to_string();
    let mut polygons = Vec::new();

    let data = attributes.get("d").unwrap();
    let data = Data::parse(data).unwrap();

    for command in data.iter() {
        match command {
            Command::Move(position, parameters) => {
                if *position == Position::Absolute {
                    polygons.push(Polygon::new());
                    let x = parameters[0];
                    let y = parameters[1];
    
                    polygons
                        .last_mut()
                        .unwrap()
                        .add_point(Point2D::new(x as f64, y as f64));
                } else {
                    panic!("Relative Move not supported")
                }

            }
            Command::Line(position, parameters) => {
                let mut point = Point2D::new(0.0, 0.0);
                if *position == Position::Absolute {
                    point.x = parameters[0] as f64;
                    point.y = parameters[1] as f64;
                } else {
                    let last_point = polygons.last().unwrap().points.last().unwrap();
                    point.x = last_point.x + parameters[0] as f64;
                    point.y = last_point.y + parameters[1] as f64;
                }

                polygons
                    .last_mut()
                    .unwrap()
                    .add_point(point);
            }
            Command::Close => {
                // remove the last point, since it is the same as the first point
                polygons.last_mut().unwrap().points.pop();
            }
            _ => {}
        }
    }

    State {
        name: name,
        polygons: polygons,
    }
}
