mod input;
use self::input::INPUT;

use std::cmp::{Ord, Ordering, PartialOrd};

use aoc_2018::{err, Error, Result};

type Carts = std::collections::BTreeMap<Point, Cart>;
type Turns = std::collections::HashMap<Point, Turn>;
//                      old    new    new
type CartUpdates = Vec<(Point, Point, Cart)>;

#[derive(Copy, Clone)]
struct Cart {
    dir: Direction,
    last_intersection: Decided,
}
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Point {
    pub x: usize,
    pub y: usize,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Turn {
    TopLeft,
    TopRight,
    FourWay,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use self::Direction::*;
use self::Turn::*;

fn main() -> Result<()> {
    let input = INPUT;
    let mut turns = Turns::new();
    let mut carts = Carts::new();
    for (y, line) in input.lines().enumerate() {
        for (x, &v) in line.as_bytes().iter().enumerate() {
            match v {
                b' ' | b'-' | b'|' => {} // Nothing
                b'\\' => {
                    turns.insert(Point { x, y }, TopRight);
                }
                b'/' => {
                    turns.insert(Point { x, y }, TopLeft);
                }
                b'+' => {
                    turns.insert(Point { x, y }, FourWay);
                }
                b'^' => {
                    carts.insert(
                        Point { x, y },
                        Cart {
                            dir: Up,
                            last_intersection: Decided::default(),
                        },
                    );
                }
                b'v' => {
                    carts.insert(
                        Point { x, y },
                        Cart {
                            dir: Down,
                            last_intersection: Decided::default(),
                        },
                    );
                }
                b'<' => {
                    carts.insert(
                        Point { x, y },
                        Cart {
                            dir: Left,
                            last_intersection: Decided::default(),
                        },
                    );
                }
                b'>' => {
                    carts.insert(
                        Point { x, y },
                        Cart {
                            dir: Right,
                            last_intersection: Decided::default(),
                        },
                    );
                }
                _ => {
                    return Err(err!(
                        "Unexpected character in input: '{}' at {},{}",
                        v,
                        x,
                        y
                    ))
                }
            }
        }
    }
    println!("Found {} carts and {} turns", carts.len(), turns.len());
    //panic!("Stop here");
    let mut collisions = Vec::new();
    let mut tmp = CartUpdates::new();
    let mut first = Point::default();
    let mut found_first = false;
    while carts.len() > 1 {
        tick(&mut carts, &turns, &mut collisions, &mut tmp);
        if !found_first && !collisions.is_empty() {
            first = *collisions.iter().min()?;
            found_first = true;
        }
    }

    println!("Part1: {}", first);
    println!("Part2: {}", carts.iter().next()?.0);

    Ok(())
}

fn tick(carts: &mut Carts, turns: &Turns, collisions: &mut Vec<Point>, tmp: &mut CartUpdates) {
    tmp.clear();
    // rename tmp to something more meaningful inside this fn
    let updates = tmp;
    collisions.clear();
    carts.iter().enumerate().for_each(|(i, (point, cart))| {
        if !collisions.contains(point) {
            // Anything already collided with, just skip
            let mut new_point = *point;
            let mut new_cart = *cart;
            match cart.dir {
                Up => new_point.y -= 1,
                Down => new_point.y += 1,
                Left => new_point.x -= 1,
                Right => new_point.x += 1,
            }
            // Any collisions?
            // Check future carts first, then already-updated carts
            let mut collided = false;
            carts
                .iter()
                .skip(i + 1)
                .filter(|(p2, _)| **p2 == new_point)
                .take(1)
                .for_each(|_| {
                    collisions.push(new_point);
                    collided = true;
                });
            if !collided {
                updates
                    .iter()
                    .filter(|(_, p2, _)| p2 == &new_point)
                    .take(1)
                    .for_each(|_| collisions.push(new_point));
            }

            // Update direction
            if let Some(&turn) = turns.get(&new_point) {
                new_cart.turn(turn);
            }

            updates.push((*point, new_point, new_cart));
        } else {
            // Was already collided with, don't move and wait for cleanup
            updates.push((*point, *point, *cart));
        }
    });

    // Update Carts
    for (oldp, newp, cart) in updates.iter() {
        carts.remove(oldp);
        if !collisions.contains(newp) {
            carts.insert(*newp, *cart);
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let y = self.y.cmp(&other.y);
        match y {
            Ordering::Equal => self.x.cmp(&other.x),
            _ => y,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(out, "{},{}", self.x, self.y)
    }
}

impl Direction {
    fn clockwise(self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    fn counter_clockwise(self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
enum Decided {
    Left,
    Straight,
    Right,
}
impl Default for Decided {
    fn default() -> Self {
        Decided::Right // Default to last, that way on first iteration, choose first
    }
}

impl Cart {
    fn turn(&mut self, turn: Turn) {
        self.dir = match (turn, self.dir) {
            //    /
            (TopLeft, Up) => Right,
            (TopLeft, Down) => Left,
            (TopLeft, Left) => Down,
            (TopLeft, Right) => Up,
            //    \
            (TopRight, Up) => Left,
            (TopRight, Down) => Right,
            (TopRight, Left) => Up,
            (TopRight, Right) => Down,
            //    +
            (FourWay, _) => match self.last_intersection {
                Decided::Left => {
                    self.last_intersection = Decided::Straight;
                    self.dir
                }
                Decided::Straight => {
                    self.last_intersection = Decided::Right;
                    self.dir.clockwise()
                }
                Decided::Right => {
                    self.last_intersection = Decided::Left;
                    self.dir.counter_clockwise()
                }
            },
        }
    }
}
