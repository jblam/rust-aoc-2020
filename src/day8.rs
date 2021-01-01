use super::util::split_tuple_2;
use anyhow::{bail, Error};
use std::str::FromStr;

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

struct State<T: Iterator<Item = Instruction>> {
    address: usize,
    accumulator: i32,
    history: Vec<Instruction>,
    remaining: T,
}

impl<T: Iterator<Item = Instruction>> State<T> {
    pub fn new(t: T) -> State<T> {
        State {
            remaining: t,
            address: 0,
            accumulator: 0,
            history: Default::default(),
        }
    }
    fn parse_next(&mut self) -> Option<Instruction> {
        if let Some(i) = self.remaining.next() {
            self.history.push(i);
            Some(i)
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
        assert_eq!(Instruction(InstructionKind::Acc, 4), "acc +4".parse().unwrap());
        assert_eq!(Instruction(InstructionKind::Jmp, -4), "jmp -4".parse().unwrap());
    }
}
