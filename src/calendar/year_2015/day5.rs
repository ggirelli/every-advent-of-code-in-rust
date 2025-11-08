use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::io::read_lines;

fn is_char_vowel(c: &char) -> bool {
    "aeiou".contains(*c)
}

fn is_string_nice(s: &str) -> Result<bool, Error> {
    let mut vowel_counter: usize = 0;
    let mut found_dimer: bool = false;
    let blacklist: [&str; 4] = ["ab", "cd", "pq", "xy"];
    let mut found_blacklist: bool = false;

    let mut previous_char: char = s.chars().next().unwrap();
    if is_char_vowel(&previous_char) {
        vowel_counter += 1;
    }

    for c in s.chars().skip(1) {
        if is_char_vowel(&c) {
            vowel_counter += 1;
        }
        if c == previous_char {
            found_dimer = true;
        }
        let dimer: String = format!("{}{}", previous_char, c);
        if blacklist.contains(&&dimer[..]) {
            found_blacklist = true;
        }

        previous_char = c;
    }

    Ok((vowel_counter >= 3) & found_dimer & (!found_blacklist))
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let mut nice_string_counter: usize = 0;
    for l in read_lines(input)? {
        if is_string_nice(&l)? {
            nice_string_counter += 1;
        }
    }
    tracing::info!("{} strings are nice", nice_string_counter);
    Ok(())
}

fn is_string_nice_v2(s: &str) -> Result<bool, Error> {
    let mut found_repeated_nonoverlapping_dimer: bool = false;
    let mut found_trimer: bool = false;
    for (p, c) in s.char_indices() {
        if p + 1 < s.len() && s[(p + 2)..].contains(&s[p..(p + 2)]) {
            found_repeated_nonoverlapping_dimer = true;
        }
        if p + 2 < s.len() && c == s[(p + 2)..].chars().next().unwrap() {
            found_trimer = true;
        }
        if found_repeated_nonoverlapping_dimer & found_trimer {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let mut nice_string_counter: usize = 0;
    for l in read_lines(input)? {
        if is_string_nice_v2(&l)? {
            nice_string_counter += 1;
        }
    }
    tracing::info!("{} strings are nice", nice_string_counter);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_string_nice() {
        assert_eq!(is_string_nice(&"ugknbfddgicrmopn").unwrap(), true);
        assert_eq!(is_string_nice(&"aaa").unwrap(), true);
        assert_eq!(is_string_nice(&"jchzalrnumimnmhp").unwrap(), false);
        assert_eq!(is_string_nice(&"haegwjzuvuyypxyu").unwrap(), false);
        assert_eq!(is_string_nice(&"dvszwmarrgswjxmb").unwrap(), false);
    }

    #[test]
    fn test_is_string_nice_v2() {
        assert_eq!(is_string_nice_v2(&"qjhvhtzxzqqjkmpb").unwrap(), true);
        assert_eq!(is_string_nice_v2(&"xxyxx").unwrap(), true);
        assert_eq!(is_string_nice_v2(&"uurcxstgmygtbstg").unwrap(), false);
        assert_eq!(is_string_nice_v2(&"ieodomkazucvgmuy").unwrap(), false);
    }
}
