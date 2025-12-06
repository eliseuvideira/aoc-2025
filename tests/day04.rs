use anyhow::Result;
use aoc::day04::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day04.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "1540");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day04.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "8972");
    Ok(())
}
