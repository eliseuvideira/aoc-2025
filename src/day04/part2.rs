use anyhow::Result;

use super::{Locations, can_be_accessed, parse_locations};

fn get_possible_moves(locations: &Locations) -> Vec<(i32, i32)> {
    locations
        .iter()
        .filter(|&(x, y)| can_be_accessed(*x, *y, locations))
        .copied()
        .collect()
}

pub fn run(input: &str) -> Result<String> {
    let mut locations = parse_locations(input)?;

    let mut total_moves = 0;
    loop {
        let moves = get_possible_moves(&locations);
        if moves.is_empty() {
            break;
        }

        total_moves += moves.len();

        for (x, y) in moves {
            locations.remove(&(x, y));
        }
    }

    Ok(total_moves.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(run(input)?, "43");
        Ok(())
    }
}
