use anyhow::{bail, Context, Error};
use std::str::FromStr;

pub(crate) fn part1(s: &str) -> usize {
    s.lines()
        .map(|l| {
            l.parse::<Position>()
                .with_context(|| format!("Could not parse {}", l))
                .unwrap()
                .get_id()
        })
        .max()
        .expect("No lines")
}
pub(crate) fn part2(_: &str) {}

struct Position([u8; 10]);

const FRONT: u8 = b'F';
const BACK: u8 = b'B';
const LEFT: u8 = b'L';
const RIGHT: u8 = b'R';

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != 10 {
            bail!("Wrong length")
        }
        for &v in &b[..7] {
            if v != BACK && v != FRONT {
                bail!("Invalid front-back {}", v)
            }
        }
        for &v in &b[7..] {
            if v != LEFT && v != RIGHT {
                bail!("Invalid left-right {}", v)
            }
        }
        // MaybeUninit is hard, let's just overwrite 10 bytes.
        let mut output = [0u8; 10];
        output.copy_from_slice(b);
        Ok(Position(output))
    }
}

fn get_id_from_index(index: (usize, usize)) -> usize {
    index.0 * 8 + index.1
}

impl Position {
    fn get_index(&self) -> (usize, usize) {
        self.0.iter().fold((0, 0), |(row, col), &cur| match cur {
            FRONT => (row << 1, col),
            BACK => (1 + (row << 1), col),
            LEFT => (row, col << 1),
            RIGHT => (row, 1 + (col << 1)),
            _ => unreachable!(),
        })
    }
    fn get_id(&self) -> usize {
        get_id_from_index(self.get_index())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        assert!("FBFBBFFRLR".parse::<Position>().unwrap().0.len() == 10)
    }
    #[test]
    fn can_get_zero_index() {
        assert_eq!(
            (0, 0),
            "FFFFFFFLLL".parse::<Position>().unwrap().get_index()
        )
    }
    #[test]
    fn can_get_last_index() {
        assert_eq!(
            (127, 7),
            "BBBBBBBRRR".parse::<Position>().unwrap().get_index()
        )
    }
    #[test]
    fn can_get_example_index() {
        assert_eq!(
            (44, 5),
            "FBFBBFFRLR".parse::<Position>().unwrap().get_index()
        );
        assert_eq!(
            (70, 7),
            "BFFFBBFRRR".parse::<Position>().unwrap().get_index()
        );
        assert_eq!(
            (14, 7),
            "FFFBBBFRRR".parse::<Position>().unwrap().get_index()
        );
        assert_eq!(
            (102, 4),
            "BBFFBBFRLL".parse::<Position>().unwrap().get_index()
        );
    }
    #[test]
    fn can_get_id() {
        assert_eq!(get_id_from_index((44, 5)), 357);
        assert_eq!(get_id_from_index((70, 7)), 567);
        assert_eq!(get_id_from_index((14, 7)), 119);
        assert_eq!(get_id_from_index((102, 4)), 820);
    }
}
