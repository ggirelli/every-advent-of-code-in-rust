use anyhow::{Error, Result};
use pretty_assertions::assert_eq;
use std::path::PathBuf;

use crate::io::read_lines;

/// Calculate the surface of a single box.
fn get_box_surface(line: &String) -> Result<i32, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_total_box_area() {
        assert_eq!(get_box_surface(&"2x3x4".to_string()).unwrap(), 58);
        assert_eq!(get_box_surface(&"1x1x10".to_string()).unwrap(), 43);
    }
}
