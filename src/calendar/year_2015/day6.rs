use std::path::PathBuf;

use anyhow::{Error, Result};
use ndarray::{Array2, s};
use pretty_assertions::assert_eq;

use crate::io::read_lines;

fn count_on_lights(grid: &Array2<bool>) -> usize {
    grid.iter().filter(|&&v| v).count()
}

fn execute_single_light_operation(
    mut grid: Array2<bool>,
    instruction: &ParsedInstruction,
) -> Result<Array2<bool>, Error> {
    match instruction.op {
        OperationType::On => {
            // Turn on the lights.
            grid.slice_mut(s![
                instruction.xmin..(instruction.xmax + 1),
                instruction.ymin..(instruction.ymax + 1)
            ])
            .mapv_inplace(|_| true);
        }
        OperationType::Off => {
            // Turn off the lights.
            grid.slice_mut(s![
                instruction.xmin..(instruction.xmax + 1),
                instruction.ymin..(instruction.ymax + 1)
            ])
            .mapv_inplace(|_| false);
        }
        OperationType::Toggle => {
            // Toggle the lights.
            grid.slice_mut(s![
                instruction.xmin..(instruction.xmax + 1),
                instruction.ymin..(instruction.ymax + 1)
            ])
            .mapv_inplace(|v| !v);
        }
    }
    Ok(grid)
}

#[derive(Debug, PartialEq)]
enum OperationType {
    On,
    Off,
    Toggle,
}

/// A parsed instruction.
#[derive(Debug, PartialEq)]
struct ParsedInstruction {
    pub op: OperationType,
    pub xmin: usize,
    pub xmax: usize,
    pub ymin: usize,
    pub ymax: usize,
}

/// Parse an instruction from string to struct.
fn parse_instruction(s: &str) -> Result<ParsedInstruction, Error> {
    let parts: Vec<&str> = s.split(" ").collect();

    let op: OperationType;
    let p_parts_indices: [usize; 2];
    match parts.len() {
        4 => {
            op = OperationType::Toggle;
            p_parts_indices = [1, 3];
        }
        5 => {
            op = if parts.get(1).unwrap() == &"on" {
                OperationType::On
            } else {
                OperationType::Off
            };
            p_parts_indices = [2, 4];
        }
        _ => panic!("Unsupported string: {}", s),
    }

    let p1: Vec<usize> = parts
        .get(p_parts_indices[0])
        .unwrap()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    assert_eq!(p1.len(), 2);

    let p2: Vec<usize> = parts
        .get(p_parts_indices[1])
        .unwrap()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    assert_eq!(p2.len(), 2);

    assert!(p1[0] <= p2[0]);
    assert!(p1[1] <= p2[1]);

    Ok(ParsedInstruction {
        op,
        xmin: p1[0],
        xmax: p2[0],
        ymin: p1[1],
        ymax: p2[1],
    })
}

fn operate_lights(instructions: Vec<String>) -> Result<Array2<bool>, Error> {
    let mut grid: Array2<bool> = Array2::zeros((1000, 1000)).mapv(|_: usize| false);
    for line in instructions {
        grid = execute_single_light_operation(grid, &parse_instruction(&line)?)?;
    }
    Ok(grid)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let grid: Array2<bool> = operate_lights(read_lines(input)?)?;
    tracing::info!("{} lights are lit.", count_on_lights(&grid));
    Ok(())
}

fn execute_single_light_operation_v2(
    mut grid: Array2<u32>,
    instruction: &ParsedInstruction,
) -> Result<Array2<u32>, Error> {
    match instruction.op {
        OperationType::On => {
            // Turn on the lights.
            grid.slice_mut(s![
                instruction.xmin..(instruction.xmax + 1),
                instruction.ymin..(instruction.ymax + 1)
            ])
            .mapv_inplace(|v: u32| v + 1);
        }
        OperationType::Off => {
            // Turn off the lights.
            grid.slice_mut(s![
                instruction.xmin..(instruction.xmax + 1),
                instruction.ymin..(instruction.ymax + 1)
            ])
            .mapv_inplace(|v: u32| if v == 0 { v } else { v - 1 });
        }
        OperationType::Toggle => {
            // Toggle the lights.
            grid.slice_mut(s![
                instruction.xmin..(instruction.xmax + 1),
                instruction.ymin..(instruction.ymax + 1)
            ])
            .mapv_inplace(|v| v + 2);
        }
    }
    Ok(grid)
}

fn operate_lights_v2(instructions: Vec<String>) -> Result<Array2<u32>, Error> {
    let mut grid: Array2<u32> = Array2::zeros((1000, 1000)).mapv(|_: u32| 0);
    for line in instructions {
        grid = execute_single_light_operation_v2(grid, &parse_instruction(&line)?)?;
    }
    Ok(grid)
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let grid: Array2<u32> = operate_lights_v2(read_lines(input)?)?;
    tracing::info!("Total brightness: {}", &grid.sum());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_execute_single_light_operation() {
        let mut grid: Array2<bool> = Array2::zeros((1000, 1000)).mapv(|_: usize| false);
        grid = execute_single_light_operation(
            grid,
            &parse_instruction("turn on 0,0 through 999,999").unwrap(),
        )
        .unwrap();
        assert_eq!(count_on_lights(&grid), 1000000);
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("turn on 0,0 through 999,999").unwrap(),
            ParsedInstruction {
                op: OperationType::On,
                xmin: 0,
                ymin: 0,
                xmax: 999,
                ymax: 999
            }
        );
        assert_eq!(
            parse_instruction("toggle 0,0 through 999,0").unwrap(),
            ParsedInstruction {
                op: OperationType::Toggle,
                xmin: 0,
                ymin: 0,
                xmax: 999,
                ymax: 0
            }
        );
        assert_eq!(
            parse_instruction("turn off 499,499 through 500,500").unwrap(),
            ParsedInstruction {
                op: OperationType::Off,
                xmin: 499,
                ymin: 499,
                xmax: 500,
                ymax: 500
            }
        );
    }
}
