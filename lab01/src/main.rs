mod geometry;
mod naive_intersect;
mod external_library;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::Instant,
};

use geometry::{Line2D, Point2D};

use crate::naive_intersect::get_intersections;
use crate::external_library::get_intersections_external;


fn main() {
    let filenames = vec!["s_1000_1.dat", "s_10000_1.dat", "s_100000_1.dat"];

    print_header();
    for filename in filenames {
        let lines = read_lines_from_file(&format!(".data/{}", filename)).unwrap();
        let start_time = Instant::now();
        let intersections = get_intersections_external(&lines);
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
        "# Lines", "# Intersections", "Duration (ms)"
    );
    println!("{}", "-".repeat(49));
}

fn print_entry(number_lines: u32, number_intersections: u32, duration_ms: u32) {
    println!(
        "| {0: <10} | {1: <16} | {2: <13} |",
        number_lines, number_intersections, duration_ms
    );
}

fn read_lines_from_file(file_name: &str) -> io::Result<Vec<Line2D>> {
    let path = Path::new(file_name);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut lines: Vec<Line2D> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<f64> = line
            .split(' ')
            .map(|num| num.trim().parse().unwrap())
            .collect();
        if nums.len() == 4 {
            let point1 = Point2D {
                x: nums[0],
                y: nums[1],
            };
            let point2 = Point2D {
                x: nums[2],
                y: nums[3],
            };
            let line = Line2D {
                start: point1,
                end: point2,
            };
            lines.push(line);
        }
    }

    Ok(lines)
}
