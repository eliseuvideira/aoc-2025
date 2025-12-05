use std::fmt;

use anyhow::{Context, Result};

#[cfg(test)]
mod bench;
pub mod part1;
pub mod part2;

#[derive(Clone, Copy)]
pub struct Range {
    pub min: u64,
    pub max: u64,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..={}", self.min, self.max)
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..={}", self.min, self.max)
    }
}

pub fn parse_database(input: &str) -> Result<(Vec<Range>, Vec<u64>)> {
    let mut lines = input.lines();

    let mut ranges = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (min, max) = line
            .split_once('-')
            .ok_or(anyhow::anyhow!("invalid range: {}", line))?;
        let range = Range {
            min: min
                .parse::<u64>()
                .context(format!("invalid min: {}", min))?,
            max: max
                .parse::<u64>()
                .context(format!("invalid max: {}", max))?,
        };

        ranges.push(range);
    }

    let mut ingredients = Vec::new();
    for line in lines {
        let number = line.parse::<u64>()?;
        ingredients.push(number);
    }

    Ok((ranges, ingredients))
}
