mod input;
use input::INPUT;

use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Hash)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}
impl Vec3 {
    fn abs_sum(self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Hash)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn potential_energy(&self) -> usize {
        self.pos.abs_sum()
    }
    fn kinetic_energy(&self) -> usize {
        self.vel.abs_sum()
    }
    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }
    fn apply_gravity(&mut self, other: Self) {
        self.vel.x += match self.pos.x.cmp(&other.pos.x) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0
        };
        self.vel.y += match self.pos.y.cmp(&other.pos.y) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0
        };
        self.vel.z += match self.pos.z.cmp(&other.pos.z) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0
        };
    }
    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
}

fn total_energy(moons: &[Moon]) -> usize {
    moons.iter().map(Moon::total_energy).sum()
}

fn step(moons: &mut [Moon]) {
    for this in 0..moons.len() {
        for other in this+1..moons.len() {
            if this == other { continue }
            moons[this].apply_gravity(moons[other]);
            moons[other].apply_gravity(moons[this]);
        }
        moons[this].apply_velocity();
    }
}

fn parse_moons(input: &str) -> Vec<Moon> {
    input.trim().lines().map(|line| {
        let x_s = line.find("=").unwrap() + 1;
        let x_e = line.find(",").unwrap();
        let x = isize::from_str(&line[x_s..x_e]).unwrap();
        let line = &line[x_e+1..];

        let y_s = line.find("=").unwrap() + 1;
        let y_e = line.find(",").unwrap();
        let y = isize::from_str(&line[y_s..y_e]).unwrap();
        let line = &line[y_e+1..];

        let z_s = line.find("=").unwrap() + 1;
        let z_e = line.find(">").unwrap();
        let z = isize::from_str(&line[z_s..z_e]).unwrap();
        
        Moon {
            pos: Vec3 { x, y, z },
            vel: Vec3::default(),
        }
    }).collect()
}

fn main() {
    let mut moons = parse_moons(INPUT);

    for _ in 0..1000 {
        step(&mut moons);
    }

    println!("1. {}", total_energy(&moons));

    let mut moons = parse_moons(INPUT);
    let initial = moons.clone();
    let mut steps = 0;
    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;

    loop {
        step(&mut moons);
        steps += 1;
        if x_period.is_none() && compare_x(&moons, &initial) {
            // println!("Found x period: {}", steps);
            x_period = Some(steps);
        }
        if y_period.is_none() && compare_y(&moons, &initial) {
            // println!("Found y period: {}", steps);
            y_period = Some(steps);
        }
        if z_period.is_none() && compare_z(&moons, &initial) {
            // println!("Found z period: {}", steps);
            z_period = Some(steps);
        }
        if let (Some(xp), Some(yp), Some(zp)) = (x_period, y_period, z_period) {
            println!("2. {}", lcm(xp, yp, zp));
            break
        }
    }
}

fn compare_x(moons: &[Moon], initial: &[Moon]) -> bool {
    moons.iter().map(|moon| (moon.pos.x, moon.vel.x))
        .zip(initial.iter().map(|moon| (moon.pos.x, moon.vel.x)))
        .find(|(m1, m2)| m1 != m2)
        .is_none()
}
fn compare_y(moons: &[Moon], initial: &[Moon]) -> bool {
    moons.iter().map(|moon| (moon.pos.y, moon.vel.y))
        .zip(initial.iter().map(|moon| (moon.pos.y, moon.vel.y)))
        .find(|(m1, m2)| m1 != m2)
        .is_none()
}
fn compare_z(moons: &[Moon], initial: &[Moon]) -> bool {
    moons.iter().map(|moon| (moon.pos.z, moon.vel.z))
        .zip(initial.iter().map(|moon| (moon.pos.z, moon.vel.z)))
        .find(|(m1, m2)| m1 != m2)
        .is_none()
}


fn lcm(mut x: usize, mut y: usize, mut z: usize) -> usize {
    let mut lcm = 1;

    for n in 2.. {
        let count_x = factor_count(&mut x, n);
        let count_y = factor_count(&mut y, n);
        let count_z = factor_count(&mut z, n);
        if count_x > 0 || count_y > 0 || count_z > 0 {
            let count = count_x.max(count_y).max(count_z);
            lcm *= n.pow(count as u32);
        }
        if x == 1 && y == 1 && z == 1 { break }
    }

    lcm
}

fn factor_count(x: &mut usize, n: usize) -> usize {
    let mut count = 0;
    while *x % n == 0 {
        count += 1;
        *x /= n;
    }
    count
}