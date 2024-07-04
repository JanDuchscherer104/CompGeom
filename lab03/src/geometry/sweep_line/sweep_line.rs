use std::cell::RefCell;
use std::cmp::Ordering;
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
        let y_self = self.value.y_at(self.x.borrow().clone());
        let y_other = other.value.y_at(other.x.borrow().clone());

        if y_self.is_none(){
            panic!("Cant calculate y value for the line {} at x = {}", self.value, self.x.borrow());
       }
        if y_other.is_none(){
            panic!("Cant calculate y value for the line {} at x = {}", other.value, self.x.borrow());
        }

        OrderedFloat::from(y_self.unwrap()).cmp(&OrderedFloat::from(y_other.unwrap()))
    }
}


#[derive(Debug)]
pub struct Neighbors {
    pub bigger: Option<Line2D>,
    pub smaller: Option<Line2D>,
}

#[derive(Debug)]
pub struct SweepLine {
    lines: Vec<OrderedLine>,
    x: Rc<RefCell<f64>>,
}

/// Implementation of SweepLine structure
/// Current implementation is not optimal as it sorts the lines every time x is updated
impl SweepLine {
    pub fn new() -> Self {
        SweepLine {
            lines: Vec::new(),
            x: Rc::new(RefCell::new(f64::NEG_INFINITY))
        }
    }

    pub fn set_x(&mut self, x: f64) {
        *self.x.borrow_mut() = x;
        self.lines.sort();
    }

    pub fn add(&mut self, line: Line2D) {
        self.lines.push(OrderedLine::new(line, self.x.clone()));
        self.lines.sort();
    }

    pub fn remove(&mut self, line: &Line2D) -> bool {
        if let Some(ordered_line) = self.find_ordered_line(line).cloned() {
            // get index of ordered line
            let index = self.lines.iter().position(|ol| ol == &ordered_line).unwrap();
            self.lines.remove(index);
            self.lines.sort();
            true
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

    pub fn print_lines(&self) {
        println!("Line Order @ {}", self.x.borrow());
        for ol in &self.lines {
            println!("\t{} @ {} = {}", ol.value, self.x.borrow(), ol.value.y_at(self.x.borrow().clone()).unwrap());
        }
    }
    
    pub fn get_sorted_lines(&self) -> Vec<OrderedLine> {
        self.lines.iter().cloned().collect()
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
    fn sweep_line_should_be_sorted() {
        let line1 = Line2D::new(0.0, 0.0, 10.0, 5.0);
        let line2 = Line2D::new(0.0, 2.0, 10.0, 2.0);
        let line3 = Line2D::new(0.0, 5.0, 10.0, 0.0);

        let mut sweep_line = SweepLine::new();
        sweep_line.set_x(0.0);

        sweep_line.add(line1.clone());
        sweep_line.add(line2.clone());
        sweep_line.add(line3.clone());

        let sorted_lines = sweep_line.get_sorted_lines();
        assert_eq!(sorted_lines[0].value, line1);
        assert_eq!(sorted_lines[1].value, line2);
        assert_eq!(sorted_lines[2].value, line3);

        sweep_line.set_x(4.1);
        let sorted_lines = sweep_line.get_sorted_lines();
        assert_eq!(sorted_lines[0].value, line2);
        assert_eq!(sorted_lines[1].value, line1);
        assert_eq!(sorted_lines[2].value, line3);

        sweep_line.set_x(5.1);
        let sorted_lines = sweep_line.get_sorted_lines();
        assert_eq!(sorted_lines[0].value, line2);
        assert_eq!(sorted_lines[1].value, line3);
        assert_eq!(sorted_lines[2].value, line1);

        sweep_line.set_x(6.1);
        let sorted_lines = sweep_line.get_sorted_lines();
        assert_eq!(sorted_lines[0].value, line3);
        assert_eq!(sorted_lines[1].value, line2);
        assert_eq!(sorted_lines[2].value, line1);
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
