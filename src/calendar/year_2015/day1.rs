use anyhow::{Error, Result};
use std::fs::read_to_string;
use std::path::PathBuf;

/// Find final floor based on instructions.
pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let mut current_position: i32 = 0;
    for instruction in read_to_string(input)?.chars() {
        match instruction {
            '(' => current_position += 1,
            ')' => current_position -= 1,
            _ => panic!("Unsupported character: {}", instruction),
        }
    }
    tracing::info!("Final destination: {}", current_position);
    Ok(())
}

/// Find first time in the basement, base-1 index.
pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let mut current_position: i32 = 0;
    for (pos, instruction) in read_to_string(input)?.chars().enumerate() {
        match instruction {
            '(' => current_position += 1,
            ')' => current_position -= 1,
            _ => panic!("Unsupported character: {}", instruction),
        }
        if current_position == -1 {
            // Offset 1 due to base-1 index.
            tracing::info!("Instruction index: {}", pos + 1);
            break;
        }
    }
    Ok(())
}
