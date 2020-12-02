use aoc_2018::{err, Error, Result};
use std::str::FromStr;

mod input;
use self::input::INPUT;

fn main() -> Result<()> {
    let mut rules = Vec::new();
    let mut plants = Plants {
        data: Vec::new(),
        offset: 0,
    };
    for (i, linestr) in INPUT.trim().lines().enumerate() {
        let line = linestr.trim().as_bytes();
        if i == 0 {
            const INIT_STATE_LABEL: &str = "initial state: ";
            let initial_state = INIT_STATE_LABEL.as_bytes();
            if line.len() < initial_state.len() + 1 {
                return Err(err!("Initial state line too short"));
            } else {
                for x in line[initial_state.len()..].iter() {
                    match x {
                        b'.' => plants.data.push(false),
                        b'#' => plants.data.push(true),
                        _ => return Err(err!("Invalid character in initial state")),
                    }
                }
            }
        } else if i >= 2 {
            match linestr.parse::<Rule>() {
                Ok(rule) => rules.push(rule),
                Err(e) => match e {
                    RuleFromStrError::Dud => {} // Just skip it
                    RuleFromStrError::Other(e) => return Err(e),
                },
            }
        }
    }
    let mut tmp = Vec::new();
    let mut history = Vec::new();
    history.push(plants);
    'outer: for i in 0..1000 {
        //println!("Step {}: {}", i, history[i].sum());
        plants = history[i].step_cloned(&rules, &mut tmp);
        for (j, p) in history.iter().enumerate() {
            if &plants == p {
                println!("Found cycle: {} -> {}", i + 1, j);
                break 'outer;
            } else if same_pattern(&plants, p) {
                println!(
                    "Found pattern: {} -> {}, offsets: {} -> {}, sums: {} -> {}",
                    i + 1,
                    j,
                    plants.offset,
                    p.offset,
                    plants.sum(),
                    p.sum()
                );
                println!("You can use this information to answer part 2.\nA calculator is easier than coding it correctly.\nAnswer ends up being:\n8646 + (8646 - 8574) * (50000000000 - 92) = 3,600,000,002,022 ( 3600000002022 )");
                break 'outer;
            }
        }
        history.push(plants.clone())
    }
    println!("Part1: Sum after 20 generations: {}", history[20].sum());
    Ok(())
}

#[derive(Clone, Eq, PartialEq)]
struct Plants {
    data: Vec<bool>,
    offset: isize,
}

fn same_pattern(p1: &Plants, p2: &Plants) -> bool {
    p1.data == p2.data
}

impl Plants {
    fn sum(&self) -> isize {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, &v)| v)
            .map(|(i, _)| i as isize + self.offset)
            .sum()
    }
    fn step_cloned(&mut self, rules: &[Rule], tmp: &mut Vec<bool>) -> Plants {
        //println!("Stepping...");
        if self.data.is_empty() {
            return self.clone();
        }
        let lo_hi = [false; 4];
        //let len = lo_buf.len() + hi_buf.len() + self.data.len();
        let old = lo_hi.iter().chain(self.data.iter()).chain(lo_hi.iter());
        tmp.clear();
        tmp.extend(old);
        let mut new = Plants {
            data: Vec::with_capacity(tmp.len() - 4),
            offset: self.offset,
        };
        let mut lo_extra = 2;
        let mut found_first = false;
        tmp.windows(5).for_each(|set| {
            let mut matched_rule = false;
            for rule in rules {
                if rule.matches(set) {
                    matched_rule = true;
                    if !found_first {
                        let r = rule.result();
                        if r {
                            found_first = true;
                            new.data.push(r);
                        } else {
                            lo_extra -= 1;
                        }
                    } else {
                        // found_first
                        new.data.push(rule.result());
                    }
                    break;
                } // rule matches set
            } // rules

            if !matched_rule {
                if found_first {
                    new.data.push(set[2]);
                } else {
                    // Is this the first?
                    if set[2] {
                        found_first = true;
                        new.data.push(true);
                    } else {
                        lo_extra -= 1;
                    }
                }
            }
        }); // set in windows

        if !new.data.is_empty() {
            let mut i = 1;
            while !new.data[new.data.len() - i] {
                i += 1
            }
            new.data.truncate(new.data.len() - (i - 1));
        }
        new.offset -= lo_extra;
        new
    }
}

#[derive(Default)]
struct Rule {
    mval: [bool; 5], // Matching value
}

impl Rule {
    pub fn matches(&self, data: &[bool]) -> bool {
        self.mval == data
    }
    pub fn result(&self) -> bool {
        !self.mval[2]
    }
}

impl FromStr for Rule {
    type Err = RuleFromStrError<Error>;
    fn from_str(s: &str) -> std::result::Result<Rule, Self::Err> {
        let s = s.as_bytes();
        if s.len() != 10 {
            Err(RuleFromStrError::Other(err!("Invalid rule length!")))
        } else if s[2] == s[9] && (s[2] == b'.' || s[2] == b'#') {
            Err(RuleFromStrError::Dud)
        } else if s[5..9] != [b' ', b'=', b'>', b' '] {
            Err(RuleFromStrError::Other(err!("Invalid rule format!")))
        } else {
            let mut r = Rule::default();
            for (i, x) in s.iter().enumerate().take(5) {
                match x {
                    b'.' => r.mval[i] = false,
                    b'#' => r.mval[i] = true,
                    _ => {
                        return Err(RuleFromStrError::Other(err!(
                            "Invalid character in rule body"
                        )))
                    }
                }
            }
            Ok(r)
        }
    }
}

enum RuleFromStrError<O> {
    Dud,      // The rule doesn't do anything; it's a dud
    Other(O), // The parser had an actual error
}

impl<O> From<O> for RuleFromStrError<O> {
    fn from(o: O) -> Self {
        RuleFromStrError::Other(o)
    }
}
