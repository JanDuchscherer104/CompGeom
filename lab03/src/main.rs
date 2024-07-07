use utils::get_dat_paths;
use geometry::line_segments::LineSegments2D;
use geometry::sweep_line::handler::Handler;
use crate::geometry::sweep_line::handler::SweepLineOptions;

mod geometry;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = get_dat_paths("data")?;

    let lines = LineSegments2D::from_dat(&paths[3])?;
    // println!("{:?}", lines);
    let mut handler = Handler::new(lines.lines, SweepLineOptions::panic_disabled());
    let intersections = handler.run();

    println!("Found {} intersections", intersections.len());

    Ok(())
}
