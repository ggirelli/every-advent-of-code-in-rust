use std::path::PathBuf;

use anyhow::{Error, Result};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
        3 => match part {
            1 => day3::run_part_1(input),
            2 => day3::run_part_2(input),
            _ => panic!("Unsupported part {}", part),
        },
        4 => match part {
            1 => day4::run_part_1(input),
            2 => day4::run_part_2(input),
            _ => panic!("Unsupported part {}", part),
        },
        5 => match part {
            1 => day5::run_part_1(input),
            2 => day5::run_part_2(input),
            _ => panic!("Unsupported part {}", part),
        },
        6 => match part {
            1 => day6::run_part_1(input),
            2 => day6::run_part_2(input),
            _ => panic!("Unsupported part {}", part),
        },
        _ => panic!("Unsupported day {}", day),
    }
}
