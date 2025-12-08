use anyhow::Result;
use aoc::day08::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day08.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "29406");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day08.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "7499461416");
    Ok(())
}
