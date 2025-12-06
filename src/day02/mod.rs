use anyhow::{Context, Result};
use std::ops::RangeInclusive;

pub mod part1;
pub mod part2;

fn parse_range(line: &str) -> Result<RangeInclusive<usize>> {
    let (min, max) = line
        .split_once('-')
        .context(format!("invalid range: {}", line))?;

    let min = min
        .parse::<usize>()
        .context(format!("invalid min: {}", min))?;
    let max = max
        .parse::<usize>()
        .context(format!("invalid max: {}", max))?;

    Ok(min..=max)
}

pub fn parse_ranges(input: &str) -> Result<Vec<RangeInclusive<usize>>> {
    input
        .trim()
        .split(',')
        .map(parse_range)
        .collect::<Result<Vec<_>>>()
}

pub fn digit_count(number: usize) -> usize {
    if number == 0 {
        return 1;
    }
    number.ilog10() as usize + 1
}
