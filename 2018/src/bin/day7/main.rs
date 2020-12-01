const REGEX_STR: &str = "Step (?P<need>[A-Z]) must be finished before step (?P<node>[A-Z]) can begin.";

mod input;
use self::input::INPUT;

extern crate aoc_2018;
use aoc_2018::{Result};

extern crate regex;
use regex::Regex;

extern crate lazy_static;
use lazy_static::lazy_static;

use std::collections::{BTreeSet, BTreeMap};


lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(REGEX_STR).expect("Failed to init lazy_static Regex");
}

fn main() -> Result<()> {
    
    let order = part1(INPUT)?;
    println!("Part1: Order: {}", order);
    let seconds_passed = part2(INPUT, 5)?; // 4 elves + me
    println!("Part2: Seconds Passed: {}", seconds_passed);

    Ok(())
}

fn part1(input: &str) -> Result<String> {
    type DepMap = BTreeMap<u8, BTreeSet<u8>>;
    fn step(dep_map: &mut DepMap) -> Result<u8> {
        // #[allow(clippy::clippy::skip_while_next)]
        let (&next, _) = dep_map.iter()
            .find(|(_,needs)| needs.is_empty()).expect("Couldn't find empty needs");
        dep_map.remove(&next);
        dep_map.iter_mut().for_each(|(_, needs)| {
            needs.remove(&next);
        });
        Ok(next)
    }
    let mut dep_map = DepMap::new();
    for node in b'A'..=b'Z' { // Ensure every node has an entry, even if blank
        dep_map.insert(node, Default::default());
    }
    for line in input.trim().lines() {
        let caps = LINE_REGEX.captures(line)?;
        let node = caps.name("node")?.as_str().as_bytes()[0];
        let need = caps.name("need")?.as_str().as_bytes()[0];
        dep_map.entry(node).or_default().insert(need);
    }
    let mut out = String::with_capacity(26);
    while !dep_map.is_empty() {
        out.push(step(&mut dep_map)? as char);
    }
    Ok(out)
}

fn part2(input: &str, worker_count: usize) -> Result<usize> {
    #[derive(PartialEq)]
    enum Status {
        HaveNeeds(BTreeSet<u8>),
        CanDo,
        Doing,
    }
    impl Default for Status {
        fn default() -> Status { Status::CanDo }
    }
    type DepMap = BTreeMap<u8, Status>;
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Worker {
        Free,
        Working(u8, usize)
    }
    impl Worker {
        fn is_working(&self) -> bool {
            matches!(self, Worker::Working(_, _))
        }
    }

    fn time_required(step: u8) -> usize {
        const BASE: isize = 60;
        const DECR: isize = b'A' as isize - 1; // if step == 'A', step-('A'-1) = 'A' - 'A' + 1 = 1
        let time = BASE + step as isize - DECR; // BASE + (step - DECR)
        //println!("Time required for step {}: {}", step as char, time);
        time as usize
    }

    fn next_nodes(dep_map: &mut DepMap, todo: &mut Vec<u8>, max: usize) {
        todo.extend(
            dep_map.iter_mut()
            .filter(|(_, status)| status == &&Status::CanDo)
            .take(max)
            .map(|(node, status)| {
                *status = Status::Doing;
                *node
            })
        );
    }

    let mut dep_map = DepMap::new();
    for node in b'A'..=b'Z' {
        dep_map.insert(node, Default::default());
    }
    for line in input.trim().lines() {
        let caps = LINE_REGEX.captures(line)?;
        let node = caps.name("node")?.as_str().as_bytes()[0];
        let need = caps.name("need")?.as_str().as_bytes()[0];
        dep_map.entry(node).and_modify(|status| {
            match status {
                Status::HaveNeeds(needs) => {needs.insert(need);},
                Status::CanDo => {
                    let mut needs = BTreeSet::new();
                    needs.insert(need);
                    *status = Status::HaveNeeds(needs);
                },
                _ => panic!("Found invalid initial Status"),
            }
        }).or_insert_with(|| panic!("Found bad node value!"));
    }
    let mut seconds_passed = 0;
    let mut workers = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        workers.push(Worker::Free);
    }
    let mut todo = Vec::with_capacity(worker_count);
    while !dep_map.is_empty() {
        //print!("At t = {}", seconds_passed);
        seconds_passed += tick(&mut dep_map, &mut workers, &mut todo)?;
    }

    return Ok(seconds_passed);


    fn tick(dep_map: &mut DepMap, workers: &mut Vec<Worker>, todo: &mut Vec<u8>) -> Result<usize> {
        let &passing_time = workers.iter().filter(|w| w.is_working()).map(|w| {
            if let Worker::Working(_, t) = w {
                t
            } else {
                panic!("Got a non-working worker in a filtered iter")
            }
        }).min().unwrap_or(&0);

        //println!(" + {}", passing_time);
        let mut n_free = 0;
        workers.iter_mut().for_each(|w| {
            match w {
                Worker::Free => n_free += 1,
                Worker::Working(node, t) => {
                    *t -= passing_time;
                    if *t == 0 {
                        dep_map.remove(node);
                        dep_map.iter_mut().for_each( |(_, status)| {
                            if let Status::HaveNeeds(needs) = status {
                                needs.remove(node);
                                if needs.is_empty() {
                                    *status = Status::CanDo;
                                }
                            }
                        }); // removing node from needs
                        *w = Worker::Free;
                        n_free += 1;
                    } // if t == 0
                } // w was working
            } // match w
        }); // workers.iter_mut()...
        //println!("Found {} free workers", n_free);
        assert_eq!(todo.len(), 0, "Todo is not empty at start of loop?"); //todo.clear();
        next_nodes(dep_map, todo, n_free);
        //println!("Found {} available jobs", todo.len());
        workers.iter_mut().filter(|w| w == &&Worker::Free)
            .take(todo.len())
            .for_each(|w| {
                let node = todo.pop().unwrap();
                *w = Worker::Working(node, time_required(node));
            });
        if dep_map.is_empty() {
            let final_time = workers.iter().map(|w| {
                if let Worker::Working(_, t) = w {
                    *t
                } else {
                    0
                }
            }).max()?;
            Ok(passing_time + final_time)
        } else {
            Ok(passing_time)
        }
    }
}