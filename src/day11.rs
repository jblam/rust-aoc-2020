use anyhow::{bail, Error, Result};
use std::{
    convert::TryFrom,
    fmt::{Debug, Display},
};

pub fn part1(s: &str) -> usize {
    let mut pair = SeatMapPair::parse(s).unwrap();
    while !pair.is_stable() {
        pair.step(SeatMap::step);
    }
    pair.current()
        .0
        .iter()
        .flatten()
        .filter(|&&s| s == SeatState::Full)
        .count()
}
pub fn part2(s: &str) -> usize {
    let mut pair = SeatMapPair::parse(s).unwrap();
    while !pair.is_stable() {
        pair.step(SeatMap::step_2);
    }
    pair.current()
        .0
        .iter()
        .flatten()
        .filter(|&&s| s == SeatState::Full)
        .count()
}

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
    pub fn step(&mut self, f: impl FnOnce(&SeatMap, &mut SeatMap)) {
        let (active, next) = if self.0 % 2 == 0 {
            (&self.1, &mut self.2)
        } else {
            (&self.2, &mut self.1)
        };
        f(active, next);
        self.0 += 1;
    }
    pub fn is_stable(&self) -> bool {
        self.0 > 0 && self.1 == self.2
    }
    pub fn step_count(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl Direction {
    fn offset(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = pos;
        match self {
            Self::East => Some((x, y + 1)),
            Self::SouthEast => Some((x + 1, y + 1)),
            Self::South => Some((x + 1, y)),
            Self::SouthWest => y.checked_sub(1).and_then(|v| Some((x + 1, v))),
            Self::West => y.checked_sub(1).and_then(|v| Some((x, v))),
            Self::NorthWest => x
                .checked_sub(1)
                .and_then(|u| y.checked_sub(1).and_then(|v| Some((u, v)))),
            Self::North => x.checked_sub(1).and_then(|u| Some((u, y))),
            Self::NorthEast => x.checked_sub(1).and_then(|u| Some((u, y + 1))),
        }
    }
}
const ALL_DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

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

    fn cardinal_dir(
        &self,
        pos: (usize, usize),
        direction: Direction,
    ) -> impl Iterator<Item = &SeatState> {
        std::iter::successors(Some(pos), move |&p| direction.offset(p))
            .skip(1)
            .map(move |(row, col)| self.0.get(row).and_then(|v| v.get(col)))
            .take_while(|o| o.is_some())
            .map(|s| s.unwrap())
    }
    fn neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = &SeatState> {
        ALL_DIRECTIONS
            .iter()
            .map(move |&d| {
                self.cardinal_dir(pos, d)
                    .filter(|&&s| s != SeatState::Floor)
                    .next()
            })
            .flatten()
    }

    fn step_2(source: &SeatMap, dest: &mut SeatMap) {
        for row in 1..source.0.len() - 1 {
            for col in 1..source.0[row].len() - 1 {
                let current = source.0[row][col];
                if current != SeatState::Floor {
                    let neighbour_count = source
                        .neighbours((row, col))
                        .filter(|&&s| s == SeatState::Full)
                        .count();
                    dest.0[row][col] = match current {
                        SeatState::Full => {
                            if neighbour_count >= 5 {
                                SeatState::Empty
                            } else {
                                SeatState::Full
                            }
                        }
                        SeatState::Empty => {
                            if neighbour_count == 0 {
                                SeatState::Full
                            } else {
                                SeatState::Empty
                            }
                        }
                        _ => unreachable!(),
                    };
                }
            }
        }
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
            self.0
                .iter()
                .flatten()
                .filter(|&&s| s == SeatState::Empty)
                .count(),
            self.0
                .iter()
                .flatten()
                .filter(|&&s| s == SeatState::Full)
                .count()
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
        map.step(SeatMap::step);
        let expected = SeatMap::parse("#").unwrap();
        assert_eq!(map.current().0[1][1], SeatState::Full);
        assert_eq!(expected, *map.current());
    }
    #[test]
    fn can_step_example() {
        let mut map = SeatMapPair::parse(EXAMPLE_0).unwrap();
        map.step(SeatMap::step);
        let expected = SeatMap::parse(EXAMPLE_1).unwrap();
        assert_eq!(expected, *map.current());
    }
    #[test]
    fn can_find_stable_example() {
        let mut map = SeatMapPair::parse(EXAMPLE_0).unwrap();
        while !map.is_stable() {
            debug_assert!(map.step_count() < 100);
            map.step(SeatMap::step);
        }
        assert_eq!(6, map.step_count());
        assert_eq!(
            *map.current(),
            SeatMap::parse(
                "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"
            )
            .unwrap()
        );
        assert_eq!(
            37,
            map.current()
                .0
                .iter()
                .flatten()
                .filter(|&&s| s == SeatState::Full)
                .count()
        )
    }
    #[test]
    fn can_get_cardinal_dir() {
        let map = SeatMap::parse(SINGLE).unwrap();
        assert_eq!(map.neighbours((1, 1)).count(), 0);
    }

    #[test]
    fn can_get_cardinal_dir_full() {
        let map = SeatMap::parse(
            "###
###
###",
        )
        .unwrap();
        let n = map.neighbours((2, 2)).copied().collect::<Vec<_>>();
        assert_eq!(n.len(), 8);
        assert!(n.iter().all(|&s| s == SeatState::Full));
    }

    #[test]
    fn cardinals_ignore_empties() {
        let map = SeatMap::parse("L.L#").unwrap();
        let east = map.cardinal_dir((1, 1), Direction::East);
        // expect one "out-of-bounds" position
        assert_eq!(
            vec![
                SeatState::Floor,
                SeatState::Empty,
                SeatState::Full,
                SeatState::Floor
            ],
            east.copied().collect::<Vec<_>>()
        )
    }

    #[test]
    fn can_step_example_2() {
        let mut pair = SeatMapPair::parse(EXAMPLE_0).unwrap();
        let expected = SeatMap::parse(
            "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
        )
        .unwrap();
        pair.step(SeatMap::step_2);
        pair.step(SeatMap::step_2);
        assert_eq!(*pair.current(), expected);
    }
}
