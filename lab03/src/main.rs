use crate::geometry::brute_force;
use crate::geometry::sweep_line::handler::SweepLineOptions;
use cpu_time::ProcessTime;
use geometry::line_segments::LineSegments2D;
use geometry::sweep_line::handler::Handler;
use std::env;
use std::path::Path;
use std::time::Duration;

mod geometry;
mod utils;

const ALL_LINES: [&str; 4] = [
    "data/s_1000_1.dat",
    "data/s_1000_10.dat",
    "data/s_10000_1.dat",
    "data/s_100000_1.dat",
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <benchmark|analyze> <filename|all>", args[0]);
        return;
    }

    match args[1].as_str() {
        "benchmark" => {
            let file = args[2].as_str();
            match file {
                "" => {
                    eprintln!("No file specified");
                    return;
                }
                "all" => benchmark_all(),
                _ => benchmark_single(file),
            }
        }
        "analyze" => {
            let file = args[2].as_str();
            match file {
                "" => {
                    eprintln!("No file specified");
                    return;
                }
                "all" => {
                    eprintln!("Analyze only supports single files");
                    return;
                }
                _ => {
                    analyze(file);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

fn analyze(file: &str) {
    let lines = get_lines(file).expect(format!("Error reading file {}", file).as_str());

    let mut brute_force_handler = brute_force::handler::BruteForceHandler::new(lines.lines);
    brute_force_handler.run();
    brute_force_handler.analyze();
}

struct BenchmarkResult {
    file: String,
    lines: usize,
    intersections: usize,
    time: Duration,
}

fn benchmark(file: &str) -> BenchmarkResult {
    let lines = get_lines(file).expect(format!("Error reading file {}", file).as_str());

    let start = ProcessTime::try_now().expect("Getting process time failed");

    let mut sweep_line_handler =
        Handler::new(lines.lines.clone(), SweepLineOptions::panic_disabled());
    let intersections = sweep_line_handler.run();

    BenchmarkResult {
        file: file.to_string(),
        lines: lines.lines.len(),
        intersections: intersections.len(),
        time: start.elapsed(),
    }
}

fn benchmark_single(file: &str) {
    let result = benchmark(file);

    print_benchmark_results(vec![result]);
}

fn benchmark_all() {
    let mut benchmark_results: Vec<BenchmarkResult> = Vec::new();
    for file in ALL_LINES.iter() {
        benchmark_results.push(benchmark(file));
    }

    print_benchmark_results(benchmark_results);
}

fn print_benchmark_results(results: Vec<BenchmarkResult>) {
    println!();
    // Header
    println!(
        "| {0: <20} | {1: <10} | {2: <15} | {3: <15} |",
        "File", "# Lines", "# Intersections", "CPU Time (ms)"
    );
    println!("| {} | {} | {} | {} |", "-".repeat(20), "-".repeat(10), "-".repeat(15), "-".repeat(15));

    for (i, result) in results.iter().enumerate() {
        println!(
            "| {0: <20} | {1: <10} | {2: <15} | {3: <15} |",
            result.file,
            result.lines,
            result.intersections,
            result.time.as_millis()
        );
    }

    println!("\nFor more detailed results, use analyze <filename>");
}

fn get_lines(file: &str) -> Option<LineSegments2D> {
    let path = Path::new(file);

    let lines = LineSegments2D::from_dat(path);
    if lines.is_err() {
        return None;
    }
    Some(lines.unwrap())
}
