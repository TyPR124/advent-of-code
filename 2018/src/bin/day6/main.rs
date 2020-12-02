mod input;
use self::input::INPUT;

extern crate aoc_2018;
use aoc_2018::{err, Error, Result};

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
    let mut bounds: Bounds = Default::default();
    // Get bounds of all points. Incrase size by 1 in each direction.
    // Later, anything touching the extremem edges can be considered DQ for being infinite
    points.iter().for_each(|p| {
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
    });
    let max_area = part1(&points, bounds)?;
    println!("Part1: Max Area: {}", max_area);
    let close_area = part2(&points, bounds)?;
    println!("Part2: Close Area: {}", close_area);
    Ok(())
}

fn distance(p1: &Point, p2: &Point) -> usize {
    fn abs(i: isize) -> isize {
        if i < 0 {
            -i
        } else {
            i
        }
    }

    let xd = abs(p1.x - p2.x) as usize;
    let yd = abs(p1.y - p2.y) as usize;
    xd + yd
}

fn part1(points: &[Point], bounds: Bounds) -> Result<usize> {
    use std::sync::Mutex;
    let results = Mutex::new(HashMap::<usize, usize>::new());
    let disq = Mutex::new(HashSet::<usize>::new());

    bounds.all_points().par_iter().for_each(|r| {
        // r becasue it's a (not-so-)random point in the bounds
        let mut min = std::usize::MAX;
        let mut owners = 0;
        let mut winner = 0;
        points.iter().enumerate().for_each(|(i, o)| {
            // o because it's a potential owner
            let r_o = distance(&r, &o);
            #[allow(clippy::clippy::comparison_chain)]
            if r_o < min {
                min = r_o;
                winner = i;
                owners = 1;
            } else if r_o == min {
                owners += 1;
            }
        });
        if owners == 1 {
            if bounds.is_on_perimeter(r) {
                // It is perimeter, therefore winner is disqualified
                let mut dqguard = disq.lock().expect("Failed to wait for disq lock");
                dqguard.insert(winner);
                drop(dqguard);
            } else {
                let mut mapguard = results.lock().expect("Failed to wait for results lock");
                use std::ops::AddAssign;
                mapguard.entry(winner).or_insert(0).add_assign(1);
                drop(mapguard);
            }
        }
    });

    let results = results.into_inner()?;
    let disq = disq.into_inner()?;

    Ok(*results
        .iter()
        .filter(|kv| !disq.contains(kv.0))
        .map(|kv| kv.1)
        .max()?)
}
fn part2(points: &[Point], bounds: Bounds) -> Result<usize> {
    Ok(bounds
        .all_points()
        .par_iter()
        .map(|r| {
            let mut r_total = 0;
            points.iter().for_each(|p| r_total += distance(&p, &r));
            r_total
        })
        .filter(|t| *t < 10000)
        .count())
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    pub x: isize,
    pub y: isize,
}
impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Point> {
        let mut parts = s.split(", ");
        if parts.clone().count() != 2 {
            Err(err!("Expected 2 parts in Point::from_str()"))
        } else {
            Ok(Point {
                x: parts.next()?.parse()?,
                y: parts.next()?.parse()?,
            })
        }
    }
}
#[derive(Copy, Clone)]
struct Bounds {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize,
}
impl Bounds {
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }
    pub fn all_points(&self) -> Vec<Point> {
        //PointGenerator::new(*self)
        let mut vec = Vec::with_capacity(self.area());
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                vec.push(Point { x, y });
            }
        }
        vec
    }
    pub fn is_on_perimeter(&self, p: &Point) -> bool {
        self.contains(p) && (p.x == self.min_x || p.x == self.max_x)
            || (p.y == self.min_y || p.y == self.max_y)
    }
    pub fn area(&self) -> usize {
        (self.max_x - self.min_x + 1) as usize * (self.max_y - self.min_y + 1) as usize
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
