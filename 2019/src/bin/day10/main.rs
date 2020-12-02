mod input;
use input::INPUT;

use std::f64::consts::PI;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let mut points = Vec::new();
    let mut angles: Vec<Vec<f64>> = Vec::new();

    for (y, line) in INPUT.trim().lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            let point_count = points.len();
            if b == b'.' {
                continue;
            }
            points.push(Point { x, y });
            let mut point_angles = Vec::new();
            for i in 0..point_count {
                let angle = (y as f64 - points[i].y as f64).atan2(x as f64 - points[i].x as f64);
                let opp_angle = if angle > 0f64 { angle - PI } else { angle + PI };
                if !point_angles.contains(&angle) {
                    point_angles.push(angle)
                }
                if !angles[i].contains(&opp_angle) {
                    angles[i].push(opp_angle)
                }
            }
            angles.push(point_angles);
        }
    }
    let (station_idx, unique_angles) = angles
        .iter()
        .map(Vec::len)
        .enumerate()
        .max_by_key(|(_i, len)| *len)
        .unwrap();
    println!("1. {}", unique_angles);

    let station = points.swap_remove(station_idx);

    // println!("Station at {}, {}", station.x, station.y);

    let mut by_angle = Vec::<(f64, Vec<(f64, Point)>)>::new();

    'adding_points: for p in points {
        let angle = (station.y as f64 - p.y as f64).atan2(station.x as f64 - p.x as f64);
        let angle = if angle < PI / 2f64 {
            angle + 2f64 * PI
        } else {
            angle
        };
        assert!(angle >= 0f64);
        let dist = (station.y as f64 - p.y as f64).hypot(station.x as f64 - p.x as f64);
        for i in 0..by_angle.len() {
            #[allow(clippy::float_cmp)]
            if angle == by_angle[i].0 {
                for j in 0..by_angle[i].1.len() {
                    if dist > by_angle[i].1[j].0 {
                        by_angle[i].1.insert(j, (dist, p));
                        continue 'adding_points;
                    }
                }
                by_angle[i].1.push((dist, p));
                continue 'adding_points;
            } else if angle < by_angle[i].0 {
                by_angle.insert(i, (angle, vec![(dist, p)]));
                continue 'adding_points;
            }
        }
        by_angle.push((angle, vec![(dist, p)]));
    }

    let mut cycle_idx = 0;
    let mut last_point = Point { x: 0, y: 0 };
    for _i in 0..200 {
        let points = &mut by_angle[cycle_idx].1;
        let tuple = points.pop().unwrap();
        last_point = tuple.1;
        let new_len = points.len();
        // match _i {
        //     0 | 1 | 2 | 9 => println!("{} : {}, {} (angle: {}) (dist: {}) (rem: {})", _i+1, last_point.x, last_point.y, by_angle[cycle_idx].0.to_degrees(), tuple.0, new_len),
        //     _ => {}
        // }
        if new_len == 0 {
            by_angle.remove(cycle_idx);
        } else {
            cycle_idx += 1;
        }
        if cycle_idx == by_angle.len() {
            cycle_idx = 0
        };
    }

    println!("2. {}", last_point.x * 100 + last_point.y);
}
