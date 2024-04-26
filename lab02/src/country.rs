use crate::{city::City, state::State};


pub struct Country {
    pub states: Vec<State>,
    pub cities: Vec<City>,
}