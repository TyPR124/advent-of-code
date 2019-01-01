mod input;
use self::input::INPUT;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

use std::cmp::Ordering;
use std::str::FromStr;

type Error = Box<std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let (p1, p2) = part1_and_part2(INPUT)?;
    println!("Part1: Sleepy guard * minute most slept: {}", p1);
    println!("Part2: Most sleepy minute * the guard who dun it: {}", p2);
    Ok(())
}

fn part1_and_part2(input: &str) -> Result<(u64, u64)> {
    let mut entries = Vec::<Entry>::new();
    for line in input.lines() {
        entries.push(line.parse()?);
    }
    entries.sort();

    use std::collections::HashMap;
    use std::ops::Range;
    use std::ops::AddAssign;
    let mut current_guard: u64 = 0;
    let mut asleep_since: usize = 0;
    let mut total_sleep = HashMap::new();
    let mut sleep_times = HashMap::new();
    entries.iter().for_each(|e|
        match e.detail {
            Detail::NewGuard(g) => current_guard = g,
            Detail::GuardAsleep => asleep_since = e.date.minute,
            Detail::GuardAwake => {
                let sleep_range = Range {start: asleep_since, end: e.date.minute};
                total_sleep.entry(current_guard).or_insert(0).add_assign(sleep_range.end - sleep_range.start);
                sleep_times.entry(current_guard).or_insert(Vec::new()).push(sleep_range);
            }
        }
    );
    let mut max_sleep = 0;
    let mut sleepy_guard = 0;
    for kv in total_sleep {
        if kv.1 > max_sleep {
            max_sleep = kv.1;
            sleepy_guard = kv.0;
        }
    }

    let mut sleepy_minutes = HashMap::new();
    for r in sleep_times[&sleepy_guard].iter() {
        for m in r.clone() {
            sleepy_minutes.entry(m).or_insert(0).add_assign(1);
        }
    }

    let mut most_sleepy_minute = 0;
    let mut sleepy_minute_count = 0;
    for kv in sleepy_minutes {
        if kv.1 > sleepy_minute_count {
            sleepy_minute_count = kv.1;
            most_sleepy_minute = kv.0;
        }
    }
    let p1 = sleepy_guard * most_sleepy_minute as u64;

    // Part 2: Go through sleep times, find most frequest minute for each guard
    //let mut guards_sleepy_minutes = HashMap::new();
    let mut sleepy_guard = 0;
    let mut sleepy_minute = 0;
    let mut sleepy_minute_freq = 0;
    let mut current_sleepy_minutes = HashMap::new();

    for kv in sleep_times {
        current_sleepy_minutes.clear();
        for r in kv.1 {
            for m in r {
                current_sleepy_minutes.entry(m).or_insert(0).add_assign(1);
            }
        }
        let mut max = 0;
        let mut minute = 0;
        for kv in &current_sleepy_minutes {
            if *kv.1 > max {
                max = *kv.1;
                minute = *kv.0;
            }
        }
        //let v = guards_sleepy_minutes.entry(kv.0).or_insert((0, 0));
        //v.0 = minute;
        //v.1 = max;
        if max > sleepy_minute_freq {
            sleepy_minute_freq = max;
            sleepy_minute = minute;
            sleepy_guard = kv.0;
        }
    }
    let p2 = sleepy_guard * sleepy_minute as u64;

    Ok((p1, p2))
}

#[derive(Eq, PartialEq)]
struct Entry {
    pub date: DateTime,
    pub detail: Detail,
}

impl FromStr for Entry {
    type Err = Error;
    fn from_str(s: &str) -> Result<Entry> {
        lazy_static! {
            static ref entry_regex: Regex = Regex::new(r#"^\[([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)\] (.+)$"#).unwrap();
        }

        if let Some(caps) = entry_regex.captures(s) {
            Ok(Entry{
                date: DateTime{
                    year: caps[1].parse()?,
                    month: caps[2].parse()?,
                    day: caps[3].parse()?,
                    hour: caps[4].parse()?,
                    minute: caps[5].parse()?,
                },
                detail: caps[6].parse()?
            })
        } else {
            Err(Box::from("Could not find regex matches"))
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum Detail {
    NewGuard(u64),
    GuardAwake,
    GuardAsleep,
}

impl FromStr for Detail {
    type Err = Error;
    fn from_str(s: &str) -> Result<Detail> {
        lazy_static! {
            static ref newguard_regex: Regex = Regex::new(r#"^Guard #([0-9]+) begins shift$"#).unwrap();
        }
        if let Some(caps) = newguard_regex.captures(s) {
            Ok(Detail::NewGuard(caps[1].parse()?))
        } else { match s {
            "wakes up" => Ok(Detail::GuardAwake),
            "falls asleep" => Ok(Detail::GuardAsleep),
            _ => Err(Box::from("Found unexpected detail line"))
        }}
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DateTime {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl std::cmp::Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl std::cmp::Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> Ordering {
        if self.year > other.year {
            Ordering::Greater
        } else if self.year < other.year {
            Ordering::Less
        } else if self.month > other.month {
            Ordering::Greater
        } else if self.month < other.month {
            Ordering::Less
        } else if self.day > other.day {
            Ordering::Greater
        } else if self.day < other.day {
            Ordering::Less
        } else if self.hour > other.hour {
            Ordering::Greater
        } else if self.hour < other.hour {
            Ordering::Less
        } else if self.minute > other.minute {
            Ordering::Greater
        } else if self.minute < other.minute {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        if self.year > other.year {
            Some(Ordering::Greater)
        } else if self.year < other.year {
            Some(Ordering::Less)
        } else if self.month > other.month {
            Some(Ordering::Greater)
        } else if self.month < other.month {
            Some(Ordering::Less)
        } else if self.day > other.day {
            Some(Ordering::Greater)
        } else if self.day < other.day {
            Some(Ordering::Less)
        } else if self.hour > other.hour {
            Some(Ordering::Greater)
        } else if self.hour < other.hour {
            Some(Ordering::Less)
        } else if self.minute > other.minute {
            Some(Ordering::Greater)
        } else if self.minute < other.minute {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}