use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::geometry::line::Line2D;
use crate::geometry::point::Point2D;

#[derive(Copy, Clone, Debug)]
pub enum Intersection {
    Crossing { line1: Line2D, line2: Line2D, point: Point2D },
    Touching { line1: Line2D, line2: Line2D, point: Point2D },
    PartialOverlap { line1: Line2D, line2: Line2D, overlap: Line2D },
    ContainedOverlap { line1: Line2D, line2: Line2D, overlap: Line2D },
    IdenticalOverlap { line1: Line2D, line2: Line2D, overlap: Line2D },
}

fn lines_equal_unordered(line1: &Line2D, line2: &Line2D, other1: &Line2D, other2: &Line2D) -> bool {
    (line1 == other1 && line2 == other2) || (line1 == other2 && line2 == other1)
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Intersection::Crossing { line1: l1a, line2: l2a, point: pa },
                Intersection::Crossing { line1: l1b, line2: l2b, point: pb }
            ) => lines_equal_unordered(l1a, l2a, l1b, l2b) && pa == pb,
            (
                Intersection::Touching { line1: l1a, line2: l2a, point: pa },
                Intersection::Touching { line1: l1b, line2: l2b, point: pb }
            ) => lines_equal_unordered(l1a, l2a, l1b, l2b) && pa == pb,

            (
                Intersection::PartialOverlap { line1: l1a, line2: l2a, overlap: oa },
                Intersection::PartialOverlap { line1: l1b, line2: l2b, overlap: ob }
            ) => lines_equal_unordered(l1a, l2a, l1b, l2b) && oa == ob,

            (
                Intersection::ContainedOverlap { line1: l1a, line2: l2a, overlap: oa },
                Intersection::ContainedOverlap { line1: l1b, line2: l2b, overlap: ob }
            ) => lines_equal_unordered(l1a, l2a, l1b, l2b) && oa == ob,

            (
                Intersection::IdenticalOverlap { line1: l1a, line2: l2a, overlap: oa },
                Intersection::IdenticalOverlap { line1: l1b, line2: l2b, overlap: ob }
            ) => lines_equal_unordered(l1a, l2a, l1b, l2b) && oa == ob,
            _ => false,
        }
    }
}

impl Eq for Intersection {}

impl Hash for Intersection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (line1, line2) = match self {
            Intersection::Crossing { line1, line2, point }
            | Intersection::Touching { line1, line2, point } => {
                point.hash(state);
                if line1 <= line2 {
                    (line1, line2)
                } else {
                    (line2, line1)
                }
            }
            Intersection::PartialOverlap { line1, line2, overlap }
            | Intersection::ContainedOverlap { line1, line2, overlap }
            | Intersection::IdenticalOverlap { line1, line2, overlap } => {
                overlap.hash(state);
                if line1 <= line2 {
                    (line1, line2)
                } else {
                    (line2, line1)
                }
            }
        };
        line1.hash(state);
        line2.hash(state);
    }
}

impl Display for Intersection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Intersection::Crossing { line1, line2, point } => {
                write!(f, "Crossing: line1: {:?}, line2: {:?}, point: {:?}", line1, line2, point)
            }
            Intersection::Touching { line1, line2, point } => {
                write!(f, "Touching: line1: {}, line2: {}, point: {}", line1, line2, point)
            }
            Intersection::PartialOverlap { line1, line2, overlap } => {
                write!(f, "PartialOverlap: line1: {}, line2: {}, overlap: {}", line1, line2, overlap)
            }
            Intersection::ContainedOverlap { line1, line2, overlap } => {
                write!(f, "ContainedOverlap: line1: {}, line2: {}, overlap: {}", line1, line2, overlap)
            }
            Intersection::IdenticalOverlap { line1, line2, overlap } => {
                write!(f, "IdenticalOverlap: line1: {}, line2: {}, overlap: {}", line1, line2, overlap)
            }
        }
    }
}