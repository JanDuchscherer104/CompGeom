use lab03::{geom::LineSegments2D, utils::get_dat_paths};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = get_dat_paths("data")?;
    for path in paths {
        let lines = LineSegments2D::from_dat(&path)?;
        println!("{:?}", lines);
    }
    Ok(())
}
