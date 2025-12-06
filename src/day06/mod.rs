use std::str::FromStr;

use anyhow::bail;

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operator::Multiply),
            "+" => Ok(Operator::Add),
            _ => bail!("invalid operator: {}", s),
        }
    }
}
