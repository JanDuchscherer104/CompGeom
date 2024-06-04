use utils::get_dat_paths;
use geometry::line_segments::LineSegments2D;

mod geometry;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = get_dat_paths("data")?;
    for path in paths {
        let lines = LineSegments2D::from_dat(&path)?;
        println!("{:?}", lines);
    }
    Ok(())
}
