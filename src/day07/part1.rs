use std::collections::HashSet;

use anyhow::Result;

use super::{Position, parse_input};

fn count_splitters(
    position: Position,
    splitters: &HashSet<Position>,
    (max_x, max_y): Position,
    seen_positions: &mut HashSet<Position>,
) -> u64 {
    let (x, y) = position;
    for dy in 1..=(max_y - y) {
        if seen_positions.contains(&(x, y + dy)) {
            return 0;
        }

        seen_positions.insert((x, y + dy));

        if splitters.contains(&(x, y + dy)) {
            let left = if x > 0 {
                count_splitters((x - 1, y + dy), splitters, (max_x, max_y), seen_positions)
            } else {
                0
            };
            let right = if x < max_x {
                count_splitters((x + 1, y + dy), splitters, (max_x, max_y), seen_positions)
            } else {
                0
            };
            return left + right + 1;
        }
    }

    0
}

pub fn run(input: &str) -> Result<String> {
    let (tachyon_start, splitters, (max_x, max_y)) = parse_input(input)?;

    let total_splits = count_splitters(
        tachyon_start,
        &splitters,
        (max_x, max_y),
        &mut HashSet::new(),
    );

    Ok(total_splits.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(run(input)?, "21".to_string());

        Ok(())
    }
}
