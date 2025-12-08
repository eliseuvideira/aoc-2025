use anyhow::Result;

use super::{can_be_accessed, parse_locations};

pub fn run(input: &str) -> Result<String> {
    let locations = parse_locations(input)?;

    let accessible_locations = locations
        .iter()
        .filter(|&(x, y)| can_be_accessed(*x, *y, &locations))
        .count();

    Ok(accessible_locations.to_string())
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
        assert_eq!(run(input)?, "13");
        Ok(())
    }
}
