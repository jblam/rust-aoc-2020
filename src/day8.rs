use super::util::split_tuple_2;
use anyhow::{bail, Error};
use std::{ops::Index, str::FromStr};

pub fn part1(_: &str) {}
pub fn part2(_: &str) {}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InstructionKind {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Instruction(InstructionKind, i32);

#[derive(Debug, PartialEq)]
struct State<'a> {
    address: i32,
    accumulator: i32,
    instructions: &'a [Instruction],
}

impl<'a> State<'a> {
    pub fn new(t: &'a [Instruction]) -> State<'a> {
        State {
            address: 0,
            accumulator: 0,
            instructions: t,
        }
    }
    fn next(self) -> Option<Self> {
        if self.address < 0 {
            return None;
        }
        if let Some(Instruction(kind, val)) = self.instructions.get(self.address as usize) {
            Some(match (kind, val) {
                (InstructionKind::Acc, val) => State {
                    accumulator: self.accumulator + val,
                    ..self
                },
                (InstructionKind::Jmp, val) => State {
                    address: self.address + val,
                    ..self
                },
                (InstructionKind::Nop, _) => State {
                    address: self.address + 1,
                    ..self
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
        let init = State::new(&instructions);
        assert_eq!(None, init.next())
    }
    #[test]
    fn can_next() {
        let instructions = [Instruction(InstructionKind::Nop, 0)];
        let init = State::new(&instructions);
        assert_eq!(
            State {
                address: 1,
                accumulator: 0,
                instructions: &instructions
            },
            init.next().unwrap()
        )
    }
}
