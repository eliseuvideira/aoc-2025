use anyhow::Result;

use super::{digit_count, parse_ranges};

fn is_repeating(number: usize) -> bool {
    let digits = digit_count(number);

    if digits % 2 != 0 {
        return false;
    }

    let divisor = 10_usize.pow((digits / 2) as u32);
    number / divisor == number % divisor
}

pub fn run(input: &str) -> Result<String> {
    let ranges = parse_ranges(input)?;

    let sum: usize = ranges
        .into_iter()
        .flat_map(|r| r)
        .filter(|&n| is_repeating(n))
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(run(input)?, "1227775554");
        Ok(())
    }
}
