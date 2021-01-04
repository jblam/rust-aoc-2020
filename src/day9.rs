use std::ops::Add;

use crate::util::{circular_buffer::CircularBuffer, tuples};

pub fn part1(s: &str) -> Option<i64> {
    const PREAMBLE_LENGTH: usize = 25;
    let mut buffer = CircularBuffer::fill_with(
        s.lines()
            .take(PREAMBLE_LENGTH)
            .map(|l| l.parse::<i64>().unwrap())
            .collect::<Vec<_>>(),
    );
    consume(
        &mut buffer,
        s.lines()
            .skip(PREAMBLE_LENGTH)
            .map(|l| l.parse::<i64>().unwrap()),
    )
}
pub fn part2(s: &str) -> i64 {
    let items = s.lines().map(|l| l.parse().unwrap());
    let (min, max) = consume_p2(items, 25);
    min + max
}

fn chain_tuples<'a, T>(slices: (&'a [T], &'a [T])) -> impl Iterator<Item = (&T, &T)> {
    let (u, v) = slices;
    fn tuple_to<'a, U>(first: &'a U, second: &'a [U]) -> impl Iterator<Item = (&'a U, &'a U)> + 'a {
        second.iter().map(move |s| (first, s))
    }
    tuples(u)
        .chain(tuples(v))
        .chain(u.iter().map(move |a| tuple_to(a, v)).flatten())
        .chain(v.iter().map(move |b| tuple_to(b, u)).flatten())
}

fn is_valid<T: Copy + PartialEq + Add<Output = T>>(buffer: &CircularBuffer<T>, next: T) -> bool {
    chain_tuples(buffer.slices()).any(|(x, y)| *x + *y == next)
}

fn consume<T: Copy + PartialEq + Add<Output = T>>(
    state: &mut CircularBuffer<T>,
    rest: impl Iterator<Item = T>,
) -> Option<T> {
    for item in rest {
        if is_valid(state, item) {
            state.push(item);
        } else {
            return Some(item);
        }
    }
    None
}
fn sums_to(vec: &[i64], i: i64) -> Option<&[i64]> {
    for (idx, total) in vec
        .iter()
        .scan(0i64, |state, cur| {
            *state += cur;
            Some(*state)
        })
        .enumerate()
    {
        if total > i {
            return None;
        } else if total == i {
            return Some(&vec[0..idx]);
        }
    }
    None
}

fn consume_p2<'a>(input: impl Iterator<Item = i64> + 'a, preamble_length: usize) -> (i64, i64) {
    let mut vec = Vec::<i64>::new();
    fn is_valid(vec: &Vec<i64>, i: i64, preamble_length: usize) -> bool {
        let (_, items) = vec.split_at(vec.len() - preamble_length);
        tuples(items).any(|(&u, &v)| u + v == i)
    };
    for candidate in input {
        if vec.len() > preamble_length && !is_valid(&vec, candidate, preamble_length) {
            let range = (0..vec.len())
                .map(|start| sums_to(&vec[start..], candidate))
                .flatten()
                .next()
                .unwrap();
            return (*range.iter().min().unwrap(), *range.iter().max().unwrap());
        } else {
            vec.push(candidate);
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_is_valid() {
        let buffer = CircularBuffer::fill_with(vec![0, 1, 2]);
        assert_eq!(true, is_valid(&buffer, 3))
    }
    #[test]
    fn can_is_not_valid() {
        let buffer = CircularBuffer::fill_with(vec![0, 1, 2]);
        assert_eq!(false, is_valid(&buffer, 5))
    }

    #[test]
    fn validates_example() {
        let mut preamble = CircularBuffer::fill_with(vec![35i32, 20, 15, 25, 47]);
        let candidates = [
            40i32, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
        ];
        assert_eq!(
            Some(127i32),
            consume(&mut preamble, candidates.iter().cloned())
        );
    }
    #[test]
    fn can_sum_to() {
        let candidates = [15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182];
        assert_eq!(Some([15, 25, 47, 40]), sums_to(&candidates, 127))
    }
    #[test]
    fn can_find_sum() {
        let candidates = [
            35i64, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!([15, 25, 47, 40], consume_p2(candidates.iter().copied(), 5));
    }
}
