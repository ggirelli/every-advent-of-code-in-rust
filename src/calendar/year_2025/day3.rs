use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::io::read_lines;

fn get_total_output_joltage(battery_banks: Vec<String>) -> Result<usize, Error> {
    let mut total_output_joltage: usize = 0;
    for bank in battery_banks {
        let mut bank_output_joltage: usize = 0;
        for (battery_pos_1, battery_jolt_1) in bank[0..(bank.len() - 1)].chars().enumerate() {
            for battery_jolt_2 in bank[(battery_pos_1 + 1)..bank.len()].chars() {
                let pair_joltage: usize =
                    format!("{}{}", battery_jolt_1, battery_jolt_2).parse::<usize>()?;
                if pair_joltage > bank_output_joltage {
                    bank_output_joltage = pair_joltage;
                }
            }
        }
        total_output_joltage += bank_output_joltage;
    }
    Ok(total_output_joltage)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let total_output_joltage: usize = get_total_output_joltage(read_lines(input)?)?;
    tracing::info!("The total output joltage is {}.", total_output_joltage);
    Ok(())
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    // Ok(())
    Err(Error::msg("Not implemented."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            get_total_output_joltage(vec![
                "987654321111111".to_string(),
                "811111111111119".to_string(),
                "234234234234278".to_string(),
                "818181911112111".to_string(),
            ])?,
            357
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        Ok(())
    }
}
