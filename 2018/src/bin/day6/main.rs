mod input;
use self::input::INPUT;

extern crate aoc_2018;
use aoc_2018::{Result, Error, err};

extern crate rayon;
use rayon::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub fn main() -> Result<()> {
    let mut points = Vec::with_capacity(INPUT.lines().count());
    for line in INPUT.trim().lines() {
        points.push(line.parse::<Point>()?)
    }
    part1(&points);


    Ok(())
}

fn distance(p1: &Point, p2: &Point) -> usize {
    fn abs(i: isize) -> isize {
        if i < 0 {
            i * -1
        } else {
            i
        }
    }

    let xd = abs(p1.x - p2.x) as usize;
    let yd = abs(p1.y - p2.y) as usize;
    xd + yd
}

fn part1(points: &Vec<Point>) -> Result<usize> {
    //let mut grid = HashMap::new();

    let mut bounds: Bounds = Default::default();
    // Get bounds of all points. Incrase size by 1 in each direction.
    // Later, anything touching the extremem edges can be considered DQ for being infinite
    bounds = points.iter().fold(bounds, |mut bounds, p| {
        if p.x <= bounds.min_x {
            bounds.min_x = p.x - 1;
        }
        if p.x >= bounds.max_x {
            bounds.max_x = p.x + 1;
        }
        if p.y <= bounds.min_y {
            bounds.min_y = p.y - 1;
        }
        if p.y >= bounds.max_y {
            bounds.max_y = p.y + 1;
        }
        bounds
    });

    

    Ok(0)
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    pub x: isize,
    pub y: isize,
}
struct Owner {
    pub id: usize,
    pub dist: usize,
}
enum State {
    Owned(Owner),
    Tied,
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Point> {
        let mut parts = s.split(", ");
        for p in parts.clone() {
            println!("Part: {}", p);
        }
        if parts.clone().count() != 2 {
            Err(err!("Expected 2 parts in Point::from_str()"))
        } else {
            Ok(Point {
                x: parts.next().unwrap().parse()?,
                y: parts.next().unwrap().parse()?
            })
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Size {
    pub x: usize,
    pub y: usize,
}
#[derive(PartialEq, Eq, Clone, Copy)]
struct Rect {
    pub origin: Point, // Origin is top_left
    pub size: Size,
}

impl Rect {
    pub fn from_sides(left: isize, right: isize, top: isize, bottom: isize) -> Rect {
        Rect {
            origin: Point { x: left, y: top },
            size: Size { x: (right-left+1) as usize, y: (bottom-top+1) as usize }
        }
    }
    pub fn left(&self) -> isize {
        self.origin.x
    }
    pub fn right(&self) -> isize {
        self.origin.x + self.size.x as isize - 1
    }
    pub fn top(&self) -> isize {
        self.origin.y
    }
    pub fn bottom(&self) -> isize {
        self.origin.y + self.size.y as isize - 1
    }
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.left() && p.x <= self.right() &&
        p.y >= self.top() && p.y <= self.bottom()
    }
}

#[derive(Copy, Clone)]
struct Bounds {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize
}
impl Bounds {
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.min_x && p.x <= self.max_x &&
        p.y >= self.min_y && p.y <= self.max_y
    }
    pub fn points(&self) -> PointGenerator {
        PointGenerator::new(*self)
    }
}

impl Default for Bounds {
    fn default() -> Bounds {
        Bounds {
            min_x: std::isize::MAX,
            min_y: std::isize::MAX,
            max_x: std::isize::MIN,
            max_y: std::isize::MIN,
        }
    }
}

struct PointGenerator {
    bounds: Bounds,
    x: isize,
    y: isize,
    out: bool,
}

impl PointGenerator {
    fn new(bounds: Bounds) -> PointGenerator {
        PointGenerator {
            bounds,
            x: bounds.min_x,
            y: bounds.min_y,
            out: false
        }
    }
}

impl Iterator for PointGenerator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if self.out {
            None
        } else {
            let p = Point{x: self.x, y: self.y};
            if self.x == self.bounds.max_x {
                if self.y == self.bounds.max_y {
                    self.out = true;
                } else {
                    self.x = self.bounds.min_x;
                    self.y += 1;
                }
            } else {
                self.x += 1;
            }
            Some(p)
        }
    }
}