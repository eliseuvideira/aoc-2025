use anyhow::Result;

pub fn run(_input: &str) -> Result<String> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "";
        assert_eq!(run(input)?, "".to_string());

        Ok(())
    }
}
