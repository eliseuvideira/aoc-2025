use std::collections::HashMap;

use anyhow::{Context, Result};

use super::{all_pairs_by_distance, join_circuits, parse_positions, Position};

fn find_last_connection(
    pairs: &[(u64, Position, Position)],
    circuit_of: &mut HashMap<Position, Position>,
) -> Option<(Position, Position)> {
    let mut circuit_count = circuit_of.len();

    for &(_, p1, p2) in pairs {
        if join_circuits(p1, p2, circuit_of) {
            circuit_count -= 1;
            if circuit_count == 1 {
                return Some((p1, p2));
            }
        }
    }
    None
}

pub fn run(input: &str) -> Result<String> {
    let positions = parse_positions(input)?;
    let pairs = all_pairs_by_distance(&positions);

    let mut circuit_of: HashMap<Position, Position> =
        positions.iter().map(|&p| (p, p)).collect();

    let (p1, p2) =
        find_last_connection(&pairs, &mut circuit_of).context("no last connection found")?;

    let result = p1.0 * p2.0;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(run(input)?, "25272".to_string());

        Ok(())
    }
}
