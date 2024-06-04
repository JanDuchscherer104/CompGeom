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
