use std::collections::HashMap;

use anyhow::{Context, Result};

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

pub type Position = (u64, u64, u64);

pub fn parse_positions(input: &str) -> Result<Vec<Position>> {
    let mut positions = Vec::new();

    for line in input.lines() {
        let (x, rhs) = line.split_once(',').context("invalid position")?;
        let (y, z) = rhs.split_once(',').context("invalid position")?;
        positions.push((x.parse::<u64>()?, y.parse::<u64>()?, z.parse::<u64>()?));
    }

    Ok(positions)
}

pub fn distance_squared(p1: Position, p2: Position) -> u64 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2)
}

pub fn all_pairs_by_distance(positions: &[Position]) -> Vec<(u64, Position, Position)> {
    let mut pairs = Vec::new();
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let distance = distance_squared(positions[i], positions[j]);
            pairs.push((distance, positions[i], positions[j]));
        }
    }
    pairs.sort_by_key(|(d, _, _)| *d);
    pairs
}

pub fn find_circuit(position: Position, circuit_of: &mut HashMap<Position, Position>) -> Position {
    let parent = circuit_of[&position];
    if parent == position {
        return position;
    }
    let root = find_circuit(parent, circuit_of);
    circuit_of.insert(position, root);
    root
}

pub fn join_circuits(
    p1: Position,
    p2: Position,
    circuit_of: &mut HashMap<Position, Position>,
) -> bool {
    let circuit1 = find_circuit(p1, circuit_of);
    let circuit2 = find_circuit(p2, circuit_of);
    if circuit1 != circuit2 {
        circuit_of.insert(circuit1, circuit2);
        return true;
    }
    false
}
