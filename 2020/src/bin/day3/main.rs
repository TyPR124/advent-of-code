use anyhow::{bail, Result, ensure};


fn main() -> Result<()> {
    let grid = parse_grid(include_str!("input.txt"))?;

    let product_of_tree_counts = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|&(x, y)| {
        let slope = Point { x, y };
        let mut tree_count = 0;
        follow_slope(&grid, Point { x: 0, y: 0 }, slope, |occupant| {
            if matches!(occupant, Occupant::Tree) { tree_count += 1 }
        });
        (slope, tree_count)
    }).fold(1, |product, (slope, tree_count)| {
        if slope == { Point { x: 3, y: 1 } } {
            println!("1: {}", tree_count);
        }
        product * tree_count
    });
    println!("2: {}", product_of_tree_counts);

    Ok(())
}

struct Grid {
    rows: Vec<Vec<Occupant>>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

fn follow_slope(grid: &Grid, mut position: Point<isize>, mut slope: Point<isize>, mut callback: impl FnMut(&Occupant)) {
    loop {
        slope.x %= grid.width as isize;
        position.x += slope.x;
        position.y += slope.y;

        if position.y < 0 || position.y as usize >= grid.height { break }
        if position.x < 0 {
            position.x += grid.width as isize
        } else if position.x as usize >= grid.width {
            position.x -= grid.width as isize
        }
        let occupant = &grid.rows[position.y as usize][position.x as usize];
        callback(occupant)
    }
}

fn parse_grid(input: &str) -> Result<Grid> {
    let mut rows = vec![];
    let mut width = 0;
    for line in input.trim().lines() {
        let mut row = vec![];
        for ch in line.chars() {
            let occupant = match ch {
                '.' => Occupant::None,
                '#' => Occupant::Tree,
                _ => bail!("Invalid grid char '{}'", ch),
            };
            row.push(occupant);
        }
        let row_len = row.len();
        rows.push(row);
        width = rows[0].len();
        ensure!(row_len == width, "Mismatched grid row lengths; {} != {}", row_len, width);
    }
    let height = rows.len();
    ensure!(width != 0 || height != 0, "Empty grid is invalid");
    let grid = Grid { rows, width, height };
    Ok(grid)
}

enum Occupant {
    Tree,
    None
}