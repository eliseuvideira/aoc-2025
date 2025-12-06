use anyhow::Result;
use aoc::day02::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day02.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "19605500130");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day02.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "36862281418");
    Ok(())
}
