use super::{City, State};
use svg::{
    node::element::tag::{Path, Type::End, SVG},
    parser::Event,
};

use std::path;

#[derive(Clone, Debug)]
pub struct Country {
    pub name: Option<String>,
    pub states: Vec<State>,
    pub cities: Vec<City>,
}

impl Country {
    pub fn new() -> Self {
        Self {
            name: None,
            states: Vec::new(),
            cities: Vec::new(),
        }
    }

    pub fn add_state(&mut self, state: State) {
        self.states.push(state);
    }

    pub fn add_city(&mut self, city: City) {
        self.cities.push(city);
    }

    pub fn parse_svg(svg_path: &path::Path) -> Result<Self, &'static str> {
        let mut content = String::new();
        let mut country = Self::new();

        let mut is_state_group = true;
        for event in svg::open(svg_path, &mut content).unwrap() {
            match event {
                Event::Tag(SVG, _, attributes) => {
                    if let Some(id) = attributes.get("id") {
                        let id = format!("{}", id);
                        println!("Country name: {}", &id);
                        country.name = Some(id);
                    }
                    // let name = attributes.get("id").unwrap().to_string();
                }
                Event::Tag(Path, _, attributes) => {
                    if is_state_group {
                        country.add_state(State::parse_svg(attributes).unwrap());
                    } else {
                        country.add_city(City::parse_svg(attributes).unwrap());
                    }
                }
                Event::Tag("g", End, _) => {
                    is_state_group = false;
                }
                _ => {}
            }
        }
        Ok(country)
    }

    pub fn match_cities(&mut self) -> Vec<String> {
        let mut missmatches = Vec::new();
        for city in &self.cities {
            for state in &mut self.states {
                match (state.contains(city), state.capital.is_none()) {
                    (true, true) => state.capital = Some(city.clone()),
                    (true, false) => {
                        // return Err(format!(
                        //     "Tried to assign {} to {} but it already has capital {}",
                        //     city.name,
                        //     state.name,
                        //     state.capital.as_ref().unwrap().name
                        // ))
                        missmatches.push(format!(
                            "Tried to assign {} to {} but it already has capital {}",
                            city.name,
                            state.name,
                            state.capital.as_ref().unwrap().name
                        ))
                    }
                    _ => {}
                }
            }
        }
        missmatches
    }
}
