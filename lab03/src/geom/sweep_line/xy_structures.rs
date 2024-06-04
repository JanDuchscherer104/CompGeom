use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap},
};

use super::super::{line::Line2D, point::Point2D};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    Start,
    End,
    Intersection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub point: Point2D,
    pub event_type: EventType,
    pub segment: Option<Line2D>,       // TODO: necessary?, rm Option
    pub other_segment: Option<Line2D>, // TODO: necessary?
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // The smallest x-coordinate should have the highest priority
        other.point.partial_cmp(&self.point)
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// X-Structure: Maintains the global order of transitions of the sweep line.
/// Must support the following operations efficiently:
/// - insert a new event
/// - pop the event with the minimum x-coordinate, which is the next event to be processed
/// Appropriate data structures: priority queue or balanced binary search tree
/// Contains:
///    - all endpoints of future segments
///    - all endpoints (to the right of the SL) of active segments
///    - all intersections to the right of the SL of active segments
/// - sorted w.r.t. x-coordinates
pub struct XStructure {
    pub events: BinaryHeap<Event>,
}

impl XStructure {
    pub fn new() -> Self {
        XStructure {
            events: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ActiveSegment {
    pub segment: Line2D,
    pub y: f64, // y-coordinate of the intersection with the sweep line
}

impl Eq for ActiveSegment {}

impl Ord for ActiveSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.partial_cmp(&other.y).unwrap().reverse()
    }
}

/// Y-Structure: Maintains the set of line segments that are "currently" intersected by the sweep line.
/// Appropriate data structures: balanced binary search tree (e.g. AVL tree, red-black tree; sorted w.r.t. y-coordinates
/// of intersection points with the sweep line)
/// Events / Transitions:
/// - SL encounters the left endpoint of a line segment: insert into Y-Structure
/// - SL encounters the right endpoint of a line segment: remove from Y-Structure
/// - relative order of line segments in Y-Structure changes (intersection between two line segments): update Y-Structure
/// All operations cause a change in local neighborhoods of line segments in Y-Structure
pub struct YStructure {
    pub segments: BTreeSet<ActiveSegment>,
}

impl YStructure {
    pub fn new() -> Self {
        YStructure {
            segments: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, segment: ActiveSegment) {
        self.segments.insert(segment);
    }

    pub fn remove(&mut self, segment: &ActiveSegment) {
        self.segments.remove(segment);
    }

    pub fn successor(&self, segment: &ActiveSegment) -> Option<&ActiveSegment> {
        self.segments.range(segment..).nth(1)
    }

    pub fn predecessor(&self, segment: &ActiveSegment) -> Option<&ActiveSegment> {
        self.segments.range(..segment).next_back()
    }
}
