extern crate aoc_2018;
use aoc_2018::Result;

use std::collections::HashMap;

fn main() -> Result<()> {
    let mut grid = Grid::new(7989);
    //let mut grid = Grid::new(18);
    let size3_hisum = part1(&mut grid);
    println!("Part1: Point with highest sum: {}", size3_hisum);
    println!("Part2 will take a while...");
    let hisum = part2(&mut grid);
    println!("Part2: Square with highest sum: {}", hisum);
    Ok(())
}

fn part1(grid: &mut Grid) -> Point {
    // let mut hi_point = Point{x:0,y:0};
    // let mut hi_sum = std::isize::MIN;
    // for x in 1..=98 {
    //     for y in 1..=98 {
    //         let mut sum = 0;
    //         for px in x..x+3 {
    //             for py in y..y+3 {
    //                 sum += grid.query(Point{x:px, y:py});
    //             }
    //         }
    //         if sum > hi_sum {
    //             hi_point = Point{x,y};
    //             hi_sum = sum;
    //         }
    //     }
    // }

    Squares::new()
        .whole_size(300)
        .max_size(3)
        .start_size(3)
        .shift(1)
        .iter()
        .map(|s| (s.origin, grid.query_square(s)) )
        .max_by_key(|(_, result)| *result)
        .expect("Didn't get any squares in 3x3 query")
        .0




    //hi_point
}

fn part2(grid: &mut Grid) -> Square {
    Squares::new()
        .whole_size(300)
        .max_size(300)
        .start_size(1)
        .shift(1)
        .iter()
        .map(|s| {
            //println!("Checking {}", s);
            (s, grid.query_square(s))
        })
        .max_by_key(|(_, result)| *result)
        .expect("Didn't get any squares in full query")
        .0
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Square {
    pub origin: Point,
    pub size: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(out, "{},{}", self.x, self.y)
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(out, "{},{},{}", self.origin.x, self.origin.y, self.size)
    }
}

struct Grid {
    serial: usize,
    cells: HashMap<Point, isize>,
    squares: HashMap<Square, isize>,
}

impl Grid {
    pub fn new(serial: usize) -> Grid {
        Grid {
            serial,
            cells: HashMap::new(),
            squares: HashMap::new(),
        }
    }
    pub fn query(&mut self, p: Point) -> isize {
        let serial = self.serial;
        *self.cells.entry(p).or_insert(power_level(p,serial))
    }
    pub fn query_square(&mut self, s: Square) -> isize {
        if s.size == 0 {
            0
        } else if let Some(&answer) = self.squares.get(&s) {
            answer
        } else if let Some(&almost) = self.squares.get(&Square { origin: s.origin, size: s.size - 1}) {
            //println!("Using cached square");
            let mut finsum = 0;
            let y = s.origin.y+s.size-1;
            for x in s.origin.x..s.origin.x+s.size {
                finsum += self.query(Point{x,y});
            }
            let x = s.origin.x+s.size-1;
            for y in s.origin.y..s.origin.y+s.size-1 { 
                finsum += self.query(Point{x,y});
            }
            let answer = almost+finsum;
            self.squares.insert(s, answer);
            answer
        } else {
            let mut sum = 0;
            for x in s.origin.x..s.origin.x+s.size {
                for y in s.origin.y..s.origin.y+s.size {
                    sum += self.query(Point{x,y});
                }
            }
            self.squares.insert(s, sum);
            sum
        }
    }
}

fn power_level(p: Point, serial: usize) -> isize {
    let x = p.x as usize;
    let y = p.y as usize;
    let rack_id = x + 10;
    let tmp_power_lvl = ((rack_id * y) + serial) * rack_id;
    let string = format!("{}", tmp_power_lvl);
    let len = string.len();
    let power_level = if len >= 3 {
        (&string[len-3..len-2]).parse().unwrap()
    } else {
        0isize
    } - 5;
    power_level
}

#[derive(Copy, Clone, Default)]
struct Squares {
    whole_size: usize, // Size of entire grid
    max_size: usize, // Max size of output square
    cpoint: Point, // Current point
    csize: usize, // Current size
    shift: usize, // Don't think too hard about off-by-one
                  // (Shifts all output squares by a set amount in both x,y)
}

struct SquaresBuilder {
    inner: Squares
}

impl SquaresBuilder {
    pub fn whole_size(&mut self, size: usize) -> &mut Self {
        assert!(size > 0, "Cannot have square with size 0!");
        self.inner.whole_size = size;
        self
    }
    pub fn max_size(&mut self, size: usize) -> &mut Self {
        assert!(size > 0, "Cannot have square with size 0!");
        self.inner.max_size = size;
        self
    }
    pub fn start_size(&mut self, size: usize) -> &mut Self {
        assert!(size > 0, "Cannot have square with size 0!");
        self.inner.csize = size - 1; // Gets incremented once first loop
        self
    }
    pub fn shift(&mut self, shift: usize) -> &mut Self {
        self.inner.shift = shift;
        self
    }
    pub fn iter(&mut self) -> &mut Squares {
        let sq = &mut self.inner;
        sq.cpoint = Point { // Ensures first loop increments size
            x: sq.whole_size - sq.csize,
            y: sq.whole_size - sq.csize,
        };
        sq
    }
}

impl Squares {
    pub fn new() -> SquaresBuilder {
        SquaresBuilder {
            inner: Squares::default()
        }
    }
}

impl Iterator for Squares {
    type Item = Square;
    fn next(&mut self) -> Option<Square> {
        if self.csize == self.whole_size {
            None // Already returned the 1 possible result where csize == whole_size
        } else {
            if self.cpoint.x == self.whole_size - self.csize {
                if self.cpoint.y == self.whole_size - self.csize {
                    if self.csize == self.max_size {
                        return None;
                    }
                    // Last result was last of this size
                    self.csize += 1;
                    if self.csize % 30 == 0 {
                        println!("Checked sizes up to {}...", self.csize);
                    }
                    self.cpoint = Point{x:0,y:0};
                } else {
                    // Last result was last of this row
                    self.cpoint.y += 1;
                    self.cpoint.x = 0;
                }
            } else {
                self.cpoint.x += 1;
            } // Done incrementing

            Some(Square{
                origin: Point{
                    x: self.cpoint.x + self.shift,
                    y: self.cpoint.y + self.shift,
                },
                size: self.csize
            })
        }
    }
}