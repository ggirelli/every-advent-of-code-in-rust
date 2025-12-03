use anyhow::{Error, Result};
use clap::Parser;
use std::path::PathBuf;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use every_advent_of_code::calendar::{year_2015, year_2025};

/// Every Advent of Code in Rust.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to input link_assoc file. Supports CSV and CSV.GZ.
    input: PathBuf,
    year: usize,
    day: usize,
    part: usize,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let args: Args = Args::parse();

    // Setup logger
    let subscriber: FmtSubscriber = FmtSubscriber::builder()
        .with_max_level(if args.verbose {
            Level::DEBUG
        } else {
            Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::info!("Running {} {} pt{}", args.year, args.day, args.part);
    tracing::info!("Input from {}", args.input.to_str().unwrap());

    match args.year {
        2015 => year_2015::run_day(args.day, args.part, args.input),
        2025 => year_2025::run_day(args.day, args.part, args.input),
        _ => panic!("Unsupported year {}", args.year),
    }?;

    tracing::info!("Done.");
    Ok(())
}
