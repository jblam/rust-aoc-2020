use crate::util::split_tuple_2;
use anyhow::{anyhow, bail, ensure, Error, Result};
use std::{
    num::ParseIntError,
    ops::RangeInclusive,
    str::{FromStr, Lines},
};

pub fn part1(s: &str) -> usize {
    let prob: Problem = s.parse().unwrap();
    prob.get_tickets(s)
        .map(|t| prob.get_unvalidatable_fields(t.as_ref().unwrap()).collect::<Vec<_>>())
        .flatten()
        .sum()
}
pub fn part2(_: &str) {}

#[derive(Debug)]
struct Ticket(Vec<usize>);
struct Problem {
    rules: Vec<Rule>,
    my_ticket: Ticket,
}

struct Rule {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl Rule {
    fn validates(&self, input: &usize) -> bool {
        self.range1.contains(input) || self.range2.contains(input)
    }
}
impl Problem {
    fn get_tickets<'a>(
        &self,
        s: &'a str,
    ) -> impl Iterator<Item = Result<Ticket, ParseIntError>> + 'a {
        s.lines()
            .skip_while(|&l| l != "nearby tickets:")
            .skip(1)
            .map(|l| l.parse())
    }
    fn get_unvalidatable_fields<'a>(
        &'a self,
        ticket: &'a Ticket,
    ) -> impl Iterator<Item = usize> + 'a {
        ticket
            .0
            .iter()
            .filter(move |num| !self.rules.iter().any(|r| r.validates(num)))
            .copied()
    }
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
impl FromStr for Problem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_rules(lines: &mut Lines) -> Result<Vec<Rule>> {
            let mut vec = Vec::new();
            while let Some(line) = lines.next() {
                if line.len() == 0 {
                    return Ok(vec);
                }
                vec.push(line.parse()?)
            }
            bail!("Unexpected end of file while reading rules")
        }
        let mut lines = s.lines();

        let rules = parse_rules(&mut lines)?;
        anyhow::ensure!(lines.next() == Some("your ticket:"));
        let my_ticket = Ticket(
            lines
                .next()
                .map(|l| {
                    l.split(',')
                        .map(|t| t.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .ok_or(anyhow!("Could not parse ticket"))??,
        );
        ensure!(lines.next() == Some(""));
        ensure!(lines.next() == Some("nearby tickets:"));
        Ok(Problem { rules, my_ticket })
    }
}
impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket(
            s.split(',')
                .map(|t| t.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
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

    #[test]
    fn does_validate_rule() {
        let r = Rule {
            name: "asdf".into(),
            range1: 0..=1,
            range2: 10..=11,
        };
        assert!(r.validates(&0));
        assert!(r.validates(&10));
        assert!(!r.validates(&2));
        assert!(!r.validates(&12));
    }

    const EXAMPLE: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    #[test]
    fn can_parse() {
        let prob = EXAMPLE.parse().unwrap();
        let Problem { rules, my_ticket } = &prob;
        assert_eq!(3, rules.len());
        assert_eq!(vec![7, 1, 14], my_ticket.0);
        assert_eq!(
            vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12]
            ],
            prob.get_tickets(EXAMPLE)
                .map(|r| r.unwrap().0)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn gets_example_1() {
        assert_eq!(71, part1(EXAMPLE))
    }
}
