mod input;
use input::INPUT;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct TPoint<T> {
    x: T,
    y: T
}

type Point = TPoint<i64>;
impl<T> TPoint<T> {
    fn new(x: T, y: T) -> Self { Self { x, y }}
}

use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let mut set1 = HashMap::new();
    let mut position = Point::new(0, 0);
    let mut steps: usize = 0;
    INPUT.lines().nth(0).unwrap().split(",").for_each(|m| {
        let dist = i64::from_str(&m[1..]).unwrap();
        match &m[0..1] {
            "L" => {
                for x in (position.x - dist) .. position.x {
                    steps += 1;
                    set1.entry(Point::new(x, position.y)).or_insert(steps);
                }
                position.x -= dist;
            }
            "R" => {
                for x in (position.x + 1) ..= (position.x + dist) {
                    steps += 1;
                    set1.entry(Point::new(x, position.y)).or_insert(steps);
                }
                position.x += dist;
            }
            "U" => {
                for y in (position.y - dist) .. position.y {
                    steps += 1;
                    set1.entry(Point::new(position.x, y)).or_insert(steps);
                }
                position.y -= dist;
            }
            "D" => {
                for y in (position.y + 1) ..= (position.y + dist) {
                    steps += 1;
                    set1.entry(Point::new(position.x, y)).or_insert(steps);
                }
                position.y += dist;
            }
            _ => unreachable!()
        }
    });
    position = Point::new(0, 0);
    let mut best_dist = std::i64::MAX;
    let mut best_steps = std::usize::MAX;
    steps = 0;
    INPUT.lines().nth(1).unwrap().split(",").for_each(|m| {
        let dist = i64::from_str(&m[1..]).unwrap();
        match &m[0..1] {
            "L" => {
                for x in (position.x - dist) .. position.x {
                    steps += 1;
                    if let Some(steps1) = set1.get(&Point::new(x, position.y)) {
                        let dist_from_origin = x.abs() + position.y.abs();
                        if dist_from_origin < best_dist { best_dist = dist_from_origin }
                        let combined_steps = steps + steps1;
                        if combined_steps < best_steps { best_steps = combined_steps }
                    }
                }
                position.x -= dist;
            }
            "R" => {
                for x in (position.x + 1) ..= (position.x + dist) {
                    steps += 1;
                    if let Some(steps1) = set1.get(&Point::new(x, position.y)) {
                        let dist_from_origin = x.abs() + position.y.abs();
                        if dist_from_origin < best_dist { best_dist = dist_from_origin }
                        let combined_steps = steps + steps1;
                        if combined_steps < best_steps { best_steps = combined_steps }
                    }
                }
                position.x += dist;
            }
            "U" => {
                for y in (position.y - dist) .. position.y {
                    steps += 1;
                    if let Some(steps1) = set1.get(&Point::new(position.x, y)) {
                        let dist_from_origin = y.abs() + position.x.abs();
                        if dist_from_origin < best_dist { best_dist = dist_from_origin }
                        let combined_steps = steps + steps1;
                        if combined_steps < best_steps { best_steps = combined_steps }
                    }
                }
                position.y -= dist;
            }
            "D" => {
                for y in (position.y + 1) ..= (position.y + dist) {
                    steps += 1;
                    if let Some(steps1) = set1.get(&Point::new(position.x, y)) {
                        let dist_from_origin = y.abs() + position.x.abs();
                        if dist_from_origin < best_dist { best_dist = dist_from_origin }
                        let combined_steps = steps + steps1;
                        if combined_steps < best_steps { best_steps = combined_steps }
                    }
                }
                position.y += dist;
            }
            _ => unreachable!()
        }
    });

    println!("1. {}", best_dist);
    println!("2. {}", best_steps);
}