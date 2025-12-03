use anyhow::Result;

use super::{find_index, parse_lines};

fn get_max_jolt(digits: &[u8]) -> u64 {
    for first in (1..=9).rev() {
        let first_digits = &digits[0..digits.len() - 1];

        if let Some(index) = find_index(first_digits, first) {
            let start = index + 1;
            let second_digits = &digits[start..];
            for second in (1..=9).rev() {
                if find_index(second_digits, second).is_some() {
                    return first as u64 * 10 + second as u64;
                }
            }
        }
    }

    0
}

pub fn run(input: &str) -> Result<String> {
    let banks = parse_lines(input)?;

    let sum = banks
        .into_iter()
        .map(|digits| get_max_jolt(&digits))
        .sum::<u64>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(run(input)?, "357");
        Ok(())
    }
}
