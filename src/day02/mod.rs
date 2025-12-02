use anyhow::Result;
use std::fmt;

pub mod part1;
pub mod part2;

pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..={}", self.start, self.end)
    }
}

fn parse_range(line: &str) -> Result<Range> {
    let (start, end) = line
        .split_once('-')
        .ok_or(anyhow::anyhow!("invalid range: {}", line))?;
    Ok(Range {
        start: start.parse::<usize>()?,
        end: end.parse::<usize>()?,
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
