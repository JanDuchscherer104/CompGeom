#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub p1: Point2D,
    pub p2: Point2D,
}

pub fn css(p1: &Point2D, p2: &Point2D, p3: &Point2D) -> f32 {
    // (p1q2 - p2q1) + (q1r2 - q2r1) + (p2r1 - p1r2)
    // p2(r1 - q1) + q2(p1 - r1) + r2(q1- p1)
    p2.x * (p3.y - p1.y) + p2.y * (p1.x - p3.x) + p3.x * (p1.y - p2.y)
}

pub fn css_line(line1: &Line2D, line2: &Line2D) -> f32 {
    css(&line1.p1, &line1.p2, &line2.p1) * css(&line1.p1, &line1.p2, &line2.p2)
}

pub fn count_intersections(lines: &Vec<Line2D>) -> usize {
    let mut num_intersections = 0;
    for i in 0..lines.len() {
        let p = &lines[i];
        for j in i..lines.len() {
            if css_line(p, &lines[j]) <= 0.0 {
                num_intersections += 1
            }
        }
    }
    num_intersections
}
