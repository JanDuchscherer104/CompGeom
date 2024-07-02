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
    pub fn new(lines: Vec<Line2D>) -> Self {
        Handler { queue: EventQueue::new(lines), sweep_line: SweepLine::new(), intersections: HashSet::new() }
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
        self.intersections.insert(intersection);
        self.sweep_line.swap(&smaller, &bigger);

        let above = self.sweep_line.get_neighbors(&smaller).bigger;
        let below = self.sweep_line.get_neighbors(&bigger).smaller;

        if let Some(above) = above {
            let intersection_point = above.find_intersection(smaller);
            if let Some(intersection) = intersection_point {
                self.add_intersection_event(intersection, above, smaller);
            }
        }
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
        let line1 = Line2D::new(0.0, 2.0, 6.0, 2.0);
        let line2 = Line2D::new(1.0, 1.0, 4.0, 4.0);
        let line3 = Line2D::new(1.5, 3.5, 5.0, 0.0);
        let mut handler = Handler::new(vec![line1, line2, line3]);

        handler.run();

        let intersection1 = Intersection { point: Point2D { x: OrderedFloat(2.0), y: OrderedFloat(2.0) }, line1, line2 };
        let intersection2 = Intersection { point: Point2D { x: OrderedFloat(2.5), y: OrderedFloat(2.5) }, line1: line2, line2: line3 };
        let intersection3 = Intersection { point: Point2D { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, line1, line2: line3 };


        assert_eq!(handler.intersections.len(), 3);
        assert!(handler.intersections.iter().any(|i| i.point == intersection1.point));
        assert!(handler.intersections.iter().any(|i| i.point == intersection2.point));
        assert!(handler.intersections.iter().any(|i| i.point == intersection3.point));
    }
}