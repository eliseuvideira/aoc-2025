use anyhow::Result;
use aoc::day03::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day03.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "17087");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day03.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "169019504359949");
    Ok(())
}
