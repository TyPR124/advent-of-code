mod input;
use self::input::INPUT;

extern crate aoc_2018;
use aoc_2018::{Error, err, Result};

fn main() -> Result<()> {
    let compacted_len = part1(INPUT.trim())?;
    println!("Part1: remaining units: {}", compacted_len);
    Ok(())
}
#[derive(Copy, Clone)]
enum Instruction {
    Skip(usize),
    Take(usize)
}
#[derive(Copy, Clone, Eq, PartialEq)]
enum Action {
    //Compact,
    Continue,
    Complete,
}
#[derive(Clone)]
struct State<'a> {
    bytes: &'a [u8],
    instructions: Vec<Instruction>,
    last_action: Action,
    need: Option<u8>,
    i: usize,
}

struct FinalIterator<'i, 'b> {
    instructions: &'i mut Iterator<Item = &'i Instruction>,
    to_take: usize,
    i: usize,
    bytes: &'b [u8],
}

impl<'i, 'b> FinalIterator<'i, 'b> {
    pub fn new<I: DoubleEndedIterator<Item = &'i Instruction>>(instructions: &'i mut I, bytes: &'b [u8]) -> FinalIterator<'i, 'b> {
        FinalIterator {
            instructions,
            to_take: 0,
            i: 0,
            bytes
        }
    }
}

impl<'i, 'b> Iterator for FinalIterator<'i, 'b> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.to_take > 0 {
            self.i += 1;
            self.to_take -= 1;
            return Some(self.bytes[self.i])
        } else {
            let instr = self.instructions.next()?;
            match instr {
                Instruction::Take(take) => {
                    self.to_take = *take;
                    return Some(self.next()?)
                },
                Instruction::Skip(skip) => {
                    self.i += skip;
                    return Some(self.next()?)
                }
            }
        }
    }
}

impl<'a> State<'a> {
    fn new(bytes: &'a [u8]) -> Result<State> {
        if bytes.len() < 2 {
            err!("No point... too small")
        } else {
            let mut need = bytes[0];
            if need.is_ascii_uppercase() { need.make_ascii_lowercase(); }
            else if need.is_ascii_lowercase() { need.make_ascii_uppercase(); }
            else { return err!("Unexpected character!"); }

            Ok(State {
                bytes,
                instructions: vec![Instruction::Take(1)],
                last_action: Action::Continue,
                need: Some(need),
                i: 0,
            })
        }
    }
    fn build_final(&self) -> Result<String> {
        let mut rev = self.instructions.iter().rev();
        Ok(String::from_utf8(FinalIterator::new(&mut rev, self.bytes).collect())?)
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut state = State::new(input.trim().as_bytes())?;
    run(&mut state)?;
    Ok(state.build_final()?.len())
}

fn run<'a>(state: &mut State) -> Result<()> {
    while {
        let action = step_state(state)?;
        state.last_action = action;
        // while
        action != Action::Complete
    } {}
    Ok(())
}

fn step_state(state: &mut State) -> Result<Action> {
    fn compact<'a>(instructions: &mut Vec<Instruction>, bytes: &'a [u8], i: usize) -> Result<Option<u8>> {
        println!("Compacting...");
        // At this point, i and i-1 msut be compactable
        let mut new_need = None;
        if let Some(instr) = instructions.pop() {
            match instr {
                Instruction::Take(take) => {
                    if take > 1 {
                        instructions.push(Instruction::Take(take-1));
                    }
                    if let Some(last_skip) = instructions.pop() {
                        match last_skip {
                            Instruction::Take(_) => { return err!("Found two takes in a row!") },
                            Instruction::Skip(skip2) => { instructions.push(Instruction::Skip(skip2 + 2)); }
                        }
                    } else {
                        instructions.push(Instruction::Skip(2));
                    }
                    if i < 2 {
                        new_need = None;
                    } else {
                        new_need = Some(bytes[i-2]); // Will transform later
                    }
                },
                Instruction::Skip(skip) => {
                    if skip+1 >= i {
                        return err!("This shouldn't happen. Calling compact with nothing to compact to!")
                    }
                    // skip should never be less than 2
                    // Also, there must be a take, else we wouldn't have anything to compact
                    if let Some(take_instr) = instructions.pop() {
                        match take_instr {
                            Instruction::Skip(_) => return err!("Found skip where expected take!"),
                            Instruction::Take(take) => {
                                if take > 1 {
                                    instructions.push(Instruction::Take(take-1));
                                    instructions.push(Instruction::Skip(skip+2));
                                } else { // take == 1
                                    // Is there another skip?
                                    if let Some(skip_instr) = instructions.pop() {
                                        match skip_instr {
                                            Instruction::Take(_) => return err!("Found take where expected skip!"),
                                            Instruction::Skip(skip2) => { // There is another skip
                                                // If there is another take, then there is a new_need
                                                if let Some(final_take) = instructions.pop() {
                                                    match final_take {
                                                        Instruction::Skip(_) => return err!("Found skip instead of final take!"),
                                                        Instruction::Take(_) => {
                                                            new_need = Some(bytes[i - 1 - skip - 1 - skip2]);
                                                        }
                                                    }
                                                    instructions.push(final_take);
                                                }
                                                instructions.push(Instruction::Skip(skip+skip2+2));
                                            }
                                        }
                                    } else { // No other skip
                                        new_need = None;
                                        instructions.push(Instruction::Skip(skip+2));
                                    }
                                }
                            }
                        }
                    } else { return err!("Expected a take instruction!") }
                    
                }
            }
            if let Some(n) = new_need {
                if n.is_ascii_uppercase() {
                    new_need = Some(n.to_ascii_lowercase());
                } else if n.is_ascii_lowercase() {
                    new_need = Some(n.to_ascii_uppercase());
                } else {
                    return err!("Unexpected character!")
                }
            }
            Ok(new_need)

        } else {
            return err!("Empty Instruction Stack!")
        }
    }
    match state.last_action {
        Action::Complete => { return err!("Complete state found in step_state!") },

        Action::Continue => {
            state.i += 1;
            if state.i == state.bytes.len() { return Ok(Action::Complete) }
            else if let Some(need) = state.need {
                if need == state.bytes[state.i] {
                    state.need = compact(&mut state.instructions, state.bytes, state.i)?;
                }
                return Ok(Action::Continue)
            } else { // Continue
                state.need = if state.bytes[state.i].is_ascii_uppercase() {
                    Some(state.bytes[state.i].to_ascii_lowercase())
                } else if state.bytes[state.i].is_ascii_lowercase() {
                    Some(state.bytes[state.i].to_ascii_uppercase())
                } else {
                    return err!("Unexpected character!")
                };
                if let Some(instr) = state.instructions.pop() {
                    match instr {
                        Instruction::Take(x) => {
                            state.instructions.push(Instruction::Take(x+1));
                        },
                        Instruction::Skip(_) => {
                            state.instructions.push(instr);
                            state.instructions.push(Instruction::Take(1));
                        }
                    }
                } else { return err!("Empty Instruction Stack!") }

                return Ok(Action::Continue)
            }
        },
        
    }
    //Ok(Action::Continue)
}