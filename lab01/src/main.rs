mod geometry;
use std::{fs::File, io::{self, BufRead}, path::Path};

use geometry::{Line2D, Point2D};

fn main() {
    println!("Hello, world!");
    let file = ".data/s_1000_1.dat";
    let lines = read_lines_from_file(file).unwrap();
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
