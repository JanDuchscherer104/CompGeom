use std::hash::Hash;
use crate::geometry::line::Line2D;
use crate::geometry::point::Point2D;

#[derive(Copy, Clone, Debug, Hash)]
pub enum Intersection {
    Crossing { line1: Line2D, line2: Line2D, point: Point2D },
    Touching { line1: Line2D, line2: Line2D, point: Point2D },
    /// One line is only partially overlapping the other
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