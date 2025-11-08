use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::io::read;

fn get_lowest_integer(secret_key: &str, starts_with: &str) -> Result<u32, Error> {
    let mut counter: u32 = 0;
    loop {
        let digest: md5::Digest = md5::compute(format!("{}{}", secret_key, counter));

        if format!("{:x}", digest).starts_with(starts_with) {
            break;
        }

        counter += 1;
    }
    Ok(counter)
}

pub fn run_part_1(input: PathBuf) -> Result<(), Error> {
    let counter: u32 = get_lowest_integer(&read(input)?, "00000")?;
    tracing::info!("{}", counter);
    Ok(())
}

pub fn run_part_2(input: PathBuf) -> Result<(), Error> {
    let counter: u32 = get_lowest_integer(&read(input)?, "000000")?;
    tracing::info!("{}", counter);
    Ok(())
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn test_get_lowest_integer() {
//         assert_eq!(get_lowest_integer(&"abcdef", "00000").unwrap(), 609043);
//         assert_eq!(get_lowest_integer(&"pqrstuv", "00000").unwrap(), 1048970);
//     }
// }
