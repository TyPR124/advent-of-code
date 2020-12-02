#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashSet;
use std::iter::Enumerate;
use std::slice::Iter;
use std::str::FromStr;
//use std::hash::Hash;

mod input;
use self::input::INPUT;

fn main() {
    let overlap_count = part1(INPUT);
    println!("Part 1: # of overlapping points: {}", overlap_count);
    let id = part2(INPUT);
    println!("Part 2: Id of non-overlapping claim: {}", id);
}

fn part1(input: &str) -> usize {
    let mut overlaps = HashSet::new();
    let claims = input.parse::<Claims>().expect("Could not parse claims!");
    claims.enumerate().for_each(|(i, a)| {
        claims
            .iter()
            .skip(i + 1)
            .for_each(|b| overlaps.extend(overlapping_points(&a.rect, &b.rect)))
    });
    overlaps.len()
}

fn part2(input: &str) -> u64 {
    let claims = input.parse::<Claims>().expect("Could not parse claims!");
    let mut id = 0;
    #[derive(Debug)]
    struct OverlapError;
    #[derive(Debug)]
    struct DoneError;

    if claims
        .enumerate()
        .try_for_each(|(i, a)| {
            if claims
                .enumerate()
                .filter(|(j, _)| *j != i)
                .try_for_each(|(_, b)| {
                    if !overlapping_points(&a.rect, &b.rect).is_empty() {
                        Err(OverlapError)
                    } else {
                        Ok(())
                    }
                })
                .is_ok()
            {
                // This a never overlapped with a b
                id = a.id;
                Err(DoneError)
            } else {
                Ok(()) // continue try_for_each
            }
        })
        .is_err()
    {
        // DoneError
        id
    } else {
        panic!("Failed to find non-overlapping claim")
    }
}

#[allow(clippy::if_same_then_else)]
fn overlapping_points(a: &Rect, b: &Rect) -> Vec<Point> {
    if a == b {
        // Same rect
        a.all_points()
    } else if a.right() < b.left() || a.left() > b.right() {
        // No x overlap
        Vec::with_capacity(0)
    } else if a.bottom() < b.top() || a.top() > b.bottom() {
        // No y overlap
        Vec::with_capacity(0)
    } else {
        // Must be some overlap
        use std::cmp::{max, min};
        Rect::from_sides(
            max(a.left(), b.left()),
            min(a.right(), b.right()),
            max(a.top(), b.top()),
            min(a.bottom(), b.bottom()),
        )
        .all_points()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    pub x: usize,
    pub y: usize,
}
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Size {
    pub x: usize,
    pub y: usize,
}
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Rect {
    pub origin: Point, // Origin is top_left
    pub size: Size,
}

impl Rect {
    pub fn from_sides(left: usize, right: usize, top: usize, bottom: usize) -> Rect {
        Rect {
            origin: Point { x: left, y: top },
            size: Size {
                x: right - left + 1,
                y: bottom - top + 1,
            },
        }
    }
    pub fn left(&self) -> usize {
        self.origin.x
    }
    pub fn right(&self) -> usize {
        self.origin.x + self.size.x - 1
    }
    pub fn top(&self) -> usize {
        self.origin.y
    }
    pub fn bottom(&self) -> usize {
        self.origin.y + self.size.y - 1
    }
    pub fn all_points(&self) -> Vec<Point> {
        let mut vec = Vec::with_capacity(self.size.x * self.size.y);
        for x in self.left()..=self.right() {
            for y in self.top()..=self.bottom() {
                vec.push(Point { x, y });
            }
        }
        vec
    }
}

struct Claim {
    pub id: u64,
    pub rect: Rect,
}
struct Claims(Vec<Claim>);
impl Claims {
    pub fn iter(&self) -> Iter<Claim> {
        self.0.iter()
    }
    pub fn enumerate(&self) -> Enumerate<Iter<Claim>> {
        self.0.iter().enumerate()
    }
}

impl FromStr for Claims {
    type Err = Error;
    fn from_str(s: &str) -> Result<Claims, Self::Err> {
        let mut vec = Vec::<Claim>::new();
        for line in s.lines() {
            vec.push(line.parse()?);
        }
        Ok(Claims(vec))
    }
}

lazy_static! {
    static ref CLAIM_REGEX: Regex = Regex::new(r"^#(?P<id>[0-9]+) @ (?P<origin_x>[0-9]+),(?P<origin_y>[0-9]+): (?P<size_x>[0-9]+)x(?P<size_y>[0-9]+)$").unwrap();
}
impl FromStr for Claim {
    type Err = Error;
    fn from_str(s: &str) -> Result<Claim, Self::Err> {
        if let Some(caps) = CLAIM_REGEX.captures(s) {
            Ok(Claim {
                id: caps.name("id").unwrap().as_str().parse()?,
                rect: Rect {
                    origin: Point {
                        x: caps.name("origin_x").unwrap().as_str().parse()?,
                        y: caps.name("origin_y").unwrap().as_str().parse()?,
                    },
                    size: Size {
                        x: caps.name("size_x").unwrap().as_str().parse()?,
                        y: caps.name("size_y").unwrap().as_str().parse()?,
                    },
                },
            })
        } else {
            Err(Error::new("Could not find claim regex captures"))
        }
    }
}
#[derive(Debug, Clone)]
struct Error(String);
impl Error {
    fn new(s: &str) -> Error {
        Error(s.to_owned())
    }
}
impl<E: std::error::Error> From<E> for Error {
    fn from(e: E) -> Error {
        Error(format!("{}", e))
    }
}
