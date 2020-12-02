mod input;
use self::input::INPUT;

extern crate aoc_2018;
use aoc_2018::Result;

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

pub fn main() -> Result<()> {
    let (text, ticks) = part1(INPUT)?;
    println!("Part1: Are stars aligned?\n\n{}\n", text);
    println!("Part2: It took {} seconds.", ticks);
    Ok(())
}

lazy_static! { // position=< 31129, -41131> velocity=<-3,  4>
    static ref LINE_REGEX: Regex = Regex::new("^position=<(?P<pos_x>[0-9- ]+),(?P<pos_y>[0-9- ]+)> velocity=<(?P<vel_x>[0-9- ]+),(?P<vel_y>[0-9- ]+)>$").unwrap();
}

fn part1(input: &str) -> Result<(String, usize)> {
    let input = input.trim();
    let mut points = Vec::new();
    let mut vel = Vec::new();

    for line in input.lines() {
        let caps = LINE_REGEX.captures(line.trim()).expect("No line captures!");
        let p = Point {
            x: caps
                .name("pos_x")
                .expect("No pos_x")
                .as_str()
                .trim()
                .parse()?,
            y: caps
                .name("pos_y")
                .expect("No pos_y")
                .as_str()
                .trim()
                .parse()?,
        };
        let v = Point {
            x: caps
                .name("vel_x")
                .expect("No vel_x")
                .as_str()
                .trim()
                .parse()?,
            y: caps
                .name("vel_y")
                .expect("No vel_y")
                .as_str()
                .trim()
                .parse()?,
        };
        points.push(p);
        vel.push(v);
    }
    let mut display = PointDisplay {
        points,
        vel,
        ticks: 0,
    };
    display.to_convergence();
    Ok((display.text(), display.ticks))
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    pub x: isize,
    pub y: isize,
}
#[derive(Copy, Clone, PartialEq)]
struct PointF {
    pub x: f64,
    pub y: f64,
}
#[derive(Copy, Clone, Eq, PartialEq)]
struct Rect {
    pub min: Point,
    pub max: Point,
}
impl Rect {
    pub fn from_points(points: &[Point]) -> Rect {
        let mut min = Point {
            x: std::isize::MAX,
            y: std::isize::MAX,
        };
        let mut max = Point {
            x: std::isize::MIN,
            y: std::isize::MIN,
        };

        points.iter().for_each(|p| {
            if p.x < min.x {
                min.x = p.x;
            }
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            }
            if p.y > max.y {
                max.y = p.y;
            }
        });
        Rect { min, max }
    }
    pub fn area(&self) -> usize {
        ((self.max.x - self.min.x) * (self.max.y - self.min.y)) as usize
    }
}
struct PointDisplay {
    points: Vec<Point>,
    vel: Vec<Point>,
    pub ticks: usize,
}

impl PointDisplay {
    pub fn step_forward(&mut self) {
        let vel = &self.vel;
        self.points.iter_mut().enumerate().for_each(|(i, p)| {
            p.x += vel[i].x;
            p.y += vel[i].y;
        });
        self.ticks += 1;
    }
    pub fn step_backward(&mut self) {
        let vel = &self.vel;
        self.points.iter_mut().enumerate().for_each(|(i, p)| {
            p.x -= vel[i].x;
            p.y -= vel[i].y;
        });
        self.ticks -= 1;
    }
    #[allow(clippy::clippy::wrong_self_convention)]
    pub fn to_convergence(&mut self) {
        let mut convergence = self.convergence_value();
        self.step_forward();
        let mut new = self.convergence_value();
        if new == convergence {
            return;
        }
        if new > convergence {
            // If getting farther apart
            while new > convergence {
                // Go until not getting farther apart
                convergence = new;
                self.step_forward();
                new = self.convergence_value();
            }
        }
        if new < convergence {
            // Then, If getting closer
            while new < convergence {
                // Keep going until not getting closer
                convergence = new;
                self.step_forward();
                new = self.convergence_value();
            }
        }
        // And then go back one
        self.step_backward();
    }

    pub fn convergence_value(&self) -> usize {
        Rect::from_points(&self.points).area()
    }

    fn text(&self) -> String {
        let mut out = String::new();
        let rect = Rect::from_points(&self.points);

        for y in rect.min.y..=rect.max.y {
            for x in rect.min.x..=rect.max.x {
                if self.points.contains(&Point { x, y }) {
                    out.push('#');
                } else {
                    out.push('.');
                }
            }
            if y != rect.max.y {
                out.push('\n');
            }
        }

        out
    }
}
