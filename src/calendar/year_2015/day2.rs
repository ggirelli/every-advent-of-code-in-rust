use anyhow::{Error, Result};
use pretty_assertions::assert_eq;
use std::path::PathBuf;

use crate::io::read_lines;

/// Calculate the surface of a single box.
fn get_box_surface(line: &str) -> Result<i32, Error> {
    let mut total_box_surface: i32 = 0;

    let sides: Vec<i32> = line
        .split(&"x")
        .map(|d| d.parse::<i32>().unwrap())
        .collect();
    assert_eq!(sides.len(), 3);

    let length: &i32 = sides.first().unwrap();
    let width: &i32 = sides.get(1).unwrap();
    let height: &i32 = sides.get(2).unwrap();

    let prods: [i32; 3] = [length * width, *width * height, *length * height];
    let slack: i32 = *prods.iter().min().unwrap();

    total_box_surface += prods.iter().sum::<i32>() * 2;
    total_box_surface += slack;

    Ok(total_box_surface)
}

// Find total wrapping paper area.
pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let mut total_surface: i32 = 0;
    for line in read_lines(input)? {
        total_surface += get_box_surface(&line)?;
    }
    tracing::info!("Total surface: {}", total_surface);
    Ok(())
}

/// Calculate the smalles box face perimeter.
fn get_smallest_box_face_perimeter(line: &str) -> Result<i32, Error> {
    let mut sides_vec: Vec<i32> = line
        .split(&"x")
        .map(|d| d.parse::<i32>().unwrap())
        .collect();
    assert_eq!(sides_vec.len(), 3);
    sides_vec.sort();

    let sides: [i32; 3] = [
        *sides_vec.first().unwrap(),
        *sides_vec.get(1).unwrap(),
        *sides_vec.get(2).unwrap(),
    ];
    let wrap_length: i32 = 2 * sides[0] + 2 * sides[1];
    let ribbon_length: i32 = sides.iter().product::<i32>();

    Ok(wrap_length + ribbon_length)
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let mut total_length: i32 = 0;
    for line in read_lines(input)? {
        total_length += get_smallest_box_face_perimeter(&line)?;
    }
    tracing::info!("Total length: {}", total_length);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_box_surface() {
        assert_eq!(get_box_surface(&"2x3x4").unwrap(), 58);
        assert_eq!(get_box_surface(&"1x1x10").unwrap(), 43);
    }

    #[test]
    fn test_ribbon_feet_for_box() {
        assert_eq!(get_smallest_box_face_perimeter(&"2x3x4").unwrap(), 34);
        assert_eq!(get_smallest_box_face_perimeter(&"1x1x10").unwrap(), 14);
    }
}
