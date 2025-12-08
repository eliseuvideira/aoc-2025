use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::day07::{Position, parse_input};

fn count_worlds(
    pos: Position,
    splitters: &HashSet<Position>,
    (max_x, max_y): Position,
    cache: &mut HashMap<Position, u64>,
) -> u64 {
    let (x, y) = pos;

    for dy in 1..=(max_y - y) {
        if splitters.contains(&(x, y + dy)) {
            let left = match cache.get(&(x - 1, y + dy)) {
                Some(count) => *count,
                None if x > 0 => count_worlds((x - 1, y + dy), splitters, (max_x, max_y), cache),
                None => 0,
            };
            let right = match cache.get(&(x + 1, y + dy)) {
                Some(count) => *count,
                None if x < max_x => {
                    count_worlds((x + 1, y + dy), splitters, (max_x, max_y), cache)
                }
                None => 0,
            };

            cache.insert((x, y), left + right);
            return left + right;
        }
    }

    cache.insert((x, y), 1);
    1
}

pub fn run(input: &str) -> Result<String> {
    let (tachyon_start, splitters, (max_x, max_y)) = parse_input(input)?;

    let mut cache = HashMap::new();
    let total_worlds = count_worlds(tachyon_start, &splitters, (max_x, max_y), &mut cache);

    Ok(total_worlds.to_string())
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
        assert_eq!(run(input)?, "40".to_string());

        Ok(())
    }
}
