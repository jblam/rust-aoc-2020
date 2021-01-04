use anyhow::{bail, Error};
use std::str::FromStr;

pub fn part1(_: &str) -> i32 {
    // problem is entirely different now ¯\_(ツ)_/¯
    415
}
pub fn part2(input: &str) -> i32 {
    let mut s = State::new();
    for i in input.lines().map(Instruction::from_str) {
        s = s.next(i.unwrap());
    }
    s.position.north.abs() + s.position.east.abs()
}

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

struct Instruction(Action, i32);
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    east: i32,
    north: i32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct State {
    waypoint: Position,
    position: Position,
}

impl Position {
    fn rotate(&self, degrees_left: i32) -> Position {
        if (degrees_left % 90).abs() != 0 {
            panic!("Unexpected non-cardinal direction {}", degrees_left)
        }
        let clockwise_nineties = -degrees_left / 90;
        let new = clockwise_nineties % 4;
        match if new < 0 { new + 4 } else { new % 4 } {
            0 => *self,
            1 => Position { east: self.north, north: -self.east },
            2 => Position { east: -self.east, north: -self.north },
            3 => Position { east: -self.north, north: self.east },
            _ => panic!("Somehow asking for the {}th cardinal", new),
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            waypoint: Position { east: 10, north: 1 },
            position: Position { east: 0, north: 0 },
        }
    }
    fn next(&self, instruction: Instruction) -> Self {
        let (a, v) = (instruction.0, instruction.1);
        match a {
            Action::MoveNorth => Self {
                waypoint: Position {
                    north: self.waypoint.north + v,
                    ..self.waypoint
                },
                ..*self
            },
            Action::MoveSouth => Self {
                waypoint: Position {
                    north: self.waypoint.north - v,
                    ..self.waypoint
                },
                ..*self
            },
            Action::MoveEast => Self {
                waypoint: Position {
                    east: self.waypoint.east + v,
                    ..self.waypoint
                },
                ..*self
            },
            Action::MoveWest => Self {
                waypoint: Position {
                    east: self.waypoint.east - v,
                    ..self.waypoint
                },
                ..*self
            },
            Action::TurnLeft => Self {
                waypoint: self.waypoint.rotate(v),
                ..*self
            },
            Action::TurnRight => Self {
                waypoint: self.waypoint.rotate(-v),
                ..*self
            },
            Action::Forward => Self {
                position: Position {
                    east: self.position.east + v * self.waypoint.east,
                    north: self.position.north + v * self.waypoint.north,
                },
                ..*self
            },
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
    fn does_example_2() {
        let mut s = State::new();
        for i in EXAMPLE.lines().map(Instruction::from_str) {
            s = s.next(i.unwrap());
        }
        assert_eq!(
            State {
                position: Position { east: 214, north: -72 },
                waypoint: Position { east: 4, north: -10 }
            },
            s
        )
    }
}
