pub mod external_library;
pub mod geometry;
pub mod naive_intersect;
pub mod utils;
use cpu_time::ProcessTime;

use utils::read_lines_from_file;

use external_library::get_intersections_external;
use crate::naive_intersect::get_intersections;

fn main() {
    let filenames = vec!["s_1000_1.dat", "s_10000_1.dat", "s_100000_1.dat"];

    print_header();
    for filename in filenames {
        let lines = read_lines_from_file(&format!(".data/{}", filename)).unwrap();
        let start_time = ProcessTime::now();

        // let intersections = get_intersections_external(&lines);
        let intersections = get_intersections(&lines);
        let duration = start_time.elapsed().as_millis();
        print_entry(
            lines.len().try_into().unwrap(),
            intersections.len().try_into().unwrap(),
            duration.try_into().unwrap(),
        )
    }
}

fn print_header() {
    println!("{}", "-".repeat(49));
    println!(
        "| {0: <10} | {1: <16} | {2: <13} |",
        "# Lines", "# Intersections", "CPU Time (ms)"
    );
    println!("{}", "-".repeat(49));
}

fn print_entry(number_lines: u32, number_intersections: u32, duration_ms: u32) {
    println!(
        "| {0: <10} | {1: <16} | {2: <13} |",
        number_lines, number_intersections, duration_ms
    );
}
