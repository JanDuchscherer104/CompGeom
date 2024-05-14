use lab02_duj::Country;

use std::{fs::File, io::Write, path::Path};

fn main() {
    let path = Path::new("data/germany.svg");
    assert!(path.exists());

    let mut country = Country::parse_svg(path).unwrap();
    let missmatches = country.match_cities();
    println!("{:?}", missmatches);
    let mut file = File::create("output.txt").expect("Unable to create file");
    write!(file, "{:#?}", country).expect("Unable to write data");
}
