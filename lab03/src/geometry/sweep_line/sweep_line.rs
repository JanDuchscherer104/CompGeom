use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::Rc;
use ordered_float::OrderedFloat;
use crate::geometry::line::Line2D;


#[derive(Debug, Clone)]
struct OrderedLine {
    value: Line2D,
    x: Rc<RefCell<f64>>,
}

impl OrderedLine {
    pub fn new(value: Line2D, x: Rc<RefCell<f64>>) -> Self {
        OrderedLine {
            value,
            x,
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
        let y_self = OrderedFloat::from(self.value.y_at(self.x.borrow().clone()).expect("Can't calculate y at x"));
        let y_other = OrderedFloat::from(other.value.y_at(other.x.borrow().clone()).expect("Cant calculate y at x"));

        y_self.cmp(&y_other)
    }
}


#[derive(Debug)]
pub struct Neighbors {
    pub bigger: Option<Line2D>,
    pub smaller: Option<Line2D>,
}

#[derive(Debug)]
pub struct SweepLine {
    lines: BTreeSet<OrderedLine>,
    x: Rc<RefCell<f64>>,
}

impl SweepLine {
    pub fn new() -> Self {
        SweepLine {
            lines: BTreeSet::new(),
            x: Rc::new(RefCell::new(f64::NEG_INFINITY))
        }
    }

    pub fn set_x(&mut self, x: f64) {
        *self.x.borrow_mut() = x;
    }

    pub fn add(&mut self, line: Line2D) {
        self.lines.insert(OrderedLine::new(line, self.x.clone()));
    }

    pub fn remove(&mut self, line: &Line2D) -> bool {
        if let Some(ordered_line) = self.find_ordered_line(line).cloned() {
            self.lines.remove(&ordered_line)
        } else {
            false
        }
    }

    pub fn get_neighbors(&self, line: &Line2D) -> Neighbors {
        let ordered_line = OrderedLine::new(line.clone(), Rc::clone(&self.x));

        let mut smaller = None;
        let mut bigger = None;
        let mut found = false;

        for ol in self.lines.iter() {
            if ol == &ordered_line {
                found = true;
                continue;
            }

            if found {
                bigger = Some(ol.clone().value);
                break;
            } else {
                smaller = Some(ol.clone().value);
            }
        }

        Neighbors { smaller, bigger }
    }

    pub fn contains(&self, line: &Line2D) -> bool {
        let ordered_line = OrderedLine::new(line.clone(), self.x.clone());
        self.lines.contains(&ordered_line)
    }

    fn find_ordered_line(&self, line: &Line2D) -> Option<&OrderedLine> {
        self.lines.iter().find(|ol| ol.value == *line)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_line_should_be_ordered_when_y_static() {
        let line1 = Line2D::new(0.0, 1.0, 0.0, 1.0);
        let line2 = Line2D::new(0.0, 2.0, 0.0, 2.0);
        let line3 = Line2D::new(0.0, 3.0, 0.0, 3.0);

        let x = Rc::new(RefCell::new(0.0));
        let ordered_line1 = OrderedLine::new(line1, x.clone());
        let ordered_line2 = OrderedLine::new(line2, x.clone());
        let ordered_line3 = OrderedLine::new(line3, x.clone());

        assert!(ordered_line1 < ordered_line2);
        assert!(ordered_line2 < ordered_line3);
    }

    #[test]
    fn ordered_line_should_be_ordered_according_to_x() {
        let line1 = Line2D::new(0.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.0, 2.0, 2.0, 0.0);

        let x = Rc::new(RefCell::new(0.0));
        let ordered_line1 = OrderedLine::new(line1, x.clone());
        let ordered_line2 = OrderedLine::new(line2, x.clone());

        assert!(ordered_line1 < ordered_line2);

        *x.borrow_mut() = 2.0;

        assert!(ordered_line1 > ordered_line2);
    }


    #[test]
    fn add_and_remove_lines() {
        let line1 = Line2D::new(0.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(0.0, 0.5, 2.0, 1.5);
        let mut sweepline = SweepLine::new();
        sweepline.set_x(0.0);

        sweepline.add(line1);
        sweepline.add(line2);

        assert!(sweepline.contains(&line1));
        assert!(sweepline.contains(&line2));

        sweepline.remove(&line1);
        assert!(!sweepline.contains(&line1));
        assert!(sweepline.contains(&line2));
    }

    #[test]
    fn get_neighbors() {
        let line1 = Line2D::new(0.0, 0.0, 2.0, 0.0);
        let line2 = Line2D::new(0.0, 1.0, 2.0, 1.0);
        let line3 = Line2D::new(0.0, 2.0, 2.0, 2.0);
        let mut sweepline = SweepLine::new();
        sweepline.set_x(0.0);

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
    fn get_neighbours_behind_intersection() {
        let line1 = Line2D::new(0.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.0, 2.0, 2.0, 0.0);
        let line3 = Line2D::new(0.0, 3.0, 2.0, 3.0);

        let mut sweepline = SweepLine::new();
        sweepline.set_x(1.0 + f64::EPSILON);

        sweepline.add(line1);
        sweepline.add(line2);
        sweepline.add(line3);

        let neighbors1 = sweepline.get_neighbors(&line1);
        assert_eq!(neighbors1.smaller, Some(line2));
        assert_eq!(neighbors1.bigger, Some(line3));

        let neighbors2 = sweepline.get_neighbors(&line2);
        assert_eq!(neighbors2.smaller, None);
        assert_eq!(neighbors2.bigger, Some(line1));

        let neighbors3 = sweepline.get_neighbors(&line3);
        assert_eq!(neighbors3.smaller, Some(line1));
    }
}
