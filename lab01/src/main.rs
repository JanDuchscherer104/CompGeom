use std::fs::read_to_string;
use std::io::Result;
use std::time::Instant;

// use lab01::naive_impl::{count_intersections, Line2D, Point2D};
use lab01::line_sweep::{count_intersections, Line2D};

fn read_to_vec_tup(file_name: &str) -> Result<Vec<Line2D>> {
    Ok(read_to_string(file_name)?
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|s| s.parse::<f32>().unwrap());
            Line2D::from_f32_iter(&mut nums)
        })
        .collect::<Vec<Line2D>>())
}

fn main() -> Result<()> {
    let file_names = vec!["s_1000_1.dat"]; //, "s_10000_1.dat", "s_100000_1.dat"];
    for file_name in file_names.iter() {
        let start = Instant::now();

        let data = read_to_vec_tup(&format!(".data/{}", file_name)).unwrap();
        print! {"# Elements: {}\n", data.len()};
        // print!("# Intersections: {}\n", count_intersections(&data));
        // print!("{}", sweep_line(&data));

        let duration = start.elapsed();

        println!(
            "Time elapsed in count_intersections is: {:?} for file_name '{}'",
            duration, file_name
        );
    }

    Ok(())
}
