use std::collections::HashSet;
use geo::{coord, Line, LineIntersection};
use geo::line_intersection::line_intersection;
use crate::geometry::intersection::Intersection;
use crate::geometry::line::Line2D;
use crate::geometry::point::Point2D;

pub struct GeoHandler {
    lines: Vec<Line>
}

impl GeoHandler {
    pub fn new(lines: Vec<Line2D>) -> Self {
        let geo_lines: Vec<Line<f64>> = lines.iter().map(|line| GeoHandler::transform_to_geo_line(*line)).collect();

        Self {
            lines: geo_lines
        }
    }

    pub fn run(&self) -> HashSet<Intersection> {
        let mut intersections = HashSet::new();

        for i in 0..self.lines.len() {
            for j in i + 1..self.lines.len() {
                let line1 = self.lines[i];
                let line2 = self.lines[j];
                let intersection = line_intersection(line1, line2);
                if intersection.is_some() {
                    intersections.insert(Self::transform_to_intersection(intersection.unwrap(), line1, line2));
                }
            }
        }
        intersections
    }

    fn transform_to_intersection(geo_intersection: LineIntersection<f64>, line_a: Line, line_b: Line) -> Intersection {
        let line1 = Line2D::new(line_a.start.x, line_a.start.y, line_a.end.x, line_a.end.y);
        let line2 = Line2D::new(line_b.start.x, line_b.start.y, line_b.end.x, line_b.end.y);
        match geo_intersection {
            LineIntersection::SinglePoint { intersection, is_proper } => {
                let point = Point2D::new(intersection.x, intersection.y);
                if is_proper {
                    Intersection::Crossing {
                        line1,
                        line2,
                        point,
                    }
                } else {
                    Intersection::Touching {
                        line1,
                        line2,
                        point
                    }
                }
            }
            LineIntersection::Collinear { intersection } => {
                Intersection::PartialOverlap {
                    line1,
                    line2,
                    overlap: Line2D::new(intersection.start.x, intersection.start.y, intersection.end.x, intersection.end.y)
                }

            }
        }

    }

    fn transform_to_geo_line(line: Line2D) -> Line<f64> {
        Line::new(coord!(x: line.start.x.0, y: line.start.y.0), coord!(x: line.end.x.0, y: line.end.y.0))
    }
}

