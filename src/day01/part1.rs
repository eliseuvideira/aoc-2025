use anyhow::Result;

use super::{Direction, create_dial, parse_lines};

pub fn run(input: &str) -> Result<String> {
    let rotations = parse_lines(input)?;
    let mut dial = create_dial();

    let mut times_at_zero = 0;
    for rotation in rotations {
        let steps = rotation.steps % 100;

        match rotation.direction {
            Direction::Left => dial.rotate_left(steps),
            Direction::Right => dial.rotate_right(steps),
        }

        if dial.front() == Some(&0) {
            times_at_zero += 1;
        }
    }

    Ok(times_at_zero.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1_example() -> Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(run(input)?, "3".to_string());

        Ok(())
    }
}
