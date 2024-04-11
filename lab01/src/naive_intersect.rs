use crate::geometry::Line2D;



pub fn get_intersections(lines: &Vec<Line2D>) -> Vec<(Line2D, Line2D)> {
    let mut intersections: Vec<(Line2D, Line2D)>= Vec::new();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if lines[i].intersects(lines[j]) {
                intersections.push((lines[i], lines[j]))
            }
        }
    }
    intersections
}

