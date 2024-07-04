use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};
use ordered_float::OrderedFloat;

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
                        let x1 = nums[0];
                        let y1 = nums[1];
                        let x2 = nums[2];
                        let y2 = nums[3];

                        if (OrderedFloat(x1) < OrderedFloat(x2)) {
                            Ok(Line2D::new(x1,y1, x2, y2))
                        } else {
                            Ok(Line2D::new(x2,y2, x1, y1))
                        }
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
