use anyhow::Result;
use aoc::day09::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day09.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "4733727792");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day09.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "1566346198");
    Ok(())
}
