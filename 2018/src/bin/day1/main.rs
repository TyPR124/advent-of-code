mod input;
use self::input::INPUT;

fn main() {
    let end_freq = part1(0, INPUT);
    println!("Part1: Ending frequency: {}", end_freq);
    let twice = part2(0, INPUT);
    println!("Part2: First Frequency Found Twice: {}", twice);
}

fn part1(mut start: isize, input: &str) -> isize {
    for line in input.lines() {
        start += line.parse::<isize>().expect("Could not parse line");
    }
    start
}

fn part2(mut start: isize, input: &str) -> isize {
    use std::collections::HashSet;
    use std::hash::Hash;
    let mut freqs = HashSet::<isize>::new();
    loop {
        for line in input.lines() {
            start += line.parse::<isize>().expect("Could not parse line");
            if freqs.contains(&start) {
                return start;
            } else {
                freqs.insert(start);
            }
        }
    }
}