use super::util::split_tuple_2;
use anyhow::{bail, Error, Result};
use std::{collections::HashSet, str::FromStr};

pub fn part1(s: &str) -> Option<State> {
    let instructions = s
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()
        .unwrap();

    let mut states = HashSet::new();
    let iter = {
        let mut current = Some(State::new());
        std::iter::from_fn(move || {
            if let Some(actual) = current.as_ref() {
                current = actual.next(&instructions);
            }
            current
        })
    };
    for state in iter {
        if !states.insert(state.address) {
            return Some(state);
        }
        debug_assert!(states.len() < 1000)
    }
    None
}
pub fn part2(_: &str) {}

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
    #[test]
    fn executes_part1() {
        const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(State{ address: 1, accumulator: 5 }, part1(EXAMPLE).unwrap())
    }
}
