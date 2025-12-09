use anyhow::Result;

use super::{get_tile_area, parse_tiles};

pub fn run(input: &str) -> Result<String> {
    let tiles = parse_tiles(input)?;

    let mut largest_area = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let area = get_tile_area(&tiles[i], &tiles[j]);

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    Ok(largest_area.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(run(input)?, "50".to_string());

        Ok(())
    }
}
