use std::{collections::HashSet, path::PathBuf};

use anyhow::{Error, Result};

use crate::io::read_lines;

struct ParsedMap {
    pub source: (i64, i64),
    pub splitters: HashSet<(i64, i64)>,
    pub nrows: usize,
}

fn parse_map(map: Vec<String>) -> Result<ParsedMap, Error> {
    let mut source: Option<(i64, i64)> = None;
    let mut splitters: HashSet<(i64, i64)> = HashSet::new();
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, c) in row.chars().enumerate() {
            match c {
                'S' => {
                    if source.is_some() {
                        return Err(Error::msg("Found multiple sources."));
                    }
                    source = Some((i64::try_from(row_index)?, i64::try_from(col_index)?));
                }
                '^' => {
                    splitters.insert((i64::try_from(row_index)?, i64::try_from(col_index)?));
                }
                '.' => {}
                _ => {
                    return Err(Error::msg(format!("Unrecognized character '{}'.", c)));
                }
            }
        }
    }
    Ok(ParsedMap {
        source: source.ok_or(Error::msg("No source found."))?,
        splitters,
        nrows: map.len(),
    })
}

fn count_splits(parsed_map: ParsedMap) -> Result<usize, Error> {
    let mut split_counter: usize = 0;
    let mut rays_positions: HashSet<(i64, i64)> = HashSet::new();
    rays_positions.insert(parsed_map.source);

    for _ in 0..parsed_map.nrows {
        let mut current_rays_positions: HashSet<(i64, i64)> = HashSet::new();
        for ray in rays_positions.iter() {
            let new_position: (i64, i64) = (ray.0 + 1, ray.1);
            if parsed_map.splitters.contains(&new_position) {
                split_counter += 1;
                current_rays_positions.insert((new_position.0, new_position.1 - 1));
                current_rays_positions.insert((new_position.0, new_position.1 + 1));
            } else {
                current_rays_positions.insert(new_position);
            }
        }
        rays_positions = current_rays_positions;
    }
    Ok(split_counter)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let num_splits: usize = count_splits(parse_map(read_lines(input)?)?)?;
    tracing::info!("The ray splits {} times.", num_splits);
    Ok(())
}

pub fn run_part_2(_input: PathBuf) -> Result<(), Error> {
    Err(Error::msg("Not implemented."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            count_splits(parse_map(vec![
                ".......S.......".to_string(),
                "...............".to_string(),
                ".......^.......".to_string(),
                "...............".to_string(),
                "......^.^......".to_string(),
                "...............".to_string(),
                ".....^.^.^.....".to_string(),
                "...............".to_string(),
                "....^.^...^....".to_string(),
                "...............".to_string(),
                "...^.^...^.^...".to_string(),
                "...............".to_string(),
                "..^...^.....^..".to_string(),
                "...............".to_string(),
                ".^.^.^.^.^...^.".to_string(),
                "...............".to_string(),
            ])?)?,
            21
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        Ok(())
    }
}
