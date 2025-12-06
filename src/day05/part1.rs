use anyhow::Result;

use super::parse_database;

pub fn run(input: &str) -> Result<String> {
    let (ranges, ingredients) = parse_database(input)?;

    let fresh_ingredients = ingredients
        .iter()
        .filter(|&&i| ranges.iter().any(|r| i >= r.min && i <= r.max))
        .count();

    Ok(fresh_ingredients.to_string())
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
        assert_eq!(run(input)?, "3");
        Ok(())
    }
}
