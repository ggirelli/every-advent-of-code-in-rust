use std::{cmp::min, path::PathBuf};

use anyhow::{Error, Result};
use ndarray::{Array2, s};

use crate::io::read_lines;

fn count_roll_neighbors(map: &Array2<char>) -> Result<Array2<usize>, Error> {
    let mut neighborhood_sizes: Array2<usize> = Array2::zeros((map.nrows(), map.ncols()));
    for row_index in 0..map.nrows() {
        for col_index in 0..map.ncols() {
            if *map
                .get((row_index, col_index))
                .ok_or(Error::msg("Failed to retrive location content."))?
                != '@'
            {
                continue;
            }
            let row_start: usize = if row_index == 0 { 0 } else { row_index - 1 };
            let row_end: usize = min(map.nrows(), row_index + 2);
            let col_start: usize = if col_index == 0 { 0 } else { col_index - 1 };
            let col_end: usize = min(map.ncols(), col_index + 2);
            *neighborhood_sizes.slice_mut(s![row_start..row_end, col_start..col_end]) += 1;
            *neighborhood_sizes
                .get_mut((row_index, col_index))
                .ok_or(Error::msg("Failed to retrieve roll location."))? -= 1;
        }
    }
    Ok(neighborhood_sizes)
}

fn get_accessibility_mask(map: &Array2<char>) -> Result<Array2<bool>, Error> {
    let neighborhood_size: Array2<usize> = count_roll_neighbors(map)?;
    Ok(neighborhood_size.mapv(|v| v < 4) & map.mapv(|c| c == '@'))
}

fn count_accessible_rolls(accessibility_map: &Array2<bool>) -> Result<usize, Error> {
    Ok(accessibility_map.iter().filter(|&&v| v).count())
}

fn count_removable_rolls(mut map: Array2<char>) -> Result<usize, Error> {
    let accessibility_map: Array2<bool> = get_accessibility_mask(&map)?;
    let num_accessible_rolls: usize = count_accessible_rolls(&accessibility_map)?;
    if num_accessible_rolls == 0 {
        return Ok(0);
    }
    for ((row_index, col_index), &is_accessible) in accessibility_map.indexed_iter() {
        if is_accessible {
            map[(row_index, col_index)] = '.';
        }
    }
    Ok(num_accessible_rolls + count_removable_rolls(map)?)
}

fn build_roll_map(map_string: Vec<String>) -> Result<Array2<char>, Error> {
    Ok(Array2::from_shape_vec(
        (map_string.len(), map_string[0].len()),
        map_string
            .iter()
            .flat_map(|s| s.chars().collect::<Vec<char>>())
            .collect(),
    )?)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let num_accessible_rolls: usize = count_accessible_rolls(&get_accessibility_mask(
        &build_roll_map(read_lines(input)?)?,
    )?)?;
    tracing::info!("Found {} accessible rolls.", num_accessible_rolls);
    Ok(())
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let num_removable_rolls: usize = count_removable_rolls(build_roll_map(read_lines(input)?)?)?;
    tracing::info!("Found {} removable rolls.", num_removable_rolls);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            count_accessible_rolls(&get_accessibility_mask(&build_roll_map(vec![
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
            ])?)?)?,
            13
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        assert_eq!(
            count_removable_rolls(build_roll_map(vec![
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
            ])?)?,
            43
        );
        Ok(())
    }
}
