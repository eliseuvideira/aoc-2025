use anyhow::{Context, Result};

use super::Operator;

fn parse_operators(last_line: &str) -> Result<Vec<Operator>> {
    last_line.split_whitespace().map(|op| op.parse()).collect()
}

fn parse_operands(lines: &[&str]) -> Result<Vec<Vec<u64>>> {
    let num_columns = lines
        .first()
        .map(|line| line.split_whitespace().count())
        .unwrap_or(0);

    let mut operands: Vec<Vec<u64>> = vec![Vec::new(); num_columns];

    for line in lines {
        for (i, operand) in line.split_whitespace().enumerate() {
            let operand = operand.parse::<u64>()?;
            operands[i].push(operand);
        }
    }

    Ok(operands)
}

fn parse_input(input: &str) -> Result<(Vec<Vec<u64>>, Vec<Operator>)> {
    let mut lines = input.lines().collect::<Vec<_>>();
    let last_line = lines.pop().context("no last line")?;

    let operands = parse_operands(&lines)?;
    let operators = parse_operators(last_line)?;

    Ok((operands, operators))
}

pub fn run(input: &str) -> Result<String> {
    let (operands, operators) = parse_input(input)?;

    let sum = operands
        .into_iter()
        .zip(operators)
        .map(|(operands, operator)| match operator {
            Operator::Add => operands.into_iter().sum::<u64>(),
            Operator::Multiply => operands.into_iter().product::<u64>(),
        })
        .sum::<u64>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!(run(input)?, "4277556".to_string());

        Ok(())
    }
}
