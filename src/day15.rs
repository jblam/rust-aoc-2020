use std::{collections::HashMap, ops::Sub};

pub fn part1(init: &[i32]) -> std::option::Option<i32> {
    enumerate(init).skip(2019).next().map(|Number(n)| n)
}
pub fn part2(_: &[i32]) {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Turn(i32);
#[derive(Debug, Clone, Copy, PartialEq)]
struct Number(i32);

impl Sub for Turn {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

struct State(HashMap<i32, Turn>);
struct Step(Number, Turn);

fn enumerate(init: &[i32]) -> impl Iterator<Item = Number> + '_ {
    let (mut state, last_turn) = State::create(init);
    let start = init.iter().map(|&i| Number(i));
    let rest = std::iter::successors(Some(last_turn), move |prev| Some(state.step(prev)))
        .map(|Step(num, _)| num);
    start.chain(rest)
}

impl State {
    fn create(init: &[i32]) -> (State, Step) {
        let s = init
            .iter()
            .enumerate()
            .map(|(idx, &val)| (val, Turn(idx as i32 + 1)))
            .collect::<HashMap<_, _>>();
        (
            State(s),
            Step(Number(0), Turn(1 + init.len() as i32)),
        )
    }
    fn step(&mut self, Step(num, turn): &Step) -> Step {
        let next_num = if let Some(last_spoken) = self.0.insert(num.0, *turn) {
            *turn - last_spoken
        } else {
            Number(0)
        };
        Step(next_num, Turn(turn.0 + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init() {
        let (state, Step(prev_num, prev_turn)) = State::create(&[0, 3, 6]);
        assert_eq!(state.0.len(), 3);
        assert_eq!(0, prev_num.0);
        assert_eq!(Turn(4), prev_turn);
    }
    #[test]
    fn can_sequence() {
        assert_eq!(vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0], enumerate(&[0, 3, 6]).map(|Number(num)| num).take(10).collect::<Vec<_>>())
    }
    #[test]
    fn runs_example() {
        assert_eq!(1, part1(&[1, 3, 2]).unwrap());
        assert_eq!(10, part1(&[2, 1, 3]).unwrap());
        assert_eq!(27, part1(&[1, 2, 3]).unwrap());
        assert_eq!(78, part1(&[2, 3, 1]).unwrap());
        assert_eq!(438, part1(&[3, 2, 1]).unwrap());
        assert_eq!(1836, part1(&[3, 1, 2]).unwrap());
    }
}
