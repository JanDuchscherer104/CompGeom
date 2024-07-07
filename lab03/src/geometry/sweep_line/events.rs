use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::geometry::intersection::Intersection;
use crate::geometry::line::Line2D;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    StartEvent {
        line: Line2D,
    },
    EndEvent {
        line: Line2D,
    },
    IntersectionEvent {
        bigger: Line2D,
        smaller: Line2D,
        intersection: Intersection,
    },
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Event::StartEvent { line: line1 }, Event::StartEvent { line: line2 }) => line1.start.cmp(&line2.start),
            (Event::StartEvent { line: start}, Event::EndEvent { line: end }) => start.start.cmp(&end.end),
            (Event::StartEvent { line: start }, Event::IntersectionEvent { intersection: i1, .. }) => start.start.cmp(&i1.point),
            (Event::EndEvent { line: line1 }, Event::EndEvent { line: line2 }) => line1.end.cmp(&line2.end),
            (Event::EndEvent { line: end }, Event::StartEvent { line: start }) => end.end.cmp(&start.start),
            (Event::EndEvent { line: end }, Event::IntersectionEvent { intersection: i1, .. }) => end.end.cmp(&i1.point),
            (Event::IntersectionEvent { intersection: i1, .. }, Event::IntersectionEvent { intersection: i2, .. }) => i1.point.cmp(&i2.point),
            (Event::IntersectionEvent { intersection: i1, .. }, Event::StartEvent { line: start }) => i1.point.cmp(&start.start),
            (Event::IntersectionEvent { intersection: i1, .. }, Event::EndEvent { line: end }) => i1.point.cmp(&end.end),
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::StartEvent { line } => {
                write!(f, "StartEvent at x={}\t| Line: {}", line.start.x, line)
            },
            Event::EndEvent { line } => {
                write!(f,"EndEvent at x={}\t| Line: {}", line.end.x, line)

            },
            Event::IntersectionEvent { intersection, smaller, bigger } => {
                write!(f, "IntersectionEvent between at x={}\t| Lines: {} and {}", intersection.point.x, smaller, bigger)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;
    use crate::geometry::point::Point2D;
    use super::*;

    #[test]
    fn test_event_equality() {
        let p1 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(2.0) };
        let p2 = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(3.0) };

        let l1 = Line2D { start: p1, end: p2 };
        let l2 = Line2D { start: p2, end: p1 };

        let i1 = Intersection { line1: l1, line2: l2, point: p1 };

        let se1 = Event::StartEvent { line: l1 };
        let se2 = Event::StartEvent { line: l1 };

        let ee1 = Event::EndEvent { line: l2 };
        let ee2 = Event::EndEvent { line: l2 };

        let ie1 = Event::IntersectionEvent { bigger: l1, smaller: l2, intersection: i1 };
        let ie2 = Event::IntersectionEvent { bigger: l1, smaller: l2, intersection: i1 };

        assert_eq!(se1, se2);
        assert_eq!(ee1, ee2);
        assert_eq!(ie1, ie2);
    }

    #[test]
    fn test_event_ordering() {
        let p1 = Point2D { x: OrderedFloat(1.0), y: OrderedFloat(1.0) };
        let p2 = Point2D { x: OrderedFloat(2.5), y: OrderedFloat(2.0) };
        let p3 = Point2D { x: OrderedFloat(1.5), y: OrderedFloat(2.0) };
        let p4 = Point2D { x: OrderedFloat(2.0), y: OrderedFloat(1.0) };
        let p5 = Point2D { x: OrderedFloat(2.5), y: OrderedFloat(1.5) };
        let p6 = Point2D { x: OrderedFloat(3.0), y: OrderedFloat(1.5) };

        let l1 = Line2D { start: p1, end: p2 };
        let l2 = Line2D { start: p3, end: p4 };
        let l3 = Line2D { start: p5, end: p6 };

        let ip = Point2D { x: OrderedFloat(1.75), y: OrderedFloat(1.5) };


        let i2 = Intersection { line1: l1, line2: l2, point: ip};

        let e1 = Event::StartEvent { line: l1 }; // (1.0, 1.0)
        let e2 = Event::EndEvent { line: l1 }; // (2.5, 2.0)

        let e3 = Event::StartEvent { line: l2 }; // (1.5, 2.0)
        let e4 = Event::EndEvent { line: l2 }; // (2.0, 1.0)

        let e5 = Event::StartEvent { line: l3 } ; // (2.5, 1.5)
        let e6 = Event::EndEvent { line: l3 }; // (3.0, 1.5)

        let e7 = Event::IntersectionEvent { bigger: l2, smaller: l1, intersection: i2 }; // (1.75, 1.5)


        let mut events = vec![e6, e2, e1, e7, e4, e3, e5];
        events.sort();

        assert!(e1 < e2);
        assert!(e3 < e7);
        assert!(e7 < e4);
        assert!(e4 < e5);
        assert!(e5 < e2);
        assert!(e2 < e6);

        assert_eq!(events, vec![e1, e3, e7, e4, e5, e2, e6]);
    }
}