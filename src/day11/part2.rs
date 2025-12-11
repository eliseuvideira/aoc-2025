use std::collections::HashMap;

use anyhow::{Context, Result};

use super::parse_input;

const START_DEVICE: &str = "svr";
const END_DEVICE: &str = "out";
const NECESSARY_DEVICES: [&str; 2] = ["dac", "fft"];

fn count_possible_paths(
    device_outputs: &HashMap<String, Vec<String>>,
    device: &str,
    seen: (bool, bool),
    memo: &mut HashMap<(String, (bool, bool)), u64>,
) -> Result<u64> {
    let seen_0 = seen.0 || device == NECESSARY_DEVICES[0];
    let seen_1 = seen.1 || device == NECESSARY_DEVICES[1];

    if device == END_DEVICE {
        return Ok(if seen_0 && seen_1 { 1 } else { 0 });
    }

    if let Some(count) = memo.get(&(device.to_string(), (seen_0, seen_1))) {
        return Ok(*count);
    }

    let outputs = device_outputs.get(device).context("device not found")?;

    let mut count = 0;
    for output in outputs {
        count += count_possible_paths(device_outputs, output, (seen_0, seen_1), memo)?;
    }

    memo.insert((device.to_string(), (seen_0, seen_1)), count);
    Ok(count)
}

pub fn run(input: &str) -> Result<String> {
    let device_outputs = parse_input(input)?;

    let possible_paths = count_possible_paths(
        &device_outputs,
        START_DEVICE,
        (false, false),
        &mut HashMap::new(),
    )?;

    Ok(possible_paths.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(run(input)?, "2".to_string());

        Ok(())
    }
}
