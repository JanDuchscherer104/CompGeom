use crate::geometry::Line2D;

pub fn get_intersections(lines: &Vec<Line2D>) -> Vec<(Line2D, Line2D)> {
    let mut intersections: Vec<(Line2D, Line2D)> = Vec::new();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if lines[i].intersects(lines[j]) {
                intersections.push((lines[i], lines[j]))
            }
        }
    }
    intersections
}

mod tests {
    use crate::geometry::Line2D;

    use super::get_intersections;

    #[test]
    fn empty_vector() {
        let lines = Vec::new();

        let intersections = get_intersections(&lines);

        assert_eq!(intersections.len(), 0)
    }

    #[test]
    fn no_intersections() {
        let lines = vec![
            Line2D::new(0.0, 0.0, 1.0, 0.0),
            Line2D::new(0.0, 1.0, 1.0, 1.0),
        ];

        let intersections = get_intersections(&lines);
        println!("{}", intersections.len());
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn one_intersection() {
        let lines = vec![
            Line2D::new(0.0, 0.0, 1.0, 1.0),
            Line2D::new(0.0, 1.0, 1.0, 0.0),
        ];

        let intersections = get_intersections(&lines);

        assert_eq!(intersections.len(), 1);
    }

    #[test]
    fn multiple_intersections() {
        let lines = vec![
            Line2D::new(0.0, 0.0, 1.0, 1.0),
            Line2D::new(0.0, 1.0, 1.0, 0.0),
            Line2D::new(0.0, 0.0, 0.0, 1.0),
        ];

        let intersections = get_intersections(&lines);

        assert_eq!(intersections.len(), 3)
    }

    #[test]
    fn collinear_lines() {
        let lines = vec![
            Line2D::new(0.0, 0.0, 1.0, 1.0),
            Line2D::new(1.5, 1.5, 2.5, 2.5),
        ];

        let intersections = get_intersections(&lines);

        assert_eq!(intersections.len(), 0);
    }
}
