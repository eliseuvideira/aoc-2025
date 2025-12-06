use anyhow::{Context, Result};
use std::fmt;

pub mod part1;
pub mod part2;

pub struct Range {
    pub min: usize,
    pub max: usize,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..={}", self.min, self.max)
    }
}

fn parse_range(line: &str) -> Result<Range> {
    let (min, max) = line
        .split_once('-')
        .context(format!("invalid range: {}", line))?;
    Ok(Range {
        min: min.parse::<usize>().context(format!("invalid min: {}", min))?,
        max: max.parse::<usize>().context(format!("invalid max: {}", max))?,
    })
}

pub fn parse_ranges(input: &str) -> Result<Vec<Range>> {
    input
        .trim()
        .split(',')
        .map(parse_range)
        .collect::<Result<Vec<_>>>()
}

pub fn digit_count(number: usize) -> usize {
    let mut count = 0;
    let mut n = number;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}
