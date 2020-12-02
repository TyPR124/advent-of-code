mod input;
use self::input::INPUT;

extern crate aoc_2018;
use aoc_2018::Result;

fn main() -> Result<()> {
    let p1 = part1(INPUT);
    println!("Part1: Length of compacted: {}", p1);
    let p2 = part2(INPUT)?;
    println!("Part2: Length of shortest compacted: {}", p2);
    Ok(())
}

fn part1(input: &str) -> usize {
    let input = input.trim().as_bytes();
    let mut new = Vec::<u8>::with_capacity(input.len());
    input.iter().for_each(|b| {
        if new.is_empty() {
            new.push(*b);
        } else if is_match(new[new.len() - 1], *b) {
            new.pop();
        } else {
            new.push(*b);
        }
    });
    new.len()
}

fn part2(input: &str) -> Result<usize> {
    use std::collections::HashMap;
    let input = input.trim().as_bytes();
    let mut all = HashMap::new();
    for b in b'A'..=b'Z' {
        all.insert(b, Vec::<u8>::with_capacity(input.len()));
    }
    input.iter().for_each(|b| {
        all.iter_mut()
            .filter(|kv| *kv.0 != b.to_ascii_uppercase())
            .for_each(|kv| {
                if kv.1.is_empty() {
                    kv.1.push(*b);
                } else if is_match(kv.1[kv.1.len() - 1], *b) {
                    kv.1.pop();
                } else {
                    kv.1.push(*b);
                }
            });
    });
    Ok(all.values().map(|v| v.len()).min()?)
}

fn is_match(a: u8, b: u8) -> bool {
    a != b && (a.to_ascii_lowercase() == b || a.to_ascii_uppercase() == b)
}
