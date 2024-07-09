use std::collections::HashSet;
use crate::geometry::intersection::Intersection;
use crate::geometry::line::Line2D;
use crate::geometry::sweep_line::event_queue::EventQueue;
use crate::geometry::sweep_line::events::Event;
use crate::geometry::sweep_line::sweep_line::SweepLine;

/// Options for the sweep line algorithm
/// panic_on_identical_x: if true, the algorithm will panic if two lines have the same x coordinate, otherwise it will ignore the line
/// panic_on_vertical: if true, the algorithm will panic if a vertical line is encountered, otherwise it will ignore the line
/// panic_on_zero_length: if true, the algorithm will panic if a line has zero length, otherwise it will ignore the line
/// panic_on_overlap: if true, the algorithm will panic if two collinear lines are overlapping, otherwise it will ignore the intersection
/// panic_on_touch: if true, the algorithm will panic if two lines are only touching, but not crossing, otherwise it will ignore the intersection
pub struct SweepLineOptions {
    pub panic_on_identical_x: bool,
    pub panic_on_vertical: bool,
    pub panic_on_zero_length: bool,
    pub panic_on_overlap: bool,
    pub panic_on_touch: bool,
    pub x_shift: f64,
}

impl SweepLineOptions {
    /// Enables panicking if a requirement is not met.
    pub fn panic_enabled() -> Self {
        SweepLineOptions {
            panic_on_identical_x: true,
            panic_on_vertical: true,
            panic_on_zero_length: true,
            panic_on_overlap: true,
            panic_on_touch: true,
            x_shift: 0.000001,
        }
    }

    /// Avoid panicking, instead try to fix the input.
    /// This might lead to incorrect results.
    pub fn panic_disabled() -> Self {
        SweepLineOptions {
            panic_on_identical_x: false,
            panic_on_vertical: false,
            panic_on_zero_length: false,
            panic_on_overlap: true,
            panic_on_touch: true,
            x_shift: 0.000001,
        }
    }
}

pub struct Handler {
    queue: EventQueue,
    sweep_line: SweepLine,
    intersections: HashSet<Intersection>,
    options: SweepLineOptions,
}

impl Handler {
    pub fn new(mut lines: Vec<Line2D>, options: SweepLineOptions) -> Self {
        lines = Self::sanity_checks(&mut lines, &options);
        let handler = Handler { queue: EventQueue::new(lines.clone()), sweep_line: SweepLine::new(), intersections: HashSet::new(), options };

        handler
    }

    pub fn run(&mut self) -> HashSet<Intersection> {
        while !self.queue.is_empty() {
            let event = self.queue.pop();
            if let Some(event) = event {
                self.handle_event(event);
            }
        }
        self.intersections.clone()
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::StartEvent { line } => { self.handle_start_event(line) }
            Event::EndEvent { line } => { self.handle_end_event(line) }
            Event::IntersectionEvent { intersection, smaller, bigger } => {
                self.handle_intersection_event(intersection, smaller, bigger)
            }
        }
    }

    fn handle_start_event(&mut self, line: Line2D) {
        self.sweep_line.set_x(*line.start.x);

        self.sweep_line.add(line);
        let neighbors = self.sweep_line.get_neighbors(&line);

        if let Some(small_neighbor) = neighbors.smaller {
            let intersection_point = line.find_intersection(small_neighbor);
            if let Some(intersection) = intersection_point {
                self.add_intersection_event(intersection, small_neighbor, line);
            }
        }
        if let Some(big_neighbor) = neighbors.bigger {
            let intersection_point = line.find_intersection(big_neighbor);
            if let Some(intersection) = intersection_point {
                self.add_intersection_event(intersection, line, big_neighbor);
            }
        }
    }

    fn handle_end_event(&mut self, line: Line2D) {
        self.sweep_line.set_x(*line.end.x);

        let neighbors = self.sweep_line.get_neighbors(&line);
        self.sweep_line.remove(&line);

        if let (Some(small_neighbor), Some(big_neighbor)) = (neighbors.smaller, neighbors.bigger) {
            let intersection_point = small_neighbor.find_intersection(big_neighbor);
            if let Some(intersection) = intersection_point {
                self.add_intersection_event(intersection, small_neighbor, big_neighbor);
            }
        }
    }

    fn handle_intersection_event(&mut self, intersection: Intersection, smaller: Line2D, bigger: Line2D) {
        // small shift to the right to calculate order behind intersection
        let point = match intersection {
            Intersection::Crossing { point, .. } => { point }
            Intersection::Touching { point, .. } => { point }
            Intersection::PartialOverlap { overlap, .. } => { overlap.start }
            Intersection::ContainedOverlap { overlap, .. } => { overlap.start }
            Intersection::IdenticalOverlap { overlap, .. } => { overlap.start }
        };

        self.sweep_line.set_x(point.x.0 + self.options.x_shift);

        // add intersection to the list
        self.intersections.insert(intersection);

        let above = self.sweep_line.get_neighbors(&smaller).bigger;
        let below = self.sweep_line.get_neighbors(&bigger).smaller;


        // if intersection segE2 with segA
        if let Some(above) = above {
            let intersection_point = above.find_intersection(smaller);
            if let Some(intersection) = intersection_point {
                self.add_intersection_event(intersection, smaller, above);
            }
        }
        // if intersection segE1 with segB
        if let Some(below) = below {
            let intersection_point = below.find_intersection(bigger);
            if let Some(intersection) = intersection_point {
                self.add_intersection_event(intersection, below, bigger);
            }
        }
    }

    fn add_intersection_event(&mut self, intersection: Intersection, smaller: Line2D, bigger: Line2D) {
        match intersection {
            Intersection::Crossing { point, ..} => {
                if self.options.panic_on_identical_x {
                    if self.intersections.iter().any(|i| {
                        return match i {
                            // should be the only legal case
                            Intersection::Crossing { point: iter_point, .. } => {
                                iter_point.x == point.x
                            },
                            _ => false
                        }
                    }) {

                    }
                }

                if self.intersections.contains(&intersection) {
                    return;
                }

                self.queue.add(Event::IntersectionEvent { intersection, smaller, bigger });
            },
            // Error Handling Below
            Intersection::Touching { .. } => {
                if self.options.panic_on_touch {
                    panic!("Touching intersection detected {:?}", intersection);
                }
                eprintln!("Touching intersection detected {:?}", intersection);
                // todo: add it to the output, but not queue, or skip entirely?
                return;
            },
            _ => {
                if self.options.panic_on_overlap {
                    panic!("Overlapping intersection detected {:?}", intersection);
                }
                eprintln!("Overlapping intersection detected {:?}", intersection);
                // todo: add it to the output, but not queue, or skip entirely?
                return;
            }
        }
    }

    /// Performs sanity checks on the input lines
    /// Covers the following cases:
    ///   - lines with zero length -> remove them
    ///   - lines with identical x coordinates -> panic
    ///   - vertical lines -> panic
    /// Complexity: O(n)
    fn sanity_checks(lines: &mut Vec<Line2D>, options: &SweepLineOptions) -> Vec<Line2D> {
        let mut x_coords = HashSet::new();

        let res = lines.clone()
            .into_iter()
            .filter_map(|line| {
                if line.is_zero_length() {
                    if options.panic_on_zero_length {
                        panic!("Zero length line detected: {}", line);
                    } else {
                        eprintln!("Zero length line detected, ignoring line: {}", line);
                        return None;
                    }
                }
                if line.is_vertical() {
                    if options.panic_on_vertical {
                        panic!("Vertical line detected");
                    } else {
                        // eprintln!("Vertical line detected, shifting x coordinate slightly");
                        // line.end.x = line.end.x + options.x_shift;
                        eprintln!("Vertical line detected, ignoring line: {}", line);
                        return None;
                    }
                }

                if x_coords.contains(&(line.start.x)) {
                    if options.panic_on_identical_x {
                        panic!("Lines have identical x coordinates: {}", line.start.x);
                    } else {
                        // eprintln!("Duplicate x value detected, shifting start x coordinate of {} slightly", line);
                        // line.start.x = line.start.x + options.x_shift;
                        eprintln!("Duplicate x value detected, ignoring line: {}", line);
                        return None;
                    }
                }
                x_coords.insert(line.start.x);

                if x_coords.contains(&(line.end.x)) {
                    if options.panic_on_identical_x {
                        panic!("Lines have identical x coordinates: {}", line.end.x);
                    } else {
                        // eprintln!("Duplicate x value detected, shifting end x coordinate  of {} slightly", line);
                        // line.end.x = line.end.x + options.x_shift;
                        eprintln!("Duplicate x value detected, ignoring line: {}", line);
                        return None;
                    }
                }

                return if line.start.x > line.end.x {
                    Some(Line2D::new(*line.end.x, *line.end.y, *line.start.x, *line.start.y))
                } else {
                    Some(line)
                };
            }).collect();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;
    use crate::geometry::point::Point2D;

    fn get_options_to_panic() -> SweepLineOptions {
        SweepLineOptions::panic_enabled()
    }

    #[test]
    fn test_intersection_in_middle() {
        let line1 = Line2D::new(1.0, 1.0, 4.0, 4.0);
        let line2 = Line2D::new(2.0, 3.0, 3.0, 2.0);
        let mut handler = Handler::new(vec![line1, line2], get_options_to_panic());

        handler.run();

        let intersection = Intersection::Crossing { line1, line2, point: Point2D { x: OrderedFloat(2.5), y: OrderedFloat(2.5) } };

        assert!(handler.intersections.iter().any(|i| i == &intersection));
    }

    #[test]
    fn test_no_intersection() {
        let line1 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(3.0, 3.0, 4.0, 4.0);
        let mut handler = Handler::new(vec![line1, line2], get_options_to_panic());

        handler.run();

        assert!(handler.intersections.is_empty());
    }

    #[test]
    fn test_multiple_intersections() {
        let line1 = Line2D::new(0.0, 2.0, 10.0, 2.0);
        let line2 = Line2D::new(2.0, 0.0, 8.0, 6.0);
        let line3 = Line2D::new(5.0, 4.0, 9.0, 0.0);
        let mut handler = Handler::new(vec![line1, line2, line3], SweepLineOptions::panic_disabled());

        handler.run();

        let ip1 = Point2D { x: OrderedFloat(4.0), y: OrderedFloat(2.0) };
        let ip2 = Point2D { x: OrderedFloat(5.5), y: OrderedFloat(3.5) };
        let ip3 = Point2D { x: OrderedFloat(7.0), y: OrderedFloat(2.0) };

        let intersection1 = Intersection::Crossing { line1, line2, point: ip1 };
        let intersection2 = Intersection::Crossing { line1: line2, line2: line3, point: ip2 };
        let intersection3 = Intersection::Crossing { line1, line2: line3, point: ip3 };

        assert_eq!(handler.intersections.len(), 3);
        assert!(handler.intersections.iter().any(|i| i == &intersection1));
        assert!(handler.intersections.iter().any(|i| i == &intersection2));
        assert!(handler.intersections.iter().any(|i| i == &intersection3));
    }

    #[test]
    fn test_intersection_circle() {
        let line1 = Line2D::new(2.0, 1.0, 7.0, 1.0);
        let line2 = Line2D::new(6.0, 0.0, 8.0, 6.0);
        let line3 = Line2D::new(9.0, 5.0, 4.0, 7.0);
        let line4 = Line2D::new(5.0, 7.0, 0.0, 3.0);
        let line5 = Line2D::new(1.0, 4.0, 3.0, 0.0);

        let mut handler = Handler::new(vec![line1, line2, line3, line4, line5], get_options_to_panic());

        let intersections = handler.run();

        assert_eq!(intersections.len(), 5);
    }

    #[test]
    fn should_panic_when_lines_have_identical_x_coordinates() {
        let line1 = Line2D::new(0.0, 2.0, 0.0, 10.0);
        let line2 = Line2D::new(1.0, 0.0, 2.0, 6.0);
        let line3 = Line2D::new(5.0, 4.0, 9.0, 0.0);

        assert!(std::panic::catch_unwind(|| Handler::new(vec![line1, line2, line3], get_options_to_panic())).is_err());
    }

    #[test]
    fn should_panic_when_lines_are_vertical() {
        let line1 = Line2D::new(1.0, 0.0, 1.0, 2.0);
        let line2 = Line2D::new(0.0, 0.0, 2.0, 6.0);
        let line3 = Line2D::new(5.0, 4.0, 9.0, 0.0);

        assert!(std::panic::catch_unwind(|| Handler::new(vec![line1, line2, line3], get_options_to_panic())).is_err());
    }

    #[test]
    fn should_ignore_lines_with_length_zero() {
        let line1 = Line2D::new(0.0, 0.0, 0.0, 0.0);
        let line2 = Line2D::new(1.0, 0.0, 2.0, 6.0);
        let line3 = Line2D::new(5.0, 4.0, 9.0, 0.0);

        let handler = Handler::new(vec![line1, line2, line3], SweepLineOptions::panic_disabled());

        let queue = &handler.queue;

        // start and end events for line2 and line3
        assert_eq!(queue.len(), 4);
    }

    #[test]
    fn should_panic_when_multiple_intersections_at_same_point() {
        let line1 = Line2D::new(0.0, 0.0, 10.0, 10.0);
        let line2 = Line2D::new(9.0, 9.0, 1.0, 1.0);
        let line3 = Line2D::new(2.0, 5.0, 8.0, 5.0);

        assert!(std::panic::catch_unwind(|| {
            let mut handler = Handler::new(vec![line1, line2, line3], get_options_to_panic());
            handler.run();
        }).is_err());
    }
}
