use crate::util::split_tuple_2;
use anyhow::{bail, Error, Result};
use std::{ops::RangeInclusive, str::FromStr};

pub fn part1(_: &str) {}
pub fn part2(_: &str) {}

struct Rule {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_range(s: &str) -> Result<std::ops::RangeInclusive<usize>> {
            if let Some((lower, upper)) = split_tuple_2(s, "-") {
                Ok(RangeInclusive::new(lower.parse()?, upper.parse()?))
            } else {
                bail!("Failed to parse ranges {}", s)
            }
        }
        if let Some((name, ranges)) = split_tuple_2(s, ": ") {
            if let Some((range1, range2)) = split_tuple_2(ranges, " or ") {
                Ok(Rule {
                    name: name.to_owned(),
                    range1: parse_range(range1)?,
                    range2: parse_range(range2)?,
                })
            } else {
                bail!("Did not find `or`-separated ranges in {}", ranges);
            }
        } else {
            bail!("Couldn't parse {}", s);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_rule() {
        let Rule {
            name,
            range1,
            range2,
        } = "class: 1-3 or 5-7".parse().unwrap();
        assert_eq!("class", name.as_str());
        assert_eq!(1..=3, range1);
        assert_eq!(5..=7, range2);
    }
}
