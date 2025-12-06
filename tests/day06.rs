use anyhow::Result;
use aoc::day06::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day06.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "5667835681547");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day06.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "9434900032651");
    Ok(())
}
