use anyhow::{Result, bail};

use super::{Tile, get_tile_area, parse_tiles};

fn build_edges(tiles: &[Tile]) -> Result<(Vec<(u64, u64, u64)>, Vec<(u64, u64, u64)>)> {
    let mut vertical = Vec::new();
    let mut horizontal = Vec::new();

    for i in 0..tiles.len() {
        let t1 = &tiles[i];
        let t2 = &tiles[(i + 1) % tiles.len()];

        if t1.x == t2.x {
            vertical.push((t1.x, t1.y.min(t2.y), t1.y.max(t2.y)));
            continue;
        }

        if t1.y == t2.y {
            horizontal.push((t1.y, t1.x.min(t2.x), t1.x.max(t2.x)));
            continue;
        }

        bail!("invalid edge: {:?} -> {:?}", t1, t2);
    }

    Ok((vertical, horizontal))
}

fn has_edge_crossing(
    t1: &Tile,
    t2: &Tile,
    vertical: &[(u64, u64, u64)],
    horizontal: &[(u64, u64, u64)],
) -> bool {
    let min_x = t1.x.min(t2.x);
    let max_x = t1.x.max(t2.x);
    let min_y = t1.y.min(t2.y);
    let max_y = t1.y.max(t2.y);

    for &(x, y1, y2) in vertical {
        if x > min_x && x < max_x && y1 < max_y && y2 > min_y {
            return true;
        }
    }

    for &(y, x1, x2) in horizontal {
        if y > min_y && y < max_y && x1 < max_x && x2 > min_x {
            return true;
        }
    }

    false
}

pub fn run(input: &str) -> Result<String> {
    let tiles = parse_tiles(input)?;
    let (vertical, horizontal) = build_edges(&tiles)?;

    let mut largest_area = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            if !has_edge_crossing(&tiles[i], &tiles[j], &vertical, &horizontal) {
                let area = get_tile_area(&tiles[i], &tiles[j]);
                if area > largest_area {
                    largest_area = area;
                }
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
        assert_eq!(run(input)?, "24".to_string());

        Ok(())
    }
}
