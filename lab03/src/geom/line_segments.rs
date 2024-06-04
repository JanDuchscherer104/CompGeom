use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use super::line::Line2D;

#[derive(Clone, Debug)]
pub struct LineSegments2D {
    pub lines: Vec<Line2D>,
}

impl LineSegments2D {
    pub fn from_dat(path: &Path) -> io::Result<Self> {
        let file = fs::File::open(&path)?;
        let reader = io::BufReader::new(file);

        let lines: io::Result<Vec<Line2D>> = reader
            .lines()
            .map(|line| {
                line.and_then(|v| {
                    let nums: Vec<f64> = v
                        .split_whitespace()
                        .map(|num| num.parse::<f64>().unwrap())
                        .collect();
                    if nums.len() == 4 {
                        Ok(Line2D::new(nums[0], nums[1], nums[2], nums[3]))
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid number of points in line segment",
                        ))
                    }
                })
            })
            .collect();

        Ok(LineSegments2D { lines: lines? })
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
struct SweepLine {}

// X-Structure: Maintains the global order of transitions of the sweep line.
// Must support the following operations efficiently:
// - insert a new event
// - pop the event with the minimum x-coordinate, which is the next event to be processed
// Appropriate data structures: priority queue or balanced binary search tree
// Contains:
//    - all endpoints of future segments
//    - all endpoints (to the right of the SL) of active segments
//    - all intersections to the right of the SL of active segments
// - sorted w.r.t. x-coordinates

// Requirements for Bentley-Ottmann Algorithm:
// - No vertical line segments
// - No segments intersect at their endpoints (start & end points)
// - No three (or more) segments intersect at a single point
// - All endpoints and intersection points have distinct x-coordinates
// - No two segments overlap
//
// Algorithm:
// - Vertical SL sweeps from left to right
// - Partitions segments into 3 groups:
//   - past_segments: segments that lie entirely to the left of the SL
//   - active_segments: segments that intersect the SL
//     - maintained by the Y-Structure
//     - sorted by y-coordinate of the intersection point
//   - future_segments: segments that lie entirely to the right of the SL
// Case 1: SL encounters the left endpoint of a segment L
// - Insert L into Y-Structure
// - Search for the successor (top neighbor) of L (L')
// - Search for the predecessor (bottom neighbor) of L (L*)
// - If L and L' intersect, insert their intersection point into X-Structure
// - If L and L* intersect, insert their intersection point into X-Structure
// - L' and L* are no longer neighbors: check if they intersect to the right of the SL, if they do, delete their intersection point from the X-Structure
// Case 2: SL encounters the right endpoint of L
// - Remove L from Y-Structure
// - Search for the successor (top neighbor) of L (L')
// - Search for the predecessor (bottom neighbor) of L (L*)
// - If L' and L* intersect, insert their intersection point into X-Structure
// Case 3: SL encounters an intersection between two segments L1 and L2 (L1.y(x-) > L2.y(x-))
// - Report (L1, L2) as an intersection
// - Get successor of L1 (L1') and predecessor of L2 (L2*)
// - If L1' and L2 intersect, insert their intersection point into X-Structure
// - If L1 and L2* intersect, insert their intersection point into X-Structure
