use std::cmp::Ordering;
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
    pub p1: Point2D,
    pub p2: Point2D,
}

impl Line2D {
    pub fn from_f32_iter(nums: &mut impl Iterator<Item = f32>) -> Self {
        let p1 = Point2D::from_f32_iter(nums);
        let p2 = Point2D::from_f32_iter(nums);
        Line2D { p1, p2 }
    }
}

pub fn sweep_line(lines: &Vec<Line2D>) -> Vec<(Point2D, Point2D)> {
    let mut event_queue = BinaryHeap::<Point2D>::new();
    let mut intersections = Vec::new();

    for line in lines {
        event_queue.push(line.p1);
        event_queue.push(line.p2);
    }

    intersections
}

pub fn count_intersections(lines: &Vec<Line2D>) -> usize {
    let intersections = sweep_line(lines);
    intersections.len()
}

#[cfg(test)]
fn test() {
    let mut ep =
}
