use std::path::PathBuf;

use anyhow::{Error, Result};
use pretty_assertions::assert_eq;

use crate::io::read_lines;

fn parse_range(range_str: String) -> Result<(usize, usize), Error> {
    let parsed_range: Vec<usize> = range_str
        .split("-")
        .map(|s| -> Result<usize, Error> { Ok(s.parse::<usize>()?) })
        .collect::<Result<Vec<usize>>>()?;
    assert_eq!(parsed_range.len(), 2);
    Ok((parsed_range[0], parsed_range[1]))
}

fn is_ingredient_fresh(
    ingredient_id: usize,
    fresh_ranges: &Vec<(usize, usize)>,
) -> Result<bool, Error> {
    for range in fresh_ranges {
        if (ingredient_id <= range.1) & (ingredient_id >= range.0) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn count_fresh_ingredients(database: Vec<String>) -> Result<usize, Error> {
    let mut num_fresh_ingredients: usize = 0;

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut parsed_all_ranges: bool = false;
    for line in database {
        if line == "" {
            parsed_all_ranges = true;
            continue;
        }
        match parsed_all_ranges {
            false => {
                ranges.push(parse_range(line)?);
            }
            true => {
                if is_ingredient_fresh(line.parse::<usize>()?, &ranges)? {
                    num_fresh_ingredients += 1;
                }
            }
        }
    }

    Ok(num_fresh_ingredients)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let num_fresh_ingredients: usize = count_fresh_ingredients(read_lines(input)?)?;
    tracing::info!("Found {} fresh ingredients.", num_fresh_ingredients);
    Ok(())
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    Err(Error::msg("Not implemented."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            count_fresh_ingredients(vec![
                "3-5".to_string(),
                "10-14".to_string(),
                "16-20".to_string(),
                "12-18".to_string(),
                "".to_string(),
                "1".to_string(),
                "5".to_string(),
                "8".to_string(),
                "11".to_string(),
                "17".to_string(),
                "32".to_string(),
            ])?,
            3
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        Ok(())
    }
}
