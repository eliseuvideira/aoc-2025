use std::collections::HashSet;

use anyhow::{Result, bail};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

pub type Locations = HashSet<(i32, i32)>;

const ADJACENT_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn parse_locations(input: &str) -> Result<Locations> {
    let mut locations = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    locations.insert((x as i32, y as i32));
                }
                '.' => (),
                _ => bail!("invalid character: {}", c),
            };
        }
    }

    Ok(locations)
}

pub fn can_be_accessed(x: i32, y: i32, locations: &Locations) -> bool {
    if !locations.contains(&(x, y)) {
        return false;
    }

    let mut count = 0;
    for &(dx, dy) in &ADJACENT_OFFSETS {
        if locations.contains(&(x + dx, y + dy)) {
            count += 1;
            if count >= 4 {
                return false;
            }
        }
    }

    true
}
