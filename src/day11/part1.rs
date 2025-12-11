use std::collections::HashMap;

use anyhow::{Context, Result};

use super::parse_input;

const START_DEVICE: &str = "you";
const END_DEVICE: &str = "out";

fn count_possible_paths(
    device_outputs: &HashMap<&str, Vec<&str>>,
    device: &str,
) -> Result<u32> {
    if device == END_DEVICE {
        return Ok(1);
    }

    let outputs = device_outputs.get(device).context("device not found")?;

    let mut count = 0;
    for output in outputs {
        count += count_possible_paths(device_outputs, output)?;
    }

    Ok(count)
}

pub fn run(input: &str) -> Result<String> {
    let device_outputs = parse_input(input)?;

    let possible_paths = count_possible_paths(&device_outputs, START_DEVICE)?;

    Ok(possible_paths.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(run(input)?, "5".to_string());

        Ok(())
    }
}
