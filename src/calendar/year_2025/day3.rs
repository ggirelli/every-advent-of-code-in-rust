use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::io::read_lines;

fn get_numeric_string_max_char_and_index(s: String) -> Result<(usize, char), Error> {
    let mut max_char_index: usize = 0;
    let mut max_char: char = '0';
    let mut max_value: usize = 0;
    for (i, c) in s.chars().enumerate() {
        let value: usize = c.to_string().parse::<usize>()?;
        if value > max_value {
            max_char = c;
            max_char_index = i;
            max_value = value;
        }
    }
    Ok((max_char_index, max_char))
}

fn get_bank_joltage(bank: String, batteries_on: usize) -> Result<String, Error> {
    if batteries_on == 1 {
        Ok(get_numeric_string_max_char_and_index(bank)?.1.to_string())
    } else {
        let (max_index, joltage) = get_numeric_string_max_char_and_index(
            bank[0..(bank.len() - batteries_on + 1)].to_string(),
        )?;
        let next_batteries_on_count: usize = batteries_on - 1;
        Ok(format!(
            "{}{}",
            joltage,
            get_bank_joltage(
                bank[(max_index + 1)..bank.len()].to_string(),
                next_batteries_on_count,
            )?
        ))
    }
}

fn get_total_output_joltage(
    battery_banks: Vec<String>,
    batteries_on_per_bank: usize,
) -> Result<usize, Error> {
    let mut total_output_joltage: usize = 0;
    for bank in battery_banks {
        let bank_joltage: String = get_bank_joltage(bank, batteries_on_per_bank)?;
        total_output_joltage += bank_joltage.parse::<usize>()?;
    }
    Ok(total_output_joltage)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let total_output_joltage: usize = get_total_output_joltage(read_lines(input)?, 2)?;
    tracing::info!("The total output joltage is {}.", total_output_joltage);
    Ok(())
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let total_output_joltage: usize = get_total_output_joltage(read_lines(input)?, 12)?;
    tracing::info!("The total output joltage is {}.", total_output_joltage);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            get_total_output_joltage(
                vec![
                    "987654321111111".to_string(),
                    "811111111111119".to_string(),
                    "234234234234278".to_string(),
                    "818181911112111".to_string(),
                ],
                2
            )?,
            357
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        assert_eq!(
            get_total_output_joltage(
                vec![
                    "987654321111111".to_string(),
                    "811111111111119".to_string(),
                    "234234234234278".to_string(),
                    "818181911112111".to_string(),
                ],
                12
            )?,
            3121910778619
        );
        Ok(())
    }
}
