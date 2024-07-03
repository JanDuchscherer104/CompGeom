use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;
use crate::geometry::line::Line2D;

#[derive(Debug)]
struct OrderedLine {
    value: Line2D,
    swap_target:  Option<Rc<RefCell<OrderedLine>>>,
}

impl OrderedLine {

    pub fn new(value: Line2D) -> OrderedLine {
        OrderedLine {
            value,
            swap_target: None,
        }
    }

    pub fn set_swap_target(&mut self, target: Rc<RefCell<OrderedLine>>) {
        self.swap_target = Some(target);
    }

    fn get_value(&self) -> Line2D {
        let mut visited = HashSet::new();
        self.get_value_recursive(&mut visited)
    }

    fn get_value_recursive(&self, visited: &mut HashSet<Line2D>) -> Line2D {

        if !visited.insert(self.value) {

            panic!("Cycle detected in swap chain");
        }

        if let Some(ref target) = self.swap_target {
            // if the target has already been visited, return this value
            if visited.contains(&(target.borrow().value)) {
                println!("Next value is already visited, returning current value");
                return self.value.clone();
            } else {
                target.borrow().get_value_recursive(visited)
            }
        } else {
            self.value.clone()
        }
    }
}


impl PartialEq for OrderedLine {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for OrderedLine {}

impl PartialOrd for OrderedLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedLine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
        // match (&self.swap_target, &other.swap_target) {
        //     (Some(target), Some(other_target)) => target.borrow().cmp(&other_target.borrow()),
        //     (Some(target), None) => target.borrow().get_value().cmp(&other.value),
        //     (None, Some(other_target)) => self.value.cmp(&other_target.borrow().get_value()),
        //     (None, None) => self.value.cmp(&other.value),
        // }
    }
}


#[derive(Debug)]
pub struct Neighbors {
    pub bigger: Option<Line2D>,
    pub smaller: Option<Line2D>,
}

#[derive(Debug)]
pub struct SweepLine {
    lines: BTreeSet<Rc<RefCell<OrderedLine>>>,
}

impl SweepLine {
    pub fn new() -> Self {
        SweepLine {
            lines: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, line: Line2D) {
        let ordered_line = Rc::new(RefCell::new(OrderedLine::new(line)));
        self.lines.insert(ordered_line);
    }

    pub fn remove(&mut self, line: &Line2D) {
        let remove = self.find_ordered_line(line);
        if remove.is_some() {
            self.lines.remove(&remove.unwrap());
        }
    }

    pub fn swap(&mut self, line1: &Line2D, line2: &Line2D) {
        let ordered1 = self.find_ordered_line(line1)
            .expect("Can't swap the lines, because line1 is not in the sweepline");
        let ordered2 = self.find_ordered_line(line2)
            .expect("Can't swap the lines, because line2 is not in the sweepline");

        ordered1.borrow_mut().set_swap_target(ordered2.clone());
        ordered2.borrow_mut().set_swap_target(ordered1.clone());

    }

    pub fn get_neighbors(&self, line: &Line2D) -> Neighbors {
        let mut neighbors = Neighbors {
            bigger: None,
            smaller: None,
        };

        let mut iter = self.lines.iter();
        while let Some(ol) = iter.next() {
            if ol.borrow().get_value() == *line {
                if let Some(prev) = iter.next_back() {
                    neighbors.bigger = Some(prev.borrow().value);
                }
                if let Some(next) = iter.next() {
                    neighbors.smaller = Some(next.borrow().value);
                }
                break;
            }
        }

        neighbors
    }

    pub fn contains(&self, line: Line2D) -> bool {
        self.lines.iter().any(|ol| ol.borrow().value == line)
    }

    /// Find the ordered line to the line.
    /// Returns a reference so that the underlying OrderedLine can be modified.
    fn find_ordered_line(&self, line: &Line2D) -> Option<Rc<RefCell<OrderedLine>>> {
        self.lines.iter().find(|ol| ol.borrow().value == *line).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_line_should_get_value_when_swapped() {
        let line1 = Line2D::new(1.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 1.0, 1.5, 1.5);
        let line3 = Line2D::new(2.0, 2.0, 3.0, 3.0);

        let ordered1 =  Rc::new(RefCell::new(OrderedLine::new(line1)));
        let ordered2 =  Rc::new(RefCell::new(OrderedLine::new(line2)));
        let ordered3 =  Rc::new(RefCell::new(OrderedLine::new(line3)));

        ordered1.borrow_mut().set_swap_target(ordered3.clone());

        assert_eq!(ordered1.borrow().get_value(), line3);
    }

    #[test]
    fn ordered_line_sort_no_swaps() {
        let line1 = Line2D::new(1.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 1.0, 1.5, 1.5);
        let line3 = Line2D::new(2.0, 2.0, 3.0, 3.0);

        let ordered1 = OrderedLine::new(line1);
        let ordered2 = OrderedLine::new(line2);
        let ordered3 = OrderedLine::new(line3);

        assert!(ordered1 < ordered2);
        assert!(ordered2 < ordered3);
    }

    #[test]
    fn ordered_line_sort_swapped() {
        let line1 = Line2D::new(1.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 1.0, 1.5, 1.5);
        let line3 = Line2D::new(2.0, 2.0, 3.0, 3.0);

        let ordered1 =  Rc::new(RefCell::new(OrderedLine::new(line1)));
        let ordered2 =  Rc::new(RefCell::new(OrderedLine::new(line2)));
        let ordered3 =  Rc::new(RefCell::new(OrderedLine::new(line3)));

        ordered1.borrow_mut().set_swap_target(ordered3.clone());
        ordered3.borrow_mut().set_swap_target(ordered1.clone());

        assert_eq!(ordered1.borrow().get_value(), line3);
        assert_eq!(ordered2.borrow().get_value(), line2);
        assert_eq!(ordered3.borrow().get_value(), line1);

        assert!(ordered3 < ordered2);
        assert!(ordered3 < ordered1);
        assert!(ordered2 < ordered1);
    }

    #[test]
    fn test_add_and_remove_lines() {
        let line1 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 0.5, 1.5, 1.5);
        let mut sweepline = SweepLine::new();

        sweepline.add(line1);
        sweepline.add(line2);

        assert!(sweepline.contains(line1));
        assert!(sweepline.contains(line2));

        sweepline.remove(&line1);
        assert!(!sweepline.contains(line1));
        assert!(sweepline.contains(line2));
    }

    #[test]
    fn test_get_neighbors() {
        let line1 = Line2D::new(1.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 1.0, 1.5, 1.5);
        let line3 = Line2D::new(2.0, 2.0, 3.0, 3.0);
        let mut sweepline = SweepLine::new();

        sweepline.add(line1);
        sweepline.add(line2);
        sweepline.add(line3);

        let neighbors1 = sweepline.get_neighbors(&line1);
        assert_eq!(neighbors1.smaller, None);
        assert_eq!(neighbors1.bigger, Some(line2));

        let neighbors2 = sweepline.get_neighbors(&line2);
        assert_eq!(neighbors2.smaller, Some(line1));
        assert_eq!(neighbors2.bigger, Some(line3));

        let neighbors3 = sweepline.get_neighbors(&line3);
        assert_eq!(neighbors3.smaller, Some(line2));
        assert_eq!(neighbors3.bigger, None);
    }

    #[test]
    fn test_swapped_neighbors() {
        let line1 = Line2D::new(1.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 1.0, 1.5, 1.5);
        let line3 = Line2D::new(2.0, 2.0, 3.0, 3.0);
        let mut sweepline = SweepLine::new();

        sweepline.add(line1);
        sweepline.add(line2);
        sweepline.add(line3);

        sweepline.swap(&line1, &line3);


        let neighbors = sweepline.get_neighbors(&line2);
        assert_eq!(neighbors.smaller, Some(line3));
        assert_eq!(neighbors.bigger, Some(line1));
    }

    #[test]
    fn test_remove_swaps() {
        let line1 = Line2D::new(1.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 1.0, 1.5, 1.5);
        let line3 = Line2D::new(2.0, 2.0, 3.0, 3.0);
        let mut sweepline = SweepLine::new();

        sweepline.add(line1);
        sweepline.add(line2);
        sweepline.add(line3);

        sweepline.swap(&line1, &line3);


        let neighbors = sweepline.get_neighbors(&line2);
        assert_eq!(neighbors.smaller, Some(line3));
        assert_eq!(neighbors.bigger, Some(line1));

        sweepline.remove(&line3);

        let neighbors = sweepline.get_neighbors(&line2);
        assert_eq!(neighbors.smaller, Some(line1));
        assert_eq!(neighbors.bigger, None);
    }

    #[test]
    fn test_line_itself_swapped() {
        // todo!()
    }

    #[test]
    fn test_nested_swaps() {
        // todo!()
    }
}
