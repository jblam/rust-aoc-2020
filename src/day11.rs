use anyhow::{bail, Error, Result};
use std::{
    convert::TryFrom,
    fmt::{Debug, Display},
};

pub fn part1(_: &str) {}
pub fn part2(_: &str) {}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Full,
}

impl Display for SeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Floor => '.',
                Self::Empty => 'L',
                Self::Full => '#',
            }
        )
    }
}

impl TryFrom<u8> for SeatState {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => SeatState::Floor,
            b'L' => SeatState::Empty,
            b'#' => SeatState::Full,
            _ => bail!("Unexpected token {}", value),
        })
    }
}

#[derive(PartialEq)]
struct SeatMap(Vec<Vec<SeatState>>);
struct SeatMapPair(usize, SeatMap, SeatMap);

impl SeatMapPair {
    pub fn parse(s: &str) -> Result<SeatMapPair> {
        let first = SeatMap::parse(s)?;
        let second = SeatMap(first.0.clone());
        Ok(SeatMapPair(0, first, second))
    }
    pub fn current(&self) -> &SeatMap {
        if self.0 % 2 == 0 {
            &self.1
        } else {
            &self.2
        }
    }
    pub fn step(&mut self) {
        let (active, next) = if self.0 % 2 == 0 {
            (&self.1, &mut self.2)
        } else {
            (&self.2, &mut self.1)
        };
        SeatMap::step(active, next);
        self.0 += 1;
    }
}

impl SeatMap {
    fn parse(s: &str) -> Result<SeatMap> {
        let mut lines = s
            .lines()
            .map(|l| {
                std::iter::once(Ok(SeatState::Floor))
                    .chain(l.as_bytes().iter().copied().map(SeatState::try_from))
                    .chain(std::iter::once(Ok(SeatState::Floor)))
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        let (first, last) = {
            let expected = lines.first().ok_or(Error::msg("Empty lines"))?.len();
            let mut empty = Vec::with_capacity(expected);
            while empty.len() < expected {
                empty.push(SeatState::Floor);
            }
            (empty.clone(), empty)
        };
        lines.insert(0, first);
        lines.push(last);
        Ok(SeatMap(lines))
    }

    fn step(source: &SeatMap, dest: &mut SeatMap) {
        fn triples<T>(source: &Vec<T>) -> impl Iterator<Item = (usize, &T, &T, &T)> {
            source
                .iter()
                .enumerate()
                .zip(source[1..].iter())
                .zip(source[2..].iter())
                .map(|(((idx, a), b), c)| (idx, a, b, c))
        }
        for (row, r_prev, r_current, r_next) in triples(&source.0) {
            for col in 1..r_current.len() - 1 {
                let range = col - 1..=col + 1;
                fn count_occupants(seat: &SeatState) -> usize {
                    match seat {
                        SeatState::Full => 1,
                        _ => 0,
                    }
                }
                let neighbour_count: usize = r_prev[range.clone()]
                    .iter()
                    .chain(r_current[range.clone()].iter())
                    .chain(r_next[range].iter())
                    .map(count_occupants)
                    .sum();
                let value = match r_current[col] {
                    SeatState::Floor => SeatState::Floor,
                    SeatState::Empty => {
                        if neighbour_count == 0 {
                            SeatState::Full
                        } else {
                            SeatState::Empty
                        }
                    }
                    // NB neighbour_count also counts the current seat
                    SeatState::Full => {
                        if neighbour_count > 4 {
                            SeatState::Empty
                        } else {
                            SeatState::Full
                        }
                    }
                };
                dest.0[row + 1][col] = value;
            }
        }
    }
}

impl Debug for SeatMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SeatMap ({} empty, {} full)",
            self.0.iter().flatten().filter(|&&s| s == SeatState::Empty).count(),
            self.0.iter().flatten().filter(|&&s| s == SeatState::Full).count()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SINGLE: &str = "L";
    const EXAMPLE_0: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    const EXAMPLE_1: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    #[test]
    fn can_parse() {
        let map = SeatMapPair::parse(SINGLE).unwrap();
        assert_eq!(map.current().0[1][1], SeatState::Empty);
    }
    #[test]
    fn can_step() {
        let mut map = SeatMapPair::parse(SINGLE).unwrap();
        map.step();
        let expected = SeatMap::parse("#").unwrap();
        assert_eq!(map.current().0[1][1], SeatState::Full);
        assert_eq!(expected, *map.current());
    }
    #[test]
    fn can_step_example() {
        let mut map = SeatMapPair::parse(EXAMPLE_0).unwrap();
        map.step();
        let expected = SeatMap::parse(EXAMPLE_1).unwrap();
        assert_eq!(expected, *map.current());
    }
}
