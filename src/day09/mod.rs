use anyhow::{Context, Result};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub x: u64,
    pub y: u64,
}

pub fn parse_tiles(input: &str) -> Result<Vec<Tile>> {
    let mut tiles = Vec::new();

    for line in input.lines() {
        let (x, y) = line.split_once(',').context("invalid tile")?;
        tiles.push(Tile {
            x: x.parse::<u64>()?,
            y: y.parse::<u64>()?,
        });
    }

    Ok(tiles)
}

pub fn get_tile_area(t1: &Tile, t2: &Tile) -> u64 {
    (t1.x.abs_diff(t2.x) + 1) * (t1.y.abs_diff(t2.y) + 1)
}
