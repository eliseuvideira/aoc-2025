use std::collections::HashMap;

use anyhow::Result;

use super::{all_pairs_by_distance, find_circuit, join_circuits, parse_positions, Position};

fn collect_circuit_sizes(circuit_of: &mut HashMap<Position, Position>) -> Vec<usize> {
    let positions: Vec<_> = circuit_of.keys().copied().collect();
    let mut size_by_circuit: HashMap<Position, usize> = HashMap::new();
    for position in positions {
        let circuit = find_circuit(position, circuit_of);
        *size_by_circuit.entry(circuit).or_default() += 1;
    }
    size_by_circuit.into_values().collect()
}

pub fn run(input: &str) -> Result<String> {
    let positions = parse_positions(input)?;
    let connection_count = if positions.len() > 20 { 1000 } else { 10 };

    let pairs = all_pairs_by_distance(&positions);

    let mut circuit_of: HashMap<Position, Position> =
        positions.iter().map(|&p| (p, p)).collect();

    for (_, p1, p2) in pairs.into_iter().take(connection_count) {
        join_circuits(p1, p2, &mut circuit_of);
    }

    let mut sizes = collect_circuit_sizes(&mut circuit_of);
    sizes.sort_by_key(|&s| std::cmp::Reverse(s));

    let result = sizes.iter().take(3).product::<usize>();
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
        assert_eq!(run(input)?, "40".to_string());

        Ok(())
    }
}
