use anyhow::Result;
use aoc::day05::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day05.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "525");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day05.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "333892124923577");
    Ok(())
}
