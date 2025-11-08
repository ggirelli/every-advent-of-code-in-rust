use std::{collections::HashMap, path::PathBuf};

use anyhow::{Error, Result};

use crate::io::read;

// Count visited houses.
fn count_visited_houses(path: &str) -> Result<usize, Error> {
    let mut visited_houses: HashMap<[i32; 2], i32> = HashMap::new();
    let mut current_position: [i32; 2] = [0, 0];

    match visited_houses.get(&current_position) {
        Some(v) => visited_houses.insert(current_position, v + 1),
        None => visited_houses.insert(current_position, 1),
    };

    for c in path.chars() {
        match c {
            '>' => current_position[0] += 1,
            '^' => current_position[1] += 1,
            '<' => current_position[0] -= 1,
            'v' => current_position[1] -= 1,
            _ => panic!("Unsupported char: '{}'", c),
        }

        match visited_houses.get(&current_position) {
            Some(v) => visited_houses.insert(current_position, v + 1),
            None => visited_houses.insert(current_position, 1),
        };
    }

    Ok(visited_houses.len())
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let num_visited_houses: usize = count_visited_houses(&read(input)?)?;
    tracing::info!("Visited houses: {}", num_visited_houses);
    Ok(())
}

// Count visited houses with RoboSanta's help.
fn count_visited_houses_with_robosanta(path: &str) -> Result<usize, Error> {
    let mut visited_houses: HashMap<[i32; 2], i32> = HashMap::new();
    let mut current_position: [i32; 2] = [0, 0];
    let mut current_position_robo: [i32; 2] = [0, 0];

    match visited_houses.get(&current_position) {
        Some(v) => visited_houses.insert(current_position, v + 1),
        None => visited_houses.insert(current_position, 1),
    };

    for (i, c) in path.chars().enumerate() {
        match i % 2 {
            0 => {
                match c {
                    '>' => current_position[0] += 1,
                    '^' => current_position[1] += 1,
                    '<' => current_position[0] -= 1,
                    'v' => current_position[1] -= 1,
                    _ => panic!("Unsupported char: '{}'", c),
                }

                match visited_houses.get(&current_position) {
                    Some(v) => visited_houses.insert(current_position, v + 1),
                    None => visited_houses.insert(current_position, 1),
                };
            }
            1 => {
                match c {
                    '>' => current_position_robo[0] += 1,
                    '^' => current_position_robo[1] += 1,
                    '<' => current_position_robo[0] -= 1,
                    'v' => current_position_robo[1] -= 1,
                    _ => panic!("Unsupported char: '{}'", c),
                }

                match visited_houses.get(&current_position_robo) {
                    Some(v) => visited_houses.insert(current_position_robo, v + 1),
                    None => visited_houses.insert(current_position_robo, 1),
                };
            }
            _ => panic!("Unreachable code."),
        };
    }

    Ok(visited_houses.len())
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let num_visited_houses: usize = count_visited_houses_with_robosanta(&read(input)?)?;
    tracing::info!("Visited houses with RoboSanta: {}", num_visited_houses);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_visited_houses() {
        assert_eq!(count_visited_houses(&">").unwrap(), 2);
        assert_eq!(count_visited_houses(&"^>v<").unwrap(), 4);
        assert_eq!(count_visited_houses(&"^v^v^v^v^v").unwrap(), 2);
    }

    #[test]
    fn test_visited_houses_with_robosanta() {
        assert_eq!(count_visited_houses_with_robosanta(&"^v").unwrap(), 3);
        assert_eq!(count_visited_houses_with_robosanta(&"^>v<").unwrap(), 3);
        assert_eq!(
            count_visited_houses_with_robosanta(&"^v^v^v^v^v").unwrap(),
            11
        );
    }
}
