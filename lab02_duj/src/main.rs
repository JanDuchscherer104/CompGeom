use lab02_duj::Country;

use std::{fs::File, io::Write, path::Path};

fn main() {
    let path = Path::new("data/germany.svg");
    assert!(path.exists());

    let mut country = Country::parse_svg(path).unwrap();
    let missmatches = country.match_cities();
    println!("{:?}", missmatches);
    country.scale(640.0, 876.0); // area should be km^2
    country.compute_area();
    println!(
        "Relative difference in area {}",
        (357588.0 - country.area.unwrap()) / 357588.0
    );
    let mut file = File::create("output.txt").expect("Unable to create file");
    write!(file, "{:#?}", country).expect("Unable to write data");
}
