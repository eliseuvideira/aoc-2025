use anyhow::Result;
use aoc::day11::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day11.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "746");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day11.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "370500293582760");
    Ok(())
}
