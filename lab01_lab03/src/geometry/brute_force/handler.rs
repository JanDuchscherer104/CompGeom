use crate::geometry::intersection::Intersection;
use crate::geometry::line::Line2D;
use ordered_float::OrderedFloat;
use std::cmp::max;
use std::collections::HashSet;

pub struct BruteForceHandler {
    lines: Vec<Line2D>,
    intersections: HashSet<Intersection>,
}

impl BruteForceHandler {
    pub fn new(mut lines: Vec<Line2D>) -> Self {
        Self {
            lines: lines.drain(..).collect(),
            intersections: HashSet::new(),
        }
    }

    pub fn run(&mut self) -> HashSet<Intersection> {
        for i in 0..self.lines.len() {
            for j in i + 1..self.lines.len() {
                let line1 = self.lines[i];
                let line2 = self.lines[j];
                if line1.intersects(line2) {
                    let intersection = line1
                        .find_intersection(line2)
                        .expect("Intersection not found!");

                    self.intersections.insert(intersection);
                }
            }
        }

        self.intersections.clone()
    }

    pub fn analyze(&self) {
        if self.intersections.len() == 0 {
            println!("No intersections found!");
            return;
        }

        let mut x_coords: HashSet<OrderedFloat<f64>> = HashSet::new();
        let mut same_x_coords_counter = 0;

        let mut zero_length_lines = Vec::new();
        let mut touching_lines = Vec::new();
        let mut vertical_lines = Vec::new();
        let mut overlapping_lines = Vec::new();

        for line in self.lines.clone() {
            if line.is_zero_length() {
                zero_length_lines.push(line);
            }

            if line.is_vertical() {
                vertical_lines.push(line);
            }

            let start_x = max(line.start.x, line.end.x);
            if x_coords.contains(&start_x) {
                same_x_coords_counter += 1;
            } else {
                x_coords.insert(start_x);
            }
        }

        for intersection in &self.intersections {
            match intersection {
                Intersection::Crossing { point, .. } => {
                    if x_coords.contains(&point.x) {
                        same_x_coords_counter += 1;
                    } else {
                        x_coords.insert(point.x);
                    }
                }
                Intersection::Touching { .. } => {
                    touching_lines.push(intersection);
                }
                Intersection::PartialOverlap { .. }
                | Intersection::ContainedOverlap { .. }
                | Intersection::IdenticalOverlap { .. } => {
                    overlapping_lines.push(intersection.clone());
                }
            }
        }
        println!("Analysis:");
        println!("Analyzed {} lines", self.lines.len());
        println!("Found {} intersections", self.intersections.len());
        println!("Number of zero length lines: {}", zero_length_lines.len());
        println!("Number of vertical lines: {}", vertical_lines.len());
        println!("Number of touching lines: {}", touching_lines.len());
        println!("Number of overlapping lines: {}", overlapping_lines.len());
        println!(
            "Number of duplicate x-coordinates among intersections and endpoints: {}",
            same_x_coords_counter
        );
    }
}
