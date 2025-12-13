use anyhow::{Context, Result};

type Point = (u8, u8);

type Shape = Vec<Point>;

#[derive(Debug)]
struct Region {
    size: (u8, u8),
    shape_counts: Vec<u8>,
}

fn parse_input(input: &str) -> Result<(Vec<Shape>, Vec<Region>)> {
    let mut lines = input.lines().peekable();

    let mut shapes: Vec<Shape> = Vec::new();
    loop {
        lines.next().context("no line")?;
        let rows = [
            lines
                .next()
                .context("no first line")?
                .chars()
                .collect::<Vec<_>>(),
            lines
                .next()
                .context("no second line")?
                .chars()
                .collect::<Vec<_>>(),
            lines
                .next()
                .context("no third line")?
                .chars()
                .collect::<Vec<_>>(),
        ];

        let mut shape = Vec::new();
        for (y, row) in rows.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch == '#' {
                    shape.push((x as u8, y as u8));
                }
            }
        }

        shapes.push(normalize_shape(shape));

        lines.next().context("no line")?;

        let Some(line) = lines.peek() else {
            break;
        };
        if line.contains('x') {
            break;
        }
    }

    let mut regions: Vec<Region> = Vec::new();
    loop {
        let Some(line) = lines.next() else {
            break;
        };

        let (dimensions, counts) = line.split_once(':').context("expected colon")?;
        let (width, height) = dimensions.split_once('x').context("expected x")?;
        let width = width.parse::<u8>().context("expected number")?;
        let height = height.parse::<u8>().context("expected number")?;

        let shape_counts = counts
            .split_whitespace()
            .map(|p| p.parse::<u8>().context("expected number"))
            .collect::<Result<Vec<_>>>()?;

        regions.push(Region {
            size: (width, height),
            shape_counts,
        });
    }

    Ok((shapes, regions))
}

fn fits_in_area(region: &Region, shapes: &[Shape]) -> bool {
    let available = region.size.0 as u32 * region.size.1 as u32;

    let mut required = 0;
    for (shape_id, &count) in region.shape_counts.iter().enumerate() {
        required += shapes[shape_id].len() as u32 * count as u32;
    }

    required <= available
}

fn normalize_shape(mut shape: Shape) -> Shape {
    shape.sort();
    shape
}

fn rotate_right(shape: &Shape) -> Shape {
    let mut min_x = 0;
    let mut min_y = 0;

    let mut new_shape = Vec::new();
    for (x, y) in shape {
        let new_x = *y as i8;
        let new_y = -(*x as i8);

        new_shape.push((new_x, new_y));

        if new_x < min_x {
            min_x = new_x;
        }
        if new_y < min_y {
            min_y = new_y;
        }
    }

    let new_shape = new_shape
        .into_iter()
        .map(|(x, y)| ((x - min_x) as u8, (y - min_y) as u8))
        .collect::<Vec<_>>();

    normalize_shape(new_shape)
}

fn flip_horizontal(shape: &Shape) -> Shape {
    let mut min_x = 0;

    let mut new_shape = Vec::new();
    for (x, y) in shape {
        let new_x = -(*x as i8);
        let new_y = *y as i8;

        new_shape.push((new_x, new_y));

        if new_x < min_x {
            min_x = new_x;
        }
    }

    let new_shape = new_shape
        .into_iter()
        .map(|(x, y)| ((x - min_x) as u8, y as u8))
        .collect::<Vec<_>>();

    normalize_shape(new_shape)
}

fn get_orientations(shape: &Shape) -> Vec<Shape> {
    let r0 = shape.clone();
    let r1 = rotate_right(&r0);
    let r2 = rotate_right(&r1);
    let r3 = rotate_right(&r2);

    let h0 = flip_horizontal(shape);
    let h1 = rotate_right(&h0);
    let h2 = rotate_right(&h1);
    let h3 = rotate_right(&h2);

    let mut orientations = Vec::new();
    for s in [r0, r1, r2, r3, h0, h1, h2, h3] {
        if !orientations.contains(&s) {
            orientations.push(s);
        }
    }

    orientations
}

type Grid = Vec<Vec<bool>>;

fn create_grid(width: u8, height: u8) -> Grid {
    vec![vec![false; width as usize]; height as usize]
}

fn shapes_to_place(region: &Region, orientations: &[Vec<Shape>]) -> Vec<usize> {
    let mut shape_ids = Vec::new();
    for (shape_id, &count) in region.shape_counts.iter().enumerate() {
        for _ in 0..count {
            shape_ids.push(shape_id);
        }
    }
    shape_ids.sort_by_key(|&id| std::cmp::Reverse(orientations[id].len()));
    shape_ids
}

fn can_place(shape: &Shape, px: u8, py: u8, grid: &Grid) -> bool {
    let height = grid.len() as u8;
    let width = grid[0].len() as u8;

    for (x, y) in shape {
        let gx = px + *x;
        let gy = py + *y;
        if gx >= width || gy >= height {
            return false;
        }
        if grid[gy as usize][gx as usize] {
            return false;
        }
    }

    true
}

fn place(shape: &Shape, px: u8, py: u8, grid: &mut Grid) {
    for (x, y) in shape {
        let gx = px + *x;
        let gy = py + *y;
        grid[gy as usize][gx as usize] = true;
    }
}

fn unplace(shape: &Shape, px: u8, py: u8, grid: &mut Grid) {
    for (x, y) in shape {
        let gx = px + *x;
        let gy = py + *y;
        grid[gy as usize][gx as usize] = false;
    }
}

fn solve(shape_ids: &[usize], idx: usize, grid: &mut Grid, orientations: &[Vec<Shape>]) -> bool {
    if idx == shape_ids.len() {
        return true;
    }

    let height = grid.len() as u8;
    let width = grid[0].len() as u8;

    for shape in &orientations[shape_ids[idx]] {
        for y in 0..height {
            for x in 0..width {
                if can_place(shape, x, y, grid) {
                    place(shape, x, y, grid);
                    if solve(shape_ids, idx + 1, grid, orientations) {
                        return true;
                    }
                    unplace(shape, x, y, grid);
                }
            }
        }
    }

    false
}

fn build_orientations(shapes: &[Shape]) -> Vec<Vec<Shape>> {
    shapes.iter().map(get_orientations).collect()
}

pub fn run(input: &str) -> Result<String> {
    let (shapes, regions) = parse_input(input)?;

    let orientations = build_orientations(&shapes);

    let mut valid_regions = 0;
    for region in &regions {
        if !fits_in_area(region, &shapes) {
            continue;
        }

        let mut grid = create_grid(region.size.0, region.size.1);
        let shape_ids = shapes_to_place(region, &orientations);

        if solve(&shape_ids, 0, &mut grid, &orientations) {
            valid_regions += 1;
        }
    }

    Ok(valid_regions.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!(run(input)?, "2".to_string());

        Ok(())
    }
}
