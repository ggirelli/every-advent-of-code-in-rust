use std::path::PathBuf;

use anyhow::{Error, Result};

mod day1;
mod day2;

/// Run a specific day of 2015.
pub fn run_day(day: usize, part: usize, input: PathBuf) -> Result<(), Error> {
    match day {
        1 => match part {
            1 => day1::run_part_1(input),
            2 => day1::run_part_2(input),
            _ => panic!("Unsupported part {}", part),
        },
        2 => match part {
            1 => day2::run_part_1(input),
            2 => day2::run_part_2(input),
            _ => panic!("Unsupported part {}", part),
        },
        _ => panic!("Unsupported day {}", day),
    }
}
