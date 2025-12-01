use std::fmt;

pub mod part1;
pub mod part2;

use std::collections::VecDeque;

use anyhow::{Result, bail};

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub struct Rotation {
    pub direction: Direction,
    pub steps: usize,
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.direction {
            Direction::Left => write!(f, "L{}", self.steps),
            Direction::Right => write!(f, "R{}", self.steps),
        }
    }
}

pub fn parse_line(line: &str) -> Result<Rotation> {
    let (direction, steps) = line.split_at(1);

    let direction = match direction {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => bail!("invalid direction: {}", direction),
    };

    let steps = steps.parse::<usize>()?;

    Ok(Rotation { direction, steps })
}

pub fn parse_lines(lines: &str) -> Result<Vec<Rotation>> {
    lines.lines().map(parse_line).collect::<Result<Vec<_>>>()
}

pub fn create_dial() -> VecDeque<usize> {
    let mut dial = VecDeque::new();
    for i in 0..100 {
        dial.push_front(i);
    }
    dial.rotate_right(51);
    dial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_rotation() {
        let rotation = Rotation {
            direction: Direction::Left,
            steps: 10,
        };
        assert_eq!(rotation.to_string(), "L10");
    }

    #[test]
    fn test_parse_line_left() -> Result<()> {
        let rotation = parse_line("L10")?;
        assert_eq!(rotation.direction, Direction::Left);
        assert_eq!(rotation.steps, 10);
        assert_eq!(rotation.to_string(), "L10");
        Ok(())
    }

    #[test]
    fn test_parse_line_right() -> Result<()> {
        let rotation = parse_line("R55")?;
        assert_eq!(rotation.direction, Direction::Right);
        assert_eq!(rotation.steps, 55);
        assert_eq!(rotation.to_string(), "R55");
        Ok(())
    }

    #[test]
    fn test_create_dial() {
        let dial = create_dial();
        assert_eq!(dial.len(), 100);
        assert_eq!(dial.front(), Some(&50));
    }

    #[test]
    fn test_dial_rotation_directions() {
        let mut dial = create_dial();
        dial.rotate_right(1);
        assert_eq!(dial.front(), Some(&51)); // right = higher

        dial.rotate_left(2);
        assert_eq!(dial.front(), Some(&49)); // left = lower
    }
}
