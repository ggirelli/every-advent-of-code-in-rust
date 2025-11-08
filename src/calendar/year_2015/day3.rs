use std::{collections::HashMap, path::PathBuf};

use anyhow::{Error, Result};

use crate::io::read;

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
}
