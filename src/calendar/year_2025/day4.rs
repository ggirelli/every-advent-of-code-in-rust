use std::{cmp::min, path::PathBuf};

use anyhow::{Error, Result};
use ndarray::{Array2, s};

use crate::io::read_lines;

fn count_roll_neighbors(map: &[String]) -> Result<Array2<usize>, Error> {
    let mut neighborhood_sizes: Array2<usize> = Array2::zeros((map.len(), map[0].len()));
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, content) in row.chars().enumerate() {
            if content != '@' {
                continue;
            }
            let row_start: usize = if row_index == 0 { 0 } else { row_index - 1 };
            let row_end: usize = min(map.len(), row_index + 2);
            let col_start: usize = if col_index == 0 { 0 } else { col_index - 1 };
            let col_end: usize = min(row.len(), col_index + 2);
            *neighborhood_sizes.slice_mut(s![row_start..row_end, col_start..col_end]) += 1;
            *neighborhood_sizes
                .get_mut((row_index, col_index))
                .ok_or(Error::msg("Failed to retrieve roll location."))? -= 1;
        }
    }
    Ok(neighborhood_sizes)
}

fn count_accessible_rolls(map: Vec<String>) -> Result<usize, Error> {
    let neighborhood_size: Array2<usize> = count_roll_neighbors(&map)?;
    let mut accessible_roll_counter: usize = 0;
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, content) in row.chars().enumerate() {
            if content == '@'
                && *neighborhood_size
                    .get((row_index, col_index))
                    .ok_or(Error::msg("Failed to get neighborhood size."))?
                    < 4
            {
                accessible_roll_counter += 1;
            }
        }
    }
    Ok(accessible_roll_counter)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let num_accessible_rolls: usize = count_accessible_rolls(read_lines(input)?)?;
    tracing::info!("Found {} accessible rolls.", num_accessible_rolls);
    Ok(())
}

pub fn run_part_2(_input: PathBuf) -> Result<(), Error> {
    Err(Error::msg("Not implemented."))
    //Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            count_accessible_rolls(vec![
                "..@@.@@@@.".to_string(),
                "@@@.@.@.@@".to_string(),
                "@@@@@.@.@@".to_string(),
                "@.@@@@..@.".to_string(),
                "@@.@@@@.@@".to_string(),
                ".@@@@@@@.@".to_string(),
                ".@.@.@.@@@".to_string(),
                "@.@@@.@@@@".to_string(),
                ".@@@@@@@@.".to_string(),
                "@.@.@@@.@.".to_string(),
            ])?,
            13
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        Ok(())
    }
}
