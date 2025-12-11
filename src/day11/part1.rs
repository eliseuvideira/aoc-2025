use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use super::parse_input;

const START_DEVICE: &str = "you";
const END_DEVICE: &str = "out";

fn count_possible_paths(
    device_outputs: &HashMap<String, Vec<String>>,
    device: &str,
    seen: &mut HashSet<Vec<String>>,
    current_path: &mut Vec<String>,
) -> Result<u32> {
    if device == END_DEVICE {
        return Ok(1);
    }

    if seen.contains(current_path) {
        return Ok(0);
    }

    seen.insert(current_path.clone());

    let outputs = device_outputs.get(device).context("device not found")?;

    let mut count = 0;
    for output in outputs {
        current_path.push(output.clone());
        count += count_possible_paths(device_outputs, output, seen, current_path)?;
        current_path.pop();
    }

    Ok(count)
}

pub fn run(input: &str) -> Result<String> {
    let device_outputs = parse_input(input)?;

    let possible_paths = count_possible_paths(
        &device_outputs,
        START_DEVICE,
        &mut HashSet::new(),
        &mut Vec::new(),
    )?;

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
