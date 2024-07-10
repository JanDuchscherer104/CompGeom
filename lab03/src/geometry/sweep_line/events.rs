use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::geometry::intersection::Intersection;
use crate::geometry::line::Line2D;
use crate::geometry::point::Point2D;

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
        match (self, other) {
            (Event::StartEvent { line: line1 }, Event::StartEvent { line: line2 }) => line1 == line2,
            (Event::EndEvent { line: line1 }, Event::EndEvent { line: line2 }) => line1 == line2,
            (Event::IntersectionEvent { intersection: i1, .. }, Event::IntersectionEvent { intersection: i2, .. }) => i1 == i2,
            _ => false,
        }
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
            (Event::StartEvent { line: start }, Event::IntersectionEvent { intersection: i1, .. }) => start.start.cmp(get_point_of_intersection(i1)),
            (Event::EndEvent { line: line1 }, Event::EndEvent { line: line2 }) => line1.end.cmp(&line2.end),
            (Event::EndEvent { line: end }, Event::StartEvent { line: start }) => end.end.cmp(&start.start),
            (Event::EndEvent { line: end }, Event::IntersectionEvent { intersection: i1, .. }) => end.end.cmp(get_point_of_intersection(i1)),
            (Event::IntersectionEvent { intersection: i1, .. }, Event::IntersectionEvent { intersection: i2, .. }) => {
                println!("Comparing {} with {}", i1, i2);
                get_point_of_intersection(i1).cmp(get_point_of_intersection(i2))
            },
            (Event::IntersectionEvent { intersection: i1, .. }, Event::StartEvent { line: start }) => get_point_of_intersection(i1).cmp(&start.start),
            (Event::IntersectionEvent { intersection: i1, .. }, Event::EndEvent { line: end }) => get_point_of_intersection(i1).cmp(&end.end),
        }
    }
}

impl Hash for Event {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Event::StartEvent { line } => {
                line.start.hash(state);
                line.end.hash(state);
            }
            Event::EndEvent { line } => {
                line.start.hash(state);
                line.end.hash(state);
            }
            Event::IntersectionEvent { intersection, .. } => {
                intersection.hash(state);
            }
        }
    }
}


fn get_point_of_intersection(intersection: &Intersection) -> &Point2D {
    match intersection {
        Intersection::Touching { line1: _line1, line2: _line2, point } => {
            point
        },
        Intersection::Crossing { line1: _line1, line2: _line2, point } => {
            point
        },
        Intersection::ContainedOverlap { line1: _line1, line2: _line2, overlap } => {
            &overlap.start
        },
        Intersection::IdenticalOverlap { line1: _, line2: _, overlap } => {
            &overlap.start
        },
        Intersection::PartialOverlap { line1: _line1, line2: _line2, overlap } => {
            &overlap.start
        },
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
                write!(f, "IntersectionEvent between at x={}\t| Lines: {} and {}", get_point_of_intersection(intersection).x, smaller, bigger)
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

        let i1 = Intersection::PartialOverlap { line1: l1, line2: l2, overlap: l1 };

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
        let l1 = Line2D::new(1.0, 1.0, 2.5, 2.0);
        let l2 = Line2D::new(1.5, 2.0, 2.0, 1.0);
        let l3 = Line2D::new(2.5, 1.5, 3.0, 1.5);

        let ip = Point2D { x: OrderedFloat(1.75), y: OrderedFloat(1.5) }; // not correct, but doesn't matter for the test

        let i2 = Intersection::Crossing { line1: l1, line2: l2, point: ip};

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