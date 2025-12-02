use anyhow::{Error, Result};
use std::path::PathBuf;

use crate::io::read;

fn is_id_invalid(id: &str) -> Result<bool, Error> {
    if id.chars().next() == Some('0') {
        return Ok(false);
    }
    if id.len() % 2 == 0 {
        if id[0..(id.len() / 2)] == id[(id.len() / 2)..id.len()] {
            return Ok(true);
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

fn sum_invalid_ids(payload: String) -> Result<usize, Error> {
    let mut invalid_sum: usize = 0;
    for pair in payload.split(',') {
        dbg!(pair);
        let id_extremes: Vec<&str> = pair.split('-').collect();
        assert_eq!(id_extremes.len(), 2);
        for id_num in id_extremes
            .first()
            .ok_or(Error::msg("Error retrieving first ID."))?
            .parse::<usize>()?
            ..(id_extremes
                .last()
                .ok_or(Error::msg("Error retrieving second ID."))?
                .parse::<usize>()?
                + 1)
        {
            if is_id_invalid(&format!("{}", id_num))? {
                invalid_sum += id_num;
            }
        }
    }
    Ok(invalid_sum)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let answer: usize = sum_invalid_ids(read(input)?)?;
    tracing::info!("Invalid IDs sum: {}", answer);
    Ok(())
}

// pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() -> Result<(), Error> {
        assert_eq!(
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
                    .to_string()
            )?,
            1227775554
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Error> {
        Ok(())
    }
}
