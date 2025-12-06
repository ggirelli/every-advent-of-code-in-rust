use std::path::PathBuf;

use anyhow::{Error, Result};
use ndarray::Array1;

use crate::io::read_lines;

enum OperationType {
    Sum,
    Prod,
    Unknown,
}

struct Problem {
    pub factors: Vec<usize>,
    pub operation_type: OperationType,
}

impl Problem {
    pub fn solve(&self) -> Result<usize, Error> {
        match self.operation_type {
            OperationType::Sum => Ok(Array1::from_vec(self.factors.clone()).sum()),
            OperationType::Prod => Ok(Array1::from_vec(self.factors.clone()).product()),
            OperationType::Unknown => Err(Error::msg("Unknown operation for problem solving.")),
        }
    }
}

fn calculate_grand_total(homework: Vec<String>) -> Result<usize, Error> {
    let mut grand_total: usize = 0;
    let mut problems: Vec<Problem> = Vec::new();
    for line in homework {
        let factors: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        if problems.is_empty() {
            for f in factors {
                problems.push(Problem {
                    factors: vec![f.parse::<usize>()?],
                    operation_type: OperationType::Unknown,
                });
            }
        } else if !["+", "*"].contains(&factors[0]) {
            for (problem_index, f) in factors.iter().enumerate() {
                problems[problem_index].factors.push(f.parse::<usize>()?);
            }
        } else {
            for (problem_index, f) in factors.iter().enumerate() {
                problems[problem_index].operation_type = match *f {
                    "+" => Ok(OperationType::Sum),
                    "*" => Ok(OperationType::Prod),
                    _ => Err(Error::msg("Unknown operation.")),
                }?;
            }
        }
    }
    for p in problems {
        grand_total += p.solve()?;
    }
    Ok(grand_total)
}

fn transpose_strings(homework: Vec<String>) -> Result<Vec<String>, Error> {
    let ncols: usize = homework[0].len() + 1;
    let mut transposed_homework: Vec<String> =
        (0..ncols).map(|_| "".to_string()).collect::<Vec<String>>();
    for line in homework {
        for (col_index, c) in line.chars().enumerate() {
            if ['+', '*'].contains(&c) {
                transposed_homework[ncols - col_index - 1].push(c);
            } else if c != ' ' {
                transposed_homework[ncols - col_index - 2].push(c);
            }
        }
    }
    Ok(transposed_homework)
}

fn calculate_grand_total_v2(homework: Vec<String>) -> Result<usize, Error> {
    let mut grand_total: usize = 0;
    let mut factors: Vec<usize> = Vec::new();
    for line in transpose_strings(homework)? {
        if line == "+" {
            grand_total += Array1::from_vec(factors).sum();
            factors = Vec::new();
        } else if line == "*" {
            grand_total += Array1::from_vec(factors).product();
            factors = Vec::new();
        } else {
            factors.push(line.parse::<usize>()?);
        }
    }
    Ok(grand_total)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let grand_total: usize = calculate_grand_total(read_lines(input)?)?;
    tracing::info!("The grand total is {}.", grand_total);
    Ok(())
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let grand_total: usize = calculate_grand_total_v2(read_lines(input)?)?;
    tracing::info!("The grand total is {}.", grand_total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            calculate_grand_total(vec![
                "123 328  51 64 ".to_string(),
                " 45 64  387 23 ".to_string(),
                "  6 98  215 314".to_string(),
                "*   +   *   +  ".to_string(),
            ])?,
            4277556
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        assert_eq!(
            calculate_grand_total_v2(vec![
                "123 328  51 64 ".to_string(),
                " 45 64  387 23 ".to_string(),
                "  6 98  215 314".to_string(),
                "*   +   *   +  ".to_string(),
            ])?,
            3263827
        );
        Ok(())
    }
}
