use anyhow::{bail, Error};
use std::str::FromStr;

pub fn part1(s: &str) -> i32 {
    let mut state = State::new();
    for i in s.lines().map(Instruction::from_str) {
        state = state.next(i.unwrap());
    }
    state.north.abs() + state.east.abs()
}
pub fn part2(_: &str) {}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    TurnLeft,
    TurnRight,
    Forward,
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

struct Instruction(Action, i32);
#[derive(Debug, Clone, Copy, PartialEq)]
struct State {
    east: i32,
    north: i32,
    facing: Facing,
}

impl Facing {
    fn rotate(&self, degrees_left: i32) -> Facing {
        if (degrees_left % 90).abs() != 0 {
            panic!("Unexpected non-cardinal direction {}", degrees_left)
        }
        let clockwise_nineties = -degrees_left / 90;
        let current = match self {
            Facing::North => 0,
            Facing::East => 1,
            Facing::South => 2,
            Facing::West => 3,
        };
        let new = clockwise_nineties + 4 + current;
        match new % 4 {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("Somehow asking for the {}th cardinal", new),
        }
    }
    fn to_move_action(&self) -> Action {
        match self {
            Facing::North => Action::MoveNorth,
            Facing::South => Action::MoveSouth,
            Facing::East => Action::MoveEast,
            Facing::West => Action::MoveWest,
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            north: 0,
            east: 0,
            facing: Facing::East,
        }
    }
    fn next(&self, instruction: Instruction) -> Self {
        let (a, v) = (instruction.0, instruction.1);
        match a {
            Action::MoveNorth => Self {
                north: self.north + v,
                ..*self
            },
            Action::MoveSouth => Self {
                north: self.north - v,
                ..*self
            },
            Action::MoveEast => Self {
                east: self.east + v,
                ..*self
            },
            Action::MoveWest => Self {
                east: self.east - v,
                ..*self
            },
            Action::TurnLeft => Self {
                facing: self.facing.rotate(v),
                ..*self
            },
            Action::TurnRight => Self {
                facing: self.facing.rotate(-v),
                ..*self
            },
            Action::Forward => self.next(Instruction(self.facing.to_move_action(), v)),
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, rest) = s.split_at(1);
        let val = rest.parse::<i32>()?;
        let action = match c {
            "N" => Action::MoveNorth,
            "S" => Action::MoveSouth,
            "E" => Action::MoveEast,
            "W" => Action::MoveWest,
            "L" => Action::TurnLeft,
            "R" => Action::TurnRight,
            "F" => Action::Forward,
            _ => bail!("Unrecognised action: {}", c),
        };
        Ok(Instruction(action, val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn does_example_1() {
        let mut s = State::new();
        for i in EXAMPLE.lines().map(Instruction::from_str) {
            s = s.next(i.unwrap());
        }
        assert_eq!(
            State {
                north: -8,
                east: 17,
                facing: Facing::South
            },
            s
        )
    }
}
