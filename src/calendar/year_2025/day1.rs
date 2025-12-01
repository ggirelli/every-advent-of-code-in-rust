use anyhow::{Error, Result};
use std::path::PathBuf;

use crate::io::read_lines;

fn run_instructions(
    instructions: Vec<String>,
    use_method_0x434c49434b: Option<()>,
) -> Result<usize, Error> {
    let mut zero_counter: usize = 0;
    let mut current_value: i32 = 50;

    for (idx, instruction) in instructions.into_iter().enumerate() {
        let distance: i32 = instruction[1..instruction.len()].parse::<i32>()?;
        let direction: char = instruction.chars().next().ok_or(Error::msg(format!(
            "Empty instruction found on line {}",
            idx
        )))?;

        match direction {
            'R' => {
                for _ in 0..distance {
                    current_value += 1;
                    if current_value == 100 {
                        if use_method_0x434c49434b.is_some() {
                            zero_counter += 1;
                        }
                        current_value = 0;
                    }
                }
            }
            'L' => {
                for _ in 0..distance {
                    current_value -= 1;
                    if current_value == 0 {
                        if use_method_0x434c49434b.is_some() {
                            zero_counter += 1;
                        }
                    } else if current_value == -1 {
                        current_value = 99;
                    }
                }
            }
            _ => {
                return Err(Error::msg(format!(
                    "Unrecognized direction on line {} : {}",
                    idx, instruction
                )));
            }
        }
        if use_method_0x434c49434b.is_none() && current_value == 0 {
            zero_counter += 1;
        }
    }
    Ok(zero_counter)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let zero_counter: usize = run_instructions(read_lines(input)?, None)?;
    tracing::info!("Dial pointed at 0 for a total of {} times.", zero_counter);
    Ok(())
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let zero_counter: usize = run_instructions(read_lines(input)?, Some(()))?;
    tracing::info!("Dial pointed at 0 for a total of {} times.", zero_counter);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            run_instructions(
                vec![
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
                ]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
                None
            )?,
            3
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        assert_eq!(
            run_instructions(
                vec![
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
                ]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
                Some(())
            )?,
            6
        );
        Ok(())
    }
}
