use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Error};
use itertools::Itertools;

use crate::util::split_tuple_2;

pub fn part1(_: &str) {}

pub fn part2(s: &str) -> u64 {
    let mut results = HashMap::new();
    let mut current_mask = None;
    for line in s.lines().map(Line::from_str) {
        match line.unwrap() {
            Line::Mask(mask) => {
                current_mask = Some(mask);
            }
            Line::Assign(assign) => {
                for addr in current_mask
                    .as_ref()
                    .expect("No mask was set before assignment line")
                    .apply(assign.addr)
                {
                    results.insert(addr, assign.value);
                }
            }
        }
    }
    results.values().copied().sum()
}

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
    addr: u64,
    value: u64,
}
struct Mask {
    pos: u64,
    neg: u64,
    floating_bits: Vec<u64>,
}
impl Mask {
    fn apply(&self, val: u64) -> impl Iterator<Item = u64> + '_ {
        let base = (val & self.neg) | self.pos;
        self.floating_bits.iter().map(move |&extra| base | extra)
    }
    fn with_bits(base: u64, bits: impl Iterator<Item = u8>) -> u64 {
        bits.fold(base, |prev, bit| prev | (1 << bit))
    }
}
impl FromStr for Mask {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXPECTED_LENGTH: usize = 36;
        let bytes = s.as_bytes();
        if bytes.len() != EXPECTED_LENGTH {
            bail!("Unexpected length {}", bytes.len())
        }
        let mut pos = 0u64;
        let mut neg = !0u64;
        let mut floating_bits = Vec::new();
        for (idx, &b) in bytes.iter().enumerate() {
            pos <<= 1;
            neg <<= 1;
            match b {
                b'X' => floating_bits.push((EXPECTED_LENGTH - 1 - idx) as u8),
                b'0' => {
                    neg += 1;
                }
                b'1' => {
                    neg += 1;
                    pos += 1;
                }
                unexpected => bail!("Unexpected byte: {}", unexpected),
            }
        }
        let floating_bits = (0..=floating_bits.len())
            .map(|len| floating_bits.iter().copied().combinations(len))
            .flatten()
            .map(|bits| Mask::with_bits(0, bits.iter().copied()))
            .collect::<Vec<_>>();
        Ok(Mask {
            pos,
            neg,
            floating_bits,
        })
    }
}
#[cfg(test)]
mod tests {
    use itertools::*;
    use std::mem::discriminant;

    use super::*;

    #[test]
    fn can_add_bits() {
        let base = 26u64;
        let bits = [0, 5];
        assert_eq!(59, Mask::with_bits(base, bits.iter().copied()))
    }

    #[test]
    fn can_apply_mask() {
        let sut = Mask {
            pos: 0b10010,
            neg: !0b100001,
            floating_bits: vec![0b0, 0b100000, 0b1, 0b100001],
        };
        let mut v = dbg!(sut.apply(42).collect::<Vec<_>>());
        v.sort();
        assert_eq!(vec![26, 27, 58, 59], v)
    }
    #[test]
    fn can_parse_mask() {
        let Mask {
            pos,
            neg,
            floating_bits,
        } = "000000000000000000000000000000X1001X".parse().unwrap();
        assert_eq!(pos, 0b10010);
        assert_eq!(neg, !0b100001);
        assert_eq!(floating_bits, vec![0b0, 0b100000, 0b1, 0b100001]);
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
            discriminant(&Line::Mask(Mask {
                pos: 0,
                neg: 0,
                floating_bits: Default::default()
            })),
            discriminant(
                &"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                    .parse::<Line>()
                    .unwrap()
            )
        )
    }

    const EXAMPLE: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    #[test]
    fn can_run_example() {
        assert_eq!(208, part2(EXAMPLE))
    }
}
