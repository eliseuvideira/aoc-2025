use anyhow::Result;

pub mod part1;
pub mod part2;

#[cfg(test)]
mod bench;

pub fn parse_lines(input: &str) -> Result<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
            Ok(digits)
        })
        .collect::<Result<Vec<_>>>()
}

pub fn find_index(digits: &[u8], number: u8) -> Option<usize> {
    digits.iter().position(|&d| d == number)
}
