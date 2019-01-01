mod input;
use self::input::INPUT;

fn main() {
    let p1 = part1(INPUT);
    println!("Part1: Length of compacted: {}", p1);
    let p2 = part2(INPUT);
    println!("Part2: Length of shorted compacted: {}", p2);
}

fn part1(input: &str) -> usize {
    let input = input.trim().as_bytes();
    let mut new = Vec::<u8>::with_capacity(input.len());
    input.iter().fold(&mut new, |new, b| {
        if new.len() == 0 {
            new.push(*b);
            new
        } else {
            if is_match(new[new.len()-1], *b) {
                new.pop();
                new
            } else {
                new.push(*b);
                new
            }
        }
    }).len()
}

fn part2(input: &str) -> usize {
    use std::collections::HashMap;
    let input = input.trim().as_bytes();
    let mut all = HashMap::new();
    for b in b'A'..=b'Z' {
        all.insert(b, Vec::<u8>::with_capacity(input.len()));
    }
    let all_final = input.iter().fold(&mut all, |all, b| {
        for kv in all.iter_mut() {
            if *kv.0 != b.to_ascii_uppercase() {
                if kv.1.len() == 0 {
                    kv.1.push(*b);
                } else {
                    if is_match(kv.1[kv.1.len()-1], *b) {
                        kv.1.pop();
                    } else {
                        kv.1.push(*b);
                    }
                }
            }
        }
        all
    });

    let mut min = std::usize::MAX;
    all_final.iter().for_each(|kv| {
        if kv.1.len() < min {
            min = kv.1.len()
        }
    });

    min
}

fn is_match(a: u8, b: u8) -> bool {
    a != b && (a.to_ascii_lowercase() == b || a.to_ascii_uppercase() == b)
}