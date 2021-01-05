use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Error};

use crate::util::split_tuple_2;

pub fn part1(s: &str) -> u64 {
    let mut results = HashMap::new();
    let mut current_mask = None;
    for line in s.lines().map(Line::from_str) {
        match line.unwrap() {
            Line::Mask(mask) => {
                current_mask = Some(mask);
            }
            Line::Assign(assign) => {
                results.insert(assign.addr, current_mask.as_ref().unwrap().apply(assign.value));
            }
        }
    }
    results.values().copied().sum()
}
pub fn part2(_: &str) {}

enum Line {
    Mask(Mask),
    Assign(Assign),
}
impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const MEM_START: &str = "mem[";
        if let Some((lhs, rhs)) = split_tuple_2(s, " = ") {
            if lhs == "mask" {
                Ok(Line::Mask(rhs.parse()?))
            } else if lhs.starts_with(MEM_START) && lhs.ends_with("]") {
                Ok(Line::Assign(Assign {
                    addr: lhs[MEM_START.len()..lhs.len() - 1].parse()?,
                    value: rhs.parse()?,
                }))
            } else {
                bail!("Unexpected assignment {}", lhs)
            }
        } else {
            bail!("Not a key-value pair")
        }
    }
}
struct Assign {
    addr: usize,
    value: u64,
}
struct Mask {
    pos: u64,
    neg: u64,
}
impl Mask {
    fn apply(&self, val: u64) -> u64 {
        (val & self.neg) | self.pos
    }
}
impl FromStr for Mask {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 36 {
            bail!("Unexpected length {}", bytes.len())
        }
        let mut pos = 0u64;
        let mut neg = !0u64;
        for b in bytes {
            pos <<= 1;
            neg <<= 1;
            match b {
                b'X' => {
                    neg += 1;
                }
                b'0' => (),
                b'1' => {
                    neg += 1;
                    pos += 1;
                }
                unexpected => bail!("Unexpected byte: {}", unexpected),
            }
        }
        Ok(Mask { pos, neg })
    }
}
#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use super::*;

    #[test]
    fn can_apply_mask() {
        let sut = Mask {
            pos: 0b1000000,
            neg: !0b10,
        };
        assert_eq!(sut.apply(11), 73);
        assert_eq!(sut.apply(101), 101);
        assert_eq!(sut.apply(0), 64);
    }
    #[test]
    fn can_parse_mask() {
        let Mask { pos, neg } = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();
        assert_eq!(pos, 0b1000000);
        assert_eq!(neg, !0b10);
    }
    #[test]
    fn can_parse_assign_line() {
        assert_eq!(
            discriminant(&Line::Assign(Assign { addr: 0, value: 0 })),
            discriminant(&"mem[0] = 0".parse::<Line>().unwrap())
        )
    }
    #[test]
    fn can_parse_mask_line() {
        assert_eq!(
            discriminant(&Line::Mask(Mask { pos: 0, neg: 0 })),
            discriminant(
                &"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                    .parse::<Line>()
                    .unwrap()
            )
        )
    }

    const EXAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    #[test]
    fn gets_example() {
        assert_eq!(part1(EXAMPLE), 165)
    }
}
