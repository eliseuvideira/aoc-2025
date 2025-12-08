use anyhow::{Context, Result};

use super::Operator;

fn parse_operators(last_line: &str) -> Result<Vec<Operator>> {
    last_line
        .split_whitespace()
        .rev()
        .map(|op| op.parse())
        .collect()
}

fn parse_operands(lines: &[&str], num_operators: usize) -> Result<Vec<Vec<u64>>> {
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut operands: Vec<Vec<u64>> = vec![Vec::new(); num_operators];
    let mut operation_index = 0;

    for col_idx in (0..max_width).rev() {
        let column_chars: String = lines
            .iter()
            .filter_map(|line| line.chars().nth(col_idx))
            .collect();

        if column_chars.trim().is_empty() {
            operation_index += 1;
            continue;
        }

        for num_str in column_chars.split_whitespace() {
            let num = num_str.parse::<u64>()?;
            operands[operation_index].push(num);
        }
    }

    Ok(operands)
}

fn parse_input(input: &str) -> Result<(Vec<Vec<u64>>, Vec<Operator>)> {
    let mut lines = input.lines().collect::<Vec<_>>();
    let last_line = lines.pop().context("no last line")?;

    let operators = parse_operators(last_line)?;
    let operands = parse_operands(&lines, operators.len())?;

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
        assert_eq!(run(input)?, "3263827".to_string());

        Ok(())
    }
}
