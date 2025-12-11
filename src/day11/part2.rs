use std::collections::HashMap;

use anyhow::{Context, Result};

use super::parse_input;

const START_DEVICE: &str = "svr";
const END_DEVICE: &str = "out";
const NECESSARY_DEVICES: [&str; 2] = ["dac", "fft"];

fn count_possible_paths<'a>(
    device_outputs: &HashMap<&'a str, Vec<&'a str>>,
    device: &'a str,
    seen: [bool; 2],
    memo: &mut HashMap<(&'a str, [bool; 2]), u64>,
) -> Result<u64> {
    let seen = [
        seen[0] || device == NECESSARY_DEVICES[0],
        seen[1] || device == NECESSARY_DEVICES[1],
    ];

    if device == END_DEVICE {
        return Ok(if seen[0] && seen[1] { 1 } else { 0 });
    }

    if let Some(&count) = memo.get(&(device, seen)) {
        return Ok(count);
    }

    let outputs = device_outputs.get(device).context("device not found")?;

    let mut count = 0;
    for output in outputs {
        count += count_possible_paths(device_outputs, output, seen, memo)?;
    }

    memo.insert((device, seen), count);
    Ok(count)
}

pub fn run(input: &str) -> Result<String> {
    let device_outputs = parse_input(input)?;

    let possible_paths = count_possible_paths(
        &device_outputs,
        START_DEVICE,
        [false, false],
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
