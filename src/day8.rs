use super::util::split_tuple_2;
use anyhow::{bail, Error, Result};
use std::{collections::HashSet, str::FromStr};

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()
        .unwrap()
}
fn get_outcome(slice: &[Instruction]) -> (Outcome, State) {
    let mut current = State::new();
    let iter = std::iter::once(current).chain(std::iter::from_fn(|| {
        if let Some(next) = current.next(slice) {
            current = next;
            Some(current)
        } else {
            None
        }
    }));
    let mut visited = HashSet::new();
    for state in iter {
        if !visited.insert(state.address) {
            return (Outcome::Loop, state);
        }
        debug_assert!(visited.len() < 10000);
    }
    (Outcome::Terminate, current)
}

pub fn part1(s: &str) -> (Outcome, State) {
    get_outcome(&parse_instructions(s))
}
pub fn part2(s: &str) -> Option<State> {
    let mut instructions = parse_instructions(s);
    fn get_flippables(
        i: &[Instruction],
    ) -> impl Iterator<Item = (usize, Instruction, Instruction)> + '_ {
        i.iter()
            .clone()
            .enumerate()
            .filter_map(|(idx, &Instruction(kind, val))| match kind {
                InstructionKind::Jmp => Some((
                    idx,
                    Instruction(kind, val),
                    Instruction(InstructionKind::Nop, val),
                )),
                InstructionKind::Nop => Some((
                    idx,
                    Instruction(kind, val),
                    Instruction(InstructionKind::Jmp, val),
                )),
                _ => None,
            })
    }
    let flippables = {
        let vecref = &instructions;
        get_flippables(vecref).collect::<Vec<_>>()
    };
    for (idx, original, flipped) in flippables {
        instructions[idx] = flipped;
        if let (Outcome::Terminate, winner) = get_outcome(&instructions) {
            return Some(winner);
        }
        instructions[idx] = original;
    }
    None
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Outcome {
    Loop,
    Terminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstructionKind {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction(InstructionKind, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    address: i32,
    accumulator: i32,
}

impl State {
    fn new() -> State {
        State {
            address: 0,
            accumulator: 0,
        }
    }
    fn next(&self, instructions: &[Instruction]) -> Option<Self> {
        if self.address < 0 {
            return None;
        }
        if let Some(Instruction(kind, val)) = instructions.get(self.address as usize) {
            Some(match (kind, val) {
                (InstructionKind::Acc, val) => State {
                    accumulator: self.accumulator + val,
                    address: self.address + 1,
                },
                (InstructionKind::Jmp, val) => State {
                    address: self.address + val,
                    ..*self
                },
                (InstructionKind::Nop, _) => State {
                    address: self.address + 1,
                    ..*self
                },
            })
        } else {
            None
        }
    }
}
impl FromStr for InstructionKind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "nop" => Self::Nop,
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            _ => bail!("Unrecognised instruction: {}", s),
        })
    }
}
impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((kind, arg)) = split_tuple_2(s, " ") {
            Ok(Instruction(kind.parse()?, arg.parse()?))
        } else {
            bail!("could not tokenise line")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        assert_eq!(
            Instruction(InstructionKind::Acc, 4),
            "acc +4".parse().unwrap()
        );
        assert_eq!(
            Instruction(InstructionKind::Jmp, -4),
            "jmp -4".parse().unwrap()
        );
    }
    #[test]
    fn can_terminate() {
        let instructions: [Instruction; 0] = Default::default();
        let init = State::new();
        assert_eq!(None, init.next(&instructions))
    }
    #[test]
    fn can_next() {
        let instructions = [Instruction(InstructionKind::Nop, 0)];
        let init = State::new();
        assert_eq!(
            State {
                address: 1,
                accumulator: 0,
            },
            init.next(&instructions).unwrap()
        )
    }
    const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    const FIXED_EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";
    #[test]
    fn executes_part1() {
        assert_eq!(
            (
                Outcome::Loop,
                State {
                    address: 1,
                    accumulator: 5
                }
            ),
            part1(EXAMPLE)
        )
    }
    #[test]
    fn validates_part2_example_answer() {
        let (outcome, _) = get_outcome(&parse_instructions(EXAMPLE));
        assert_eq!(Outcome::Loop, outcome);
        let (
            fixed_outcome,
            State {
                accumulator: fixed_accumulator,
                ..
            },
        ) = get_outcome(&parse_instructions(FIXED_EXAMPLE));
        assert_eq!(Outcome::Terminate, fixed_outcome);
        assert_eq!(8, fixed_accumulator);
    }
    #[test]
    fn gets_part2_example() {
        assert_eq!(part2(EXAMPLE).unwrap().accumulator, 8)
    }
}
