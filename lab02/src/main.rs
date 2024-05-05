use lab02::{city::City, state::State, svg_parser::parse_file_into_country};

fn main() {
    let path = "data/DeutschlandMitStaedten.svg";
    let country = parse_file_into_country(path.to_string());
    
    print_states_header();
    for state in &country.states {
        let mut cities = vec![];
        for city in &country.cities {
            if state.contains(&city) {
                cities.push(city);
            }
        }
        print_states_entry(state, cities);
    }
}

fn print_states_header() {
    println!(
        "| {0: <24} | {1: <10} | {2: <20} |",
        "State", "Area", "Cities"
    );
    println!("|-{}-|-{}-|-{}-|", "-".repeat(24), "-".repeat(10), "-".repeat(20));
}

fn print_states_entry(state: &State, cities: Vec<&City>) {
    // rounded to 2 decimal places
    println!(
        "| {0: <24} | {1: <10} | {2: <20} |",
        state.name, state.get_area().round(), 
        cities.iter().map(|city| city.name.clone()).collect::<Vec<String>>().join(", ")
    );
}
