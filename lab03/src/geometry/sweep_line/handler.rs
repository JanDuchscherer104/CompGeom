use std::collections::HashSet;
use crate::geometry::intersection::Intersection;
use crate::geometry::line::Line2D;
use crate::geometry::point::Point2D;
use crate::geometry::sweep_line::event_queue::EventQueue;
use crate::geometry::sweep_line::events::Event;
use crate::geometry::sweep_line::sweep_line::SweepLine;

struct Handler {
    queue: EventQueue,
    sweep_line: SweepLine,
    intersections: HashSet<Intersection>,
}

impl Handler {
    pub fn new(mut lines: Vec<Line2D>) -> Self {
        Self::sanity_checks(&mut lines);
        let handler = Handler { queue: EventQueue::new(lines.clone()), sweep_line: SweepLine::new(), intersections: HashSet::new() };

        handler
    }

    pub fn run(&mut self) {
        while !self.queue.is_empty() {
            let event = self.queue.pop();
            if let Some(event) = event {
                self.handle_event(event);
            }
        }
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
        self.sweep_line.set_x(*intersection.point.x + f64::EPSILON + 0.1);

        // add intersection to the list
        self.intersections.insert(intersection);


        let above = self.sweep_line.get_neighbors(&smaller).bigger;
        let below = self.sweep_line.get_neighbors(&bigger).smaller;

        // segE1: bigger
        // segE2: smaller
        // segA: above
        // segB: below

        // if intersection segE2 with segA
        if let Some(above) = above {
            let intersection_point = above.find_intersection(smaller);
            if let Some(intersection) = intersection_point {
                self.add_intersection_event(intersection, above, smaller);
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

    fn add_intersection_event(&mut self, point: Point2D, smaller: Line2D, bigger: Line2D) {
        let intersection = Intersection { point, line1: smaller, line2: bigger};

        if self.intersections.contains(&intersection) {
            return;
        }

        self.queue.add(Event::IntersectionEvent { intersection, smaller, bigger });
    }

    /// Performs sanity checks on the input lines
    /// Covers the following cases:
    ///   - lines with zero length -> remove them
    ///   - lines with identical x coordinates -> panic
    ///   - vertical lines -> panic
    /// Complexity: O(n)
    fn sanity_checks(lines: &mut Vec<Line2D>) {
        let mut x_coords = HashSet::new();

        lines.retain(|line| {
            if line.is_zero_length() {
                return false;
            }

            if x_coords.contains(&(line.start.x)) {
                panic!("Lines have identical x coordinates");
            } else {
                x_coords.insert(line.start.x);
            }
            if x_coords.contains(&(line.end.x)) {
                panic!("Lines have identical x coordinates");
            } else {
                x_coords.insert(line.end.x);
            }
            return true;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_intersection_in_middle() {
        let line1 = Line2D::new(1.0, 1.0, 4.0, 4.0);
        let line2 = Line2D::new(2.0, 3.0, 3.0, 2.0);
        let mut handler = Handler::new(vec![line1, line2]);

        handler.run();
        let intersection = Point2D { x: OrderedFloat(2.5), y: OrderedFloat(2.5) };

        assert!(handler.intersections.iter().any(|i| i.point == intersection));
    }

    #[test]
    fn test_no_intersection() {
        let line1 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(3.0, 3.0, 4.0, 4.0);
        let mut handler = Handler::new(vec![line1, line2]);

        handler.run();

        assert!(handler.intersections.is_empty());
    }

    #[test]
    fn test_multiple_intersections() {
        let line1 = Line2D::new(0.0, 2.0, 10.0, 2.0);
        let line2 = Line2D::new(2.0, 0.0, 8.0, 6.0);
        let line3 = Line2D::new(5.0, 4.0, 9.0, 0.0);
        let mut handler = Handler::new(vec![line1, line2, line3]);

        handler.run();

        let intersection1 = Intersection { point: Point2D { x: OrderedFloat(4.0), y: OrderedFloat(2.0) }, line1, line2 };
        let intersection2 = Intersection { point: Point2D { x: OrderedFloat(5.5), y: OrderedFloat(3.5) }, line1: line2, line2: line3 };
        let intersection3 = Intersection { point: Point2D { x: OrderedFloat(7.0), y: OrderedFloat(2.0) }, line1, line2: line3 };


        assert_eq!(handler.intersections.len(), 3);
        assert!(handler.intersections.iter().any(|i| i.point == intersection1.point));
        assert!(handler.intersections.iter().any(|i| i.point == intersection2.point));
        assert!(handler.intersections.iter().any(|i| i.point == intersection3.point));
    }

    #[test]
    fn should_panic_when_lines_have_identical_x_coordinates() {
        let line1 = Line2D::new(0.0, 2.0, 0.0, 10.0);
        let line2 = Line2D::new(1.0, 0.0, 2.0, 6.0);
        let line3 = Line2D::new(5.0, 4.0, 9.0, 0.0);

        assert!(std::panic::catch_unwind(|| Handler::new(vec![line1, line2, line3])).is_err());
    }

    #[test]
    fn should_panic_when_lines_are_vertical() {
        let line1 = Line2D::new(1.0, 0.0, 1.0, 2.0);
        let line2 = Line2D::new(0.0, 0.0, 2.0, 6.0);
        let line3 = Line2D::new(5.0, 4.0, 9.0, 0.0);

        assert!(std::panic::catch_unwind(|| Handler::new(vec![line1, line2, line3])).is_err());
    }

    #[test]
    fn should_ignore_lines_with_length_zero() {
        let line1 = Line2D::new(0.0, 0.0, 0.0, 0.0);
        let line2 = Line2D::new(1.0, 0.0, 2.0, 6.0);
        let line3 = Line2D::new(5.0, 4.0, 9.0, 0.0);

        let mut handler = Handler::new(vec![line1, line2, line3]);

        let queue = &handler.queue;

        // start and end events for line2 and line3
        assert_eq!(queue.len(), 4);
    }
}
