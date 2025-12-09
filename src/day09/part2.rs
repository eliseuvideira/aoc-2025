use anyhow::{Result, bail};

use super::{Tile, get_tile_area, parse_tiles};

fn build_polygon_edges(tiles: &[Tile]) -> Result<(Vec<(u64, u64, u64)>, Vec<(u64, u64, u64)>)> {
    let mut vertical_edges = Vec::new();
    let mut horizontal_edges = Vec::new();

    for i in 0..tiles.len() {
        let corner1 = &tiles[i];
        let corner2 = &tiles[(i + 1) % tiles.len()];

        if corner1.x == corner2.x {
            vertical_edges.push((
                corner1.x,
                corner1.y.min(corner2.y),
                corner1.y.max(corner2.y),
            ));
            continue;
        }

        if corner1.y == corner2.y {
            horizontal_edges.push((
                corner1.y,
                corner1.x.min(corner2.x),
                corner1.x.max(corner2.x),
            ));
            continue;
        }

        bail!("invalid edge: {:?} -> {:?}", corner1, corner2);
    }

    Ok((vertical_edges, horizontal_edges))
}

fn rectangle_crosses_polygon_edge(
    corner1: &Tile,
    corner2: &Tile,
    vertical_edges: &[(u64, u64, u64)],
    horizontal_edges: &[(u64, u64, u64)],
) -> bool {
    let min_x = corner1.x.min(corner2.x);
    let max_x = corner1.x.max(corner2.x);
    let min_y = corner1.y.min(corner2.y);
    let max_y = corner1.y.max(corner2.y);

    for &(x, y1, y2) in vertical_edges {
        let col_inside = x > min_x && x < max_x;
        let rows_overlap = y1 < max_y && y2 > min_y;
        if col_inside && rows_overlap {
            return true;
        }
    }

    for &(y, x1, x2) in horizontal_edges {
        let row_inside = y > min_y && y < max_y;
        let cols_overlap = x1 < max_x && x2 > min_x;
        if row_inside && cols_overlap {
            return true;
        }
    }

    false
}

pub fn run(input: &str) -> Result<String> {
    let tiles = parse_tiles(input)?;
    let (vertical_edges, horizontal_edges) = build_polygon_edges(&tiles)?;

    let mut largest_area = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let corner1 = &tiles[i];
            let corner2 = &tiles[j];

            if rectangle_crosses_polygon_edge(corner1, corner2, &vertical_edges, &horizontal_edges)
            {
                continue;
            }

            let area = get_tile_area(corner1, corner2);
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
        assert_eq!(run(input)?, "24".to_string());

        Ok(())
    }
}
