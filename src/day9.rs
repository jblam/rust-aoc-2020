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
            .map(|l| l.parse::<i64>().unwrap())
    )
}
pub fn part2(_: &str) {}

fn build_buffer<T>(init: &[T]) -> CircularBuffer<T>
where
    T: Copy,
{
    let mut output = CircularBuffer::with_capacity(init.len());
    for t in init {
        output.push(*t);
    }
    output
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
}
