use anyhow::{Error, Result};
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::{EnvFilter, fmt};

use every_advent_of_code::calendar::year_2015;

/// Every Advent of Code in Rust.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to input link_assoc file. Supports CSV and CSV.GZ.
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    year: usize,

    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    part: usize,
}

fn main() -> Result<(), Error> {
    let filter: EnvFilter = EnvFilter::from_default_env();
    fmt().with_env_filter(filter).init();

    let args: Args = Args::parse();
    tracing::info!("Running {} {} pt{}", args.year, args.day, args.part);
    tracing::info!("Input from {}", args.input.to_str().unwrap());

    match args.year {
        2015 => year_2015::run_day(args.day, args.part, args.input),
        _ => panic!("Unsupported year {}", args.year),
    }?;

    tracing::info!("Done.");
    Ok(())
}
