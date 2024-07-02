use std::collections::BTreeSet;
use crate::geometry::line::Line2D;
use crate::geometry::sweep_line::events::Event;

pub struct EventQueue {
    events: BTreeSet<Event>,
}

impl EventQueue {
    pub fn new(lines: Vec<Line2D>) -> Self {
        let mut events = BTreeSet::new();
        for line in lines {
            let start = Event::StartEvent { line };
            let end = Event::EndEvent { line };
            events.insert(start);
            events.insert(end);
        }
        Self { events }
    }

    pub fn add(&mut self, event: Event) {
        self.events.insert(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_first()
    }

    pub fn contains(&self, event: &Event) -> bool {
        self.events.contains(event)
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

pub struct Neighbors {
    above: Option<Line2D>,
    below: Option<Line2D>,
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;
    use super::*;
    use crate::geometry::point::Point2D;

    fn create_test_lines() -> Vec<Line2D> {
        let p1 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(1.0) };
        let p2 = Point2D { x: OrderedFloat(2.5), y: OrderedFloat(2.0) };
        let p3 = Point2D { x: OrderedFloat(1.5), y: OrderedFloat(2.0) };
        let p4 = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(1.0) };
        let p5 = Point2D { x: OrderedFloat(2.5), y: OrderedFloat(1.5) };
        let p6 = Point2D { x: OrderedFloat(3.0), y: OrderedFloat(1.5) };

        vec![
            Line2D { start: p1, end: p2 },
            Line2D { start: p3, end: p4 },
            Line2D { start: p5, end: p6 },
        ]
    }

    #[test]
    fn test_event_queue_creation_should_contain_sorted_events() {
        let lines = create_test_lines();
        let event_queue = EventQueue::new(lines);

        assert_eq!(event_queue.events.len(), 6);  // Each line has a start and end event

        // Check if the events are sorted correctly
        let mut last_event: Option<&Event> = None;
        for event in &event_queue.events {
            if let Some(last) = last_event {
                assert!(*last <= *event);
            }
            last_event = Some(event);
        }
    }

    #[test]
    fn test_event_queue_add() {
        let lines = create_test_lines();
        let mut event_queue = EventQueue::new(lines);

        let new_event = Event::StartEvent {
            line: Line2D {
                start: Point2D { x: OrderedFloat(0.0), y: OrderedFloat(0.0) },
                end: Point2D { x: OrderedFloat(0.5), y: OrderedFloat(0.5) },
            },
        };
        event_queue.add(new_event);

        assert_eq!(event_queue.events.len(), 7);

        // Check if the events are sorted correctly after adding
        let mut last_event: Option<&Event> = None;
        for event in &event_queue.events {
            if let Some(last) = last_event {
                assert!(*last <= *event);
            }
            last_event = Some(event);
        }
    }

    #[test]
    fn test_event_queue_pop() {
        let lines = create_test_lines();
        let mut event_queue = EventQueue::new(lines);

        let first_event = event_queue.pop();
        assert!(first_event.is_some());
        assert_eq!(event_queue.events.len(), 5);

        // Ensure the event queue remains sorted after popping
        let mut last_event: Option<&Event> = None;
        for event in &event_queue.events {
            if let Some(last) = last_event {
                assert!(*last <= *event);
            }
            last_event = Some(event);
        }
    }

    #[test]
    fn test_event_queue_contains() {
        let lines = create_test_lines();
        let mut event_queue = EventQueue::new(lines);

        let event_to_check = event_queue.events.iter().next().cloned().unwrap();
        assert!(event_queue.contains(&event_to_check));

        event_queue.pop();  // Remove the event
        assert!(!event_queue.contains(&event_to_check));
    }

    #[test]
    fn test_event_queue_is_empty() {
        let lines = vec![];
        let event_queue = EventQueue::new(lines);

        assert!(event_queue.is_empty());
    }
}