use std::collections::HashSet;

use anyhow::{Context, Result};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

pub type Position = (u64, u64);

pub fn parse_input(input: &str) -> Result<(Position, HashSet<Position>, Position)> {
    let mut tachyon_start = None;
    let mut splitters = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => tachyon_start = Some((x as u64, y as u64)),
                '^' => {
                    splitters.insert((x as u64, y as u64));
                }
                _ => (),
            }

            max_x = max_x.max(x as u64);
        }

        max_y = max_y.max(y as u64);
    }

    let tachyon_start = tachyon_start.context("no tachyon start")?;

    Ok((tachyon_start, splitters, (max_x, max_y)))
}
