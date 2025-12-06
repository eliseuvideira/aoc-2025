use std::collections::HashSet;

use anyhow::{Result, bail};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

pub type Locations = HashSet<(i32, i32)>;

pub fn parse_locations(input: &str) -> Result<Locations> {
    let mut locations = HashSet::new();
    let mut y = 0;

    for line in input.lines() {
        let mut x = 0;

        for c in line.chars() {
            match c {
                '@' => {
                    locations.insert((x, y));
                }
                '.' => (),
                _ => bail!("invalid character: {}", c),
            };

            x += 1;
        }

        y += 1;
    }

    Ok(locations)
}

pub fn get_adjacent_positions(x: i32, y: i32) -> [(i32, i32); 8] {
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

pub fn can_be_accessed(x: i32, y: i32, locations: &Locations) -> bool {
    locations.contains(&(x, y))
        && get_adjacent_positions(x, y)
            .iter()
            .filter(|pos| locations.contains(pos))
            .count()
            < 4
}
