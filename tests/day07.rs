use anyhow::Result;
use aoc::day07::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    let input = include_str!("../inputs/day07.txt");
    let result = part1::run(input)?;
    assert_eq!(result, "1605");
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = include_str!("../inputs/day07.txt");
    let result = part2::run(input)?;
    assert_eq!(result, "29893386035180");
    Ok(())
}
