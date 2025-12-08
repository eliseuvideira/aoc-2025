use anyhow::Result;

use super::{find_index, parse_lines};

fn get_max_jolt(digits: &[u8]) -> u64 {
    let mut max_jolt = 0;
    let mut next_digits = digits;
    for depth in (0..12).rev() {
        let current_digits = &next_digits[0..next_digits.len() - depth];

        for number in (1..=9).rev() {
            if let Some(index) = find_index(current_digits, number) {
                max_jolt = max_jolt * 10 + number as u64;
                next_digits = &next_digits[index + 1..];
                break;
            }
        }
    }

    max_jolt
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
        assert_eq!(run(input)?, "3121910778619");
        Ok(())
    }
}
