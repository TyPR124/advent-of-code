mod input;
use input::INPUT;

use aoc_2019::*;

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Color {
    Black,
    White,
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;
impl Direction {
    fn clockwise(self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn counter_clockwise(self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}
struct EmergencyHullPaintingRobot {
    cpu: Cpu,
    painted: HashMap<Point, Color>,
    location: Point,
    direction: Direction,
    tx: SyncSender<i64>,
    rx: Receiver<i64>,
}
impl EmergencyHullPaintingRobot {
    fn new(cpu: Cpu, tx: SyncSender<i64>, rx: Receiver<i64>) -> Self {
        Self {
            cpu,
            painted: HashMap::new(),
            location: Point { x: 0, y: 0 },
            direction: North,
            tx,
            rx,
        }
    }
    fn from_input(input: &str) -> Self {
        let (cpu, tx, rx) = Cpu::from_input(input);
        Self::new(cpu, tx, rx)
    }
    fn set_starting_panel_white(&mut self) {
        self.painted.insert(Point { x: 0, y: 0 }, Color::White);
    }
    fn run(mut self) -> HashMap<Point, Color> {
        self.cpu.run_parallel_and_drop();

        loop {
            // Send current color
            let color = self
                .painted
                .get(&self.location)
                .copied()
                .unwrap_or(Color::Black);
            let color = match color {
                Color::Black => 0,
                Color::White => 1,
            };
            match self.tx.send(color) {
                Ok(()) => {}
                Err(_) => break,
            }
            // Recv new color
            let new_color = match self.rx.recv() {
                Ok(0) => Color::Black,
                Ok(1) => Color::White,
                Err(_) => break,
                _ => unreachable!("Invalid color input"),
            };
            self.painted.insert(self.location, new_color);
            // Recv new direction
            match self.rx.recv().unwrap() {
                0 => self.direction = self.direction.counter_clockwise(),
                1 => self.direction = self.direction.clockwise(),
                _ => unreachable!("Invalid turn input"),
            }
            match self.direction {
                North => self.location.y -= 1,
                South => self.location.y += 1,
                West => self.location.x -= 1,
                East => self.location.x += 1,
            }
        } // loop
        self.painted
    }
}

fn main() {
    let robot = EmergencyHullPaintingRobot::from_input(INPUT);
    println!("1. {}", robot.run().len());
    let mut robot = EmergencyHullPaintingRobot::from_input(INPUT);
    robot.set_starting_panel_white();
    let painted = robot.run();
    let mut white: Vec<Point> = painted
        .into_iter()
        .filter_map(|(point, color)| {
            if color == Color::White {
                Some(point)
            } else {
                None
            }
        })
        .collect();
    let min_x = white.iter().map(|p| p.x).min().unwrap();
    let max_x = white.iter().map(|p| p.x).max().unwrap();
    let min_y = white.iter().map(|p| p.y).min().unwrap();
    let max_y = white.iter().map(|p| p.y).max().unwrap();

    println!("2.");
    println!();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some((i, _)) = white.iter().enumerate().find(|(_, p)| p.y == y && p.x == x) {
                print!("#");
                white.swap_remove(i);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
