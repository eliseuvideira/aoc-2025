use std::collections::HashMap;

use anyhow::{Context, Result};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

pub fn parse_input(input: &str) -> Result<HashMap<&str, Vec<&str>>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (key, value) = line.split_once(':').context("expected colon")?;
        let value: Vec<_> = value.trim_start().split_whitespace().collect();
        map.insert(key, value);
    }

    Ok(map)
}
