use lab02_duj::Country;

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let path = Path::new("data/germany.svg");
    assert!(path.exists());

    let country = Country::parse_svg(path).unwrap();

    let mut file = File::create("output.txt").expect("Unable to create file");
    write!(file, "{:#?}", country).expect("Unable to write data");
}
