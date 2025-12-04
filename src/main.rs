#![cfg_attr(test, feature(test))]

use anyhow::{Result, bail};
use clap::Parser;

#[derive(Parser)]
#[command(about = "Advent of Code 2025")]
struct Args {
    /// Day (1-12)
    day: u8,
    /// Part (1-2)
    part: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.day < 1 || args.day > 12 {
        bail!("day must be between 1 and 12");
    }
    if args.part < 1 || args.part > 2 {
        bail!("part must be 1 or 2");
    }

    let result = match (args.day, args.part) {
        (1, 1) => day01::part1::run(include_str!("../inputs/day01.txt"))?,
        (1, 2) => day01::part2::run(include_str!("../inputs/day01.txt"))?,
        (2, 1) => day02::part1::run(include_str!("../inputs/day02.txt"))?,
        (2, 2) => day02::part2::run(include_str!("../inputs/day02.txt"))?,
        (3, 1) => day03::part1::run(include_str!("../inputs/day03.txt"))?,
        (3, 2) => day03::part2::run(include_str!("../inputs/day03.txt"))?,
        (4, 1) => day04::part1::run(include_str!("../inputs/day04.txt"))?,
        (4, 2) => day04::part2::run(include_str!("../inputs/day04.txt"))?,
        (5, 1) => day05::part1()?,
        (5, 2) => day05::part2()?,
        (6, 1) => day06::part1()?,
        (6, 2) => day06::part2()?,
        (7, 1) => day07::part1()?,
        (7, 2) => day07::part2()?,
        (8, 1) => day08::part1()?,
        (8, 2) => day08::part2()?,
        (9, 1) => day09::part1()?,
        (9, 2) => day09::part2()?,
        (10, 1) => day10::part1()?,
        (10, 2) => day10::part2()?,
        (11, 1) => day11::part1()?,
        (11, 2) => day11::part2()?,
        (12, 1) => day12::part1()?,
        (12, 2) => day12::part2()?,
        _ => unreachable!(),
    };

    println!("{}", result);

    Ok(())
}

mod day01;

mod day02;

mod day03;

mod day04;

mod day05 {
    use anyhow::Result;
    pub fn part1() -> Result<String> {
        todo!()
    }
    pub fn part2() -> Result<String> {
        todo!()
    }
}

mod day06 {
    use anyhow::Result;
    pub fn part1() -> Result<String> {
        todo!()
    }
    pub fn part2() -> Result<String> {
        todo!()
    }
}

mod day07 {
    use anyhow::Result;
    pub fn part1() -> Result<String> {
        todo!()
    }
    pub fn part2() -> Result<String> {
        todo!()
    }
}

mod day08 {
    use anyhow::Result;
    pub fn part1() -> Result<String> {
        todo!()
    }
    pub fn part2() -> Result<String> {
        todo!()
    }
}

mod day09 {
    use anyhow::Result;
    pub fn part1() -> Result<String> {
        todo!()
    }
    pub fn part2() -> Result<String> {
        todo!()
    }
}

mod day10 {
    use anyhow::Result;
    pub fn part1() -> Result<String> {
        todo!()
    }
    pub fn part2() -> Result<String> {
        todo!()
    }
}

mod day11 {
    use anyhow::Result;
    pub fn part1() -> Result<String> {
        todo!()
    }
    pub fn part2() -> Result<String> {
        todo!()
    }
}

mod day12 {
    use anyhow::Result;
    pub fn part1() -> Result<String> {
        todo!()
    }
    pub fn part2() -> Result<String> {
        todo!()
    }
}
