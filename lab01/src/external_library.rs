use geo::{Intersects, LineString};
use geo::line_string;

use crate::geometry::{Line2D, Point2D};

pub fn get_intersections_external(lines: &Vec<Line2D>) -> Vec<(Line2D, Line2D)> {
    let geo_lines: Vec<LineString<f64>> = lines.iter().map(|line| transform_to_geo_line(*line)).collect();

    let mut intersections: Vec<(Line2D, Line2D)> = Vec::new();
    for i in 0..geo_lines.len() {
        for j in i + 1..geo_lines.len() {
            if geo_lines[i].intersects(&geo_lines[j]) {
                intersections.push((lines[i], lines[j]))
            }
        }
    }
    intersections
}

fn transform_to_geo_line(line: Line2D) -> LineString<f64> {
    line_string![(x: line.start.x, y: line.start.y), (x: line.end.x, y: line.end.y)]
}


#[test]
fn lines_dont_intersect() {
    let horizontal = Line2D {
        start: Point2D { x: 0.0, y: 0.0 },
        end: Point2D { x: 1.0, y: 0.5 },
    };
    let vertical = Line2D {
        start: Point2D { x: 1.0, y: 2.0 },
        end: Point2D { x: 2.0, y: 1.5 },
    };

    assert!(!horizontal.intersects(vertical))
}
