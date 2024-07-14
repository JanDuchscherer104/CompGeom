use svg::{
    node::element::{
        path::{Command, Data, Position},
        tag::Type,
    },
    parser::Event,
};

use crate::{city::City, country::Country, geometry::{point::Point2D, polygon::Polygon}, state::State};

pub fn parse_file_into_country(path: String) -> Country {
    let mut content = String::new();
    let mut states: Vec<State> = Vec::new();
    let mut cities: Vec<City> = Vec::new();

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
                    } else {
                        cities.push(parse_path_to_city(attributes));
                    }
                }
            }
            _ => {}
        }
    }

    Country {
        states: states,
        cities: cities,
    }
}

fn parse_path_to_state(attributes: std::collections::HashMap<String, svg::node::Value>) -> State {
    let name = attributes.get("id").unwrap().to_string();
    let mut polygons: Vec<Polygon> = Vec::new();

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

    // split into polygons and holes
    // svg uses fill-rule: nonzero by default.
    // determining by the direction of the polygon is a oversimplification, but it works for the given data.ss
    let (shell, holes) = categorize_polygons(polygons);
    
    State {
        name: name,
        polygons: shell,
        holes: holes,
    }
}

/// Simplified algorithm to categorize whether a polygon is a shell or a hole.
/// Assumptions:
/// - first polygon is a shell
/// - Polygons#is_nested() considers the whole polygon as nested as soon as a single point is inside.
fn categorize_polygons(polygons: Vec<Polygon>) -> (Vec<Polygon>, Vec<Polygon>) {
    let mut holes = Vec::new();
    let mut shells = Vec::new();

    if !polygons.is_empty() {
        shells.push(polygons[0].clone());
    }

    for i in 1..polygons.len() {
        let mut is_hole = false;

        for shell in &shells {
            if polygons[i].is_nested(shell) {
                is_hole = true;
                break;
            }
        }

        if is_hole {
            holes.push(polygons[i].clone());
        } else {
            shells.push(polygons[i].clone());
        }
    }

    (shells, holes)
}

fn parse_path_to_city(attributes: std::collections::HashMap<String, svg::node::Value>) -> City {
    City {
        name: attributes.get("id").unwrap().to_string(),
        location: Point2D::new(
            attributes.get("sodipodi:cx").unwrap().parse().unwrap(),
            attributes.get("sodipodi:cy").unwrap().parse().unwrap(),
        ),
    }
}