use anyhow::{Context, Result};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

pub fn parse_lines(input: &str) -> Result<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .context(format!("invalid digit: {}", c))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
}

pub fn find_index(digits: &[u8], number: u8) -> Option<usize> {
    digits.iter().position(|&d| d == number)
}
