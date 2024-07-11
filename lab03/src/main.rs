use crate::geometry::brute_force;
use crate::geometry::sweep_line::handler::SweepLineOptions;
use cpu_time::ProcessTime;
use geometry::line_segments::LineSegments2D;
use geometry::sweep_line::handler::Handler;
use geometry::external::handler::GeoHandler;
use memory_stats::memory_stats;

use std::{env, panic};
use std::fmt::Display;
use std::path::Path;
use std::time::Duration;

mod geometry;

const ALL_LINES: [&str; 4] = [
    "data/s_1000_1.dat",
    "data/s_1000_10.dat",
    "data/s_10000_1.dat",
    "data/s_100000_1.dat",
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <benchmark|analyze> <filename|all> <--brute-force|--sweep-line>", args[0]);
        return;
    }

    match args[1].as_str() {
        "benchmark" => {
            let file = args[2].as_str();
            let algorithm = match args.get(3) {
                Some(algorithm) => match algorithm.as_str() {
                    "--brute-force" => Algorithm::BruteForce,
                    "--sweep-line" => Algorithm::SweepLine,
                    "--external" => Algorithm::External,
                    _ => {
                        eprintln!("Unknown parameter: {}", algorithm);
                        return;
                    }
                },
                None => {
                    println!("Using default algorithm: Brute Force");
                    Algorithm::BruteForce
                },
            };
            match file {
                "" => {
                    eprintln!("No file specified");
                    return;
                }
                "all" => benchmark_all(algorithm),
                _ => benchmark_single(file, algorithm),
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

#[derive(Clone, Copy)]
enum Algorithm {
    BruteForce,
    SweepLine,
    External,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::BruteForce => write!(f, "Brute Force"),
            Algorithm::SweepLine => write!(f, "Sweep Line"),
            Algorithm::External => write!(f, "External (Geo Library)"),
        }
    }
}

struct BenchmarkResult {
    file: String,
    lines: usize,
    intersections: Option<usize>,
    time: Option<Duration>,
    memory: Option<u64>,
}

fn benchmark_single(file: &str, algorithm: Algorithm) {
    let result = benchmark(file, algorithm);

    print_benchmark_results(vec![result]);
}

fn benchmark_all(algorithm: Algorithm) {
    let mut benchmark_results: Vec<BenchmarkResult> = Vec::new();
    for file in ALL_LINES.iter() {
        benchmark_results.push(benchmark(file, algorithm));
    }

    print_benchmark_results(benchmark_results);
}

fn benchmark(file: &str, algorithm: Algorithm) -> BenchmarkResult {
    println!("Starting benchmark for file {} with {} Algorithm...", file, algorithm.to_string());
    let lines = get_lines(file).expect(format!("Error reading file {}", file).as_str());

    let start = ProcessTime::try_now().expect("Getting process time failed");
    let memory_start = get_memory_usage();

    let result = panic::catch_unwind(|| {
        match algorithm {
            Algorithm::BruteForce => {
                let mut brute_force_handler = brute_force::handler::BruteForceHandler::new(lines.clone().lines);
                brute_force_handler.run()
            }
            Algorithm::SweepLine => {
                let mut sweep_line_handler = Handler::new(lines.clone().lines, SweepLineOptions::panic_disabled());
                sweep_line_handler.run()
            },
            Algorithm::External => {
                let geo_handler = GeoHandler::new(lines.clone().lines);
                geo_handler.run()
            }
        }
    });

    match result {
        Ok(intersections) => BenchmarkResult {
            file: file.to_string(),
            lines: lines.lines.len(),
            intersections: Some(intersections.len()),
            time: Some(start.elapsed()),
            memory: Some(get_memory_usage() - memory_start),
        },
        Err(_) => {
            BenchmarkResult {
                file: file.to_string(),
                lines: lines.lines.len(),
                intersections: None,
                time: None,
                memory: None,
            }
        }
    }
}

fn print_benchmark_results(results: Vec<BenchmarkResult>) {
    println!();
    // Header
    println!(
        "| {0: <20} | {1: <10} | {2: <15} | {3: <15} | {4: <15} |",
        "File", "# Lines", "# Intersections", "CPU Time (ms)", "Memory (kB)"
    );
    println!(
        "| {} | {} | {} | {} | {} |",
        "-".repeat(20),
        "-".repeat(10),
        "-".repeat(15),
        "-".repeat(15),
        "-".repeat(15)
    );

    for result in results {
        let intersections = match result.intersections {
            Some(i) => i.to_string(),
            None => "Error".to_string(),
        };

        let time = match result.time {
            Some(t) => t.as_millis().to_string(),
            None => "Error".to_string(),
        };

        let memory = match result.memory {
            Some(m) => (m/1024).to_string(), // Convert to kB
            None => "Error".to_string(),
        };

        println!(
            "| {0: <20} | {1: <10} | {2: <15} | {3: <15} | {4: <15} |",
            result.file,
            result.lines,
            intersections,
            time,
            memory
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

fn get_memory_usage() -> u64 {
    if let Some(usage) = memory_stats() {
        usage.physical_mem as u64
    } else {
        eprintln!("Couldn't get memory usage.");
        0
    }
}