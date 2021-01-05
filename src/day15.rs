use std::{collections::HashMap, ops::Sub};

pub fn part1(_: &[i32]) {}
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

fn enumerate(init: &[i32]) -> impl Iterator<Item = (Number, Turn)> + '_ {
    init.iter()
        .enumerate()
        .map(|(idx, &val)| (Number(val), Turn(idx as i32 + 1)))
}

fn init_map(init: &[i32]) -> (State, Turn) {
    let s = enumerate(init)
        .map(|(Number(val), turn)| (val, turn))
        .collect::<HashMap<_, _>>();
    (State(s), Turn(init.len() as i32 + 1))
}

// PLEASE MR COMPILER KEEP TELLING ME HOW `todo()` ISN'T SIZEABLE.

// fn play(init: &[i32]) -> impl Iterator<Item = Number> {
//     let (mut state, first_turn) = init_map(init);
//     // let add = |num: Number, turn: Turn| -> Number {
//     //     if let Some(last_spoken) = state.0.insert(num.0, turn) {
//     //         turn - last_spoken
//     //     } else {
//     //         Number(0)
//     //     }
//     // };

//     // let seq = std::iter::successors(Option::<(Number, Turn)>::None, |prev| {
//     //     if let Some((num, turn)) = prev {
//     //         let next_num = add(num, turn);
//     //         Some(Some((next_num, Turn(turn.0 + 1))))
//     //     } else {
//     //         Some((Number(0), first_turn))
//     //     }
//     // });

//     // init.iter()
//     //     .map(|&v| Number(v))
//     //     .chain(todo!())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init() {
        let (u, v) = init_map(&[0, 3, 6]);
        assert_eq!(u.0.len(), 3);
        assert_eq!(Turn(4), v);
    }
}
