use std::collections::{BTreeSet, HashMap, HashSet};
use crate::geometry::line::Line2D;

#[derive(Debug)]
pub struct Neighbors {
    pub bigger: Option<Line2D>,
    pub smaller: Option<Line2D>,
}

#[derive(Debug)]
pub struct SweepLine {
    lines: BTreeSet<Line2D>,
    swaps: Swaps,
}

impl SweepLine {
    pub fn new() -> SweepLine {
        SweepLine { lines: BTreeSet::new(), swaps: Swaps::new() }
    }

    pub fn add(&mut self, line: Line2D) {
        println!("Adding line {} to sweepline", line);
        self.lines.insert(line);
    }

    pub fn remove(&mut self, line: &Line2D) {
        println!("Removing line {} from sweepline", line);
        self.lines.remove(line);
        self.swaps.remove(line);
    }

    pub fn get_neighbors(&self, line: &Line2D) -> Neighbors {
        let mut smaller = self.lines.range(..line).next_back().cloned();
        let mut bigger = self.lines.range(line..).nth(1).cloned();

        // consider swaps
        if bigger.is_some() {
            if let Some(swaps) = self.swaps.get_swaps(&bigger.unwrap()) {
            // bigger = smallest line in swaps
            for swap in swaps.iter() {
                    if swap < &bigger.unwrap() && swap != line {
                        bigger = Some(swap.clone());
                    }
                }
            }
        }

        if smaller.is_some() {
            if let Some(swaps) = self.swaps.get_swaps(&smaller.unwrap()) {
                // smaller = biggest line in swaps
                for swap in swaps.iter() {
                    if swap > &smaller.unwrap() && swap != line{
                        smaller = Some(swap.clone());
                    }
                }
            }
        }

        Neighbors { bigger, smaller }
    }

    pub fn swap(&mut self, line1: &Line2D, line2: &Line2D) {
        println!("Swapping lines {} and {}", line1, line2);
        self.swaps.add(line1.clone(), line2.clone())
    }
}

#[derive(Debug)]
struct Swaps {
    swaps: HashMap<Line2D, HashSet<Line2D>>,
}
impl Swaps {
    pub fn new() -> Self {
        Swaps {
            swaps: HashMap::new(),
        }
    }

    pub fn add(&mut self, line1: Line2D, line2: Line2D) {
        self.swaps.entry(line1).or_insert_with(HashSet::new).insert(line2);
        self.swaps.entry(line2).or_insert_with(HashSet::new).insert(line1);
    }

    pub fn remove(&mut self, line: &Line2D) {
        self.swaps.remove(&line);
        for (_, lines) in self.swaps.iter_mut() {
            lines.remove(&line);
        }
    }

    pub fn get_swaps(&self, line: &Line2D) -> Option<HashSet<Line2D>> {
        self.swaps.get(line).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_remove_lines() {
        let line1 = Line2D::new(1.0, 1.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 0.5, 1.5, 1.5);
        let mut sweepline = SweepLine {
            lines: BTreeSet::new(),
            swaps: Swaps::new(),
        };

        sweepline.add(line1);
        sweepline.add(line2);

        assert!(sweepline.lines.contains(&line1));
        assert!(sweepline.lines.contains(&line2));

        sweepline.remove(&line1);
        assert!(!sweepline.lines.contains(&line1));
        assert!(sweepline.lines.contains(&line2));
    }

    #[test]
    fn test_get_neighbors() {
        let line1 = Line2D::new(1.0, 0.0, 2.0, 2.0);
        let line2 = Line2D::new(0.5, 1.0, 1.5, 1.5);
        let line3 = Line2D::new(2.0, 2.0, 3.0, 3.0);
        let mut sweepline = SweepLine {
            lines: BTreeSet::new(),
            swaps: Swaps::new(),
        };

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
        let mut sweepline = SweepLine {
            lines: BTreeSet::new(),
            swaps: Swaps::new(),
        };

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
        let mut sweepline = SweepLine {
            lines: BTreeSet::new(),
            swaps: Swaps::new(),
        };

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
