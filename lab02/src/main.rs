use std::collections::HashMap;

use lab02::{city::City, state::State, svg_parser::parse_file_into_country};

fn main() {
    let path = "data/DeutschlandMitStaedten.svg";
    let country = parse_file_into_country(path.to_string());

    let actual_state_area = get_actual_state_area();

    // scale the actual area to the area in the svg file
    let state = country.states.get(0).expect("States should not be empty");
    let factor = *actual_state_area.get(&state.name.clone()).expect("State not found") as f64 / state.get_area();
    let adjusted_state_area = actual_state_area.iter().map(|(name, area)| (name.clone(), area / factor)).collect::<HashMap<String, f64>>();

    print_states_header();
    for state in &country.states {
        let mut cities = vec![];
        for city in &country.cities {
            if state.contains(&city) {
                cities.push(city);
            }
        }
        print_states_entry(state, cities, state.get_area() - *adjusted_state_area.get(&state.name).expect("State not found"));
    }
}

fn print_states_header() {
    println!(
        "| {0: <24} | {1: <10} | {2: <10} | {3: <20} |",
        "State", "Area", "ΔArea (%)", "Cities", 
    );
    println!("|-{}-|-{}-|-{}-|-{}-|", "-".repeat(24), "-".repeat(10),  "-".repeat(10), "-".repeat(20));
}

fn print_states_entry(state: &State, cities: Vec<&City>, area_diff: f64) {
    // rounded to 2 decimal places
    println!(
        "| {0: <24} | {1: <10.2} | {2: <10.4} | {3: <20} |",
        state.name, state.get_area(), 
        (area_diff / state.get_area()) * 100.0,
        cities.iter().map(|city| city.name.clone()).collect::<Vec<String>>().join(", "),
    );
}

/// Returns the actual area of the states (in square kilometers) in Germany
/// Source:     
fn get_actual_state_area() -> HashMap<String, f64> {
    let data: HashMap<String, f64> = [
        ("Baden-Württemberg".to_string(), 35747.85),
        ("Bayern".to_string(), 70541.58),
        ("Berlin".to_string(), 891.12),
        ("Brandenburg".to_string(), 29654.37),
        ("Bremen".to_string(), 419.37),
        ("Hamburg".to_string(), 755.09),
        ("Hessen".to_string(), 21115.63),
        ("Mecklenburg-Vorpommern".to_string(), 23294.90),
        ("Niedersachsen".to_string(), 47709.86),
        ("Nordrhein-Westfalen".to_string(), 34112.61),
        ("Rheinland-Pfalz".to_string(), 19857.97),
        ("Saarland".to_string(), 2571.52),
        ("Sachsen".to_string(), 18449.89),
        ("Sachsen-Anhalt".to_string(), 20464.04),
        ("Schleswig-Holstein".to_string(), 15804.30),
        ("Thüringen".to_string(), 16202.39),
        ].iter().cloned().collect();

    data
}
    
