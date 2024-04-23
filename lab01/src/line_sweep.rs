use std::collections::{BTreeSet, BinaryHeap};

use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2D {
    /// Ord: Lexicographic ordering w.r.t. x, and then y!
    pub x: OrderedFloat<f32>,
    pub y: OrderedFloat<f32>,
}

impl Point2D {
    pub fn from_f32_iter(nums: &mut impl Iterator<Item = f32>) -> Self {
        Point2D {
            x: OrderedFloat(nums.next().unwrap()),
            y: OrderedFloat(nums.next().unwrap()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line2D {
    pub p1: Point2D, // should be geq p2
    pub p2: Point2D,
}

impl Line2D {
    pub fn from_f32_iter(nums: &mut impl Iterator<Item = f32>) -> Self {
        let p1 = Point2D::from_f32_iter(nums);
        let p2 = Point2D::from_f32_iter(nums);

        // Ensure p1 is lexicographically greater than p2
        if p1 > p2 {
            Line2D { p1, p2 }
        } else {
            Line2D { p1: p2, p2: p1 }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub point: Point2D,
    pub line: &Line2D,
    pub is_start: bool,
}

pub fn sweep_line(lines: &Vec<Line2D>) -> Vec<(Point2D, Point2D)> {
    // event_queue: lexicographically ordered ~ Top-Left to Bottom-Right
    let mut event_queue = BinaryHeap::<Event>::new();

    // segment_list: ordered set of segments
    let mut segment_list = BTreeSet::<Line2D>::new();

    let mut intersections = Vec::new();

    for line in lines {
        event_queue.push(Event {
            point: line.p1,
            line: &line,
            is_start: true,
        });
        event_queue.push(Event {
            point: line.p2,
            line: &line,
            is_start: false,
        });
    }

    // while !event_queue.is_empty() {
    //     let p = event_queue.pop().unwrap();
    // }

    intersections
}

pub fn count_intersections(lines: &Vec<Line2D>) -> usize {
    let intersections = sweep_line(lines);
    intersections.len()
}
