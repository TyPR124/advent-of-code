use std::collections::HashMap;

mod input;
use self::input::INPUT;

fn main() {
    let checksum = part1(INPUT);
    println!("Part 1: Checksum: {}", checksum);
    let common = part2(INPUT);
    println!("Part 2: Common chars: {}", common);
}

fn part2(input: &str) -> String {
    // use std::iter::Iterator;
    for (i, a) in input.lines().enumerate() {
        for b in input.lines().skip(i+1) {
            let (differ, pos) = differs_by_one(a, b);
            if differ {
                let bytes = a.bytes()
                .enumerate()
                .filter_map(|(i, b)|
                if i != pos {
                    Some(b)
                } else { None });
                return String::from_utf8(bytes.collect()).expect("Failed to build final string");
            }
        }
    }

    "blah".to_owned()
}

fn differs_by_one(a: &str, b: &str) -> (bool, usize) {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut pos = 0;
    let mut differ = false;
    if a.len() != b.len() {
        return (false, 0);
    } else {
        for i in 0..a.len() {
            if a[i] != b[i] {
                if differ {
                    return (false, 0);
                } else {
                    differ = true;
                    pos = i;
                }
            }
        }
    }

    (differ, pos)
}

fn part1(input: &str) -> usize {
    let mut two_count = 0;
    let mut three_count = 0;

    for line in input.lines() {
        let (two, three) = line_has_two_or_three(line);
        if two { two_count += 1; }
        if three {three_count += 1; }
    }

    two_count * three_count
}

fn line_has_two_or_three(line: &str) -> (bool, bool) {
    let mut chars = HashMap::new();
    for c in line.chars() {
        if let Some(count) = chars.get_mut(&c) {
            *count += 1;
        } else {
            chars.insert(c, 1usize);
        }
    }
    let mut two = false;
    let mut three = false;
    for v in chars.values() {
        match *v {
            2 => {two = true; if three {break;}},
            3 => {three = true; if two {break;}},
            _ => continue,
        }
    }
    (two, three)
}