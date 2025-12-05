use anyhow::Result;

use crate::day05::{Range, parse_database};

pub fn run(input: &str) -> Result<String> {
    let (mut ranges, _) = parse_database(input)?;

    ranges.sort_by_key(|r| r.min);

    let mut merged: Vec<Range> = vec![];
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range.min <= last.max + 1 {
                last.max = last.max.max(range.max);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    let total: u64 = merged.into_iter().map(|r| r.max - r.min + 1).sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(run(input)?, "14");
        Ok(())
    }
}
