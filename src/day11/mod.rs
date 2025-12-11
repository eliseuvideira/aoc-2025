use std::collections::HashMap;

use anyhow::{Context, Result};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

pub fn parse_input(input: &str) -> Result<HashMap<String, Vec<String>>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (key, value) = line.split_once(':').context("expected colon")?;
        let value = value
            .trim_start()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        map.insert(key.to_string(), value);
    }

    Ok(map)
}
