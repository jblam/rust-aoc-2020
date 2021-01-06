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
        .map(|t| {
            prob.get_unvalidatable_fields(t.as_ref().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .sum()
}
pub fn part2(s: &str) -> usize {
    /// Takes a vector of alternative-options, and returns a vector containing the single solution
    fn flat_dedup<T: PartialEq + std::fmt::Debug>(mut vec: Vec<Vec<&T>>) -> Result<Vec<&T>> {
        /// Deduplicates options from the source multivec if they are unique in the dest single-vec,
        /// returning `true` if at least one element could be transferred
        fn transfer<'a, U: PartialEq + std::fmt::Debug>(
            source: &mut Vec<Vec<&'a U>>,
            dest: &mut Vec<Option<&'a U>>,
        ) -> Result<bool> {
            for unavailable in dest.iter().filter_map(|&i| i) {
                for options in source.iter_mut() {
                    options.retain(|&el| el != unavailable);
                }
            }
            let mut did_thing = false;
            for (idx, single) in source.iter_mut().enumerate().filter(|(_, v)| v.len() == 1) {
                let dest_cell = dest.get_mut(idx).unwrap();
                let mut val = single.remove(0);
                if let Some(existing) = dest_cell.replace(&mut val) {
                    bail!(
                        "Unexpectedly replaced an existing unique solution at index {} ({:?}→{:?})",
                        idx,
                        existing,
                        dest_cell
                    );
                } else {
                    did_thing = true;
                }
            }
            Ok(did_thing)
        }
        let mut out: Vec<Option<&T>> = std::iter::repeat_with(|| None).take(vec.len()).collect();
        for (idx, items) in vec.iter_mut().enumerate() {
            if items.len() == 1 {
                out[idx] = Some(items.remove(0));
            }
        }
        let mut cycles = 0;
        while transfer(&mut vec, &mut out)? {
            cycles += 1;
        }
        out.into_iter()
            .enumerate()
            .map(|(idx, el)| {
                el.ok_or(anyhow!(
                    "Could not resolve duplicates at index {} after {} cycles; {:?}",
                    idx,
                    cycles,
                    vec[idx],
                ))
            })
            .collect::<Result<Vec<_>>>()
    }

    let prob: Problem = s.parse().unwrap();
    let mut rules_for_position = std::iter::repeat(prob.rules.iter().collect::<Vec<_>>())
        .take(prob.my_ticket.0.len())
        .collect::<Vec<_>>();
    let good_tickets = prob
        .get_tickets(s)
        .map(|t| t.unwrap())
        .filter(|t| prob.get_unvalidatable_fields(t).next().is_none());
    for Ticket(fields) in good_tickets {
        debug_assert!(fields.len() == rules_for_position.len());
        for (field, rules_for_field) in fields.iter().zip(rules_for_position.iter_mut()) {
            debug_assert!(rules_for_field.len() > 0);
            rules_for_field.retain(|&rule| rule.validates(field));
        }
    }
    dbg!(&rules_for_position);
    let rules_for_position = dbg!(flat_dedup(rules_for_position)).unwrap();
    rules_for_position
        .into_iter()
        .enumerate()
        .filter(|(_idx, rule)| rule.name.starts_with("departure"))
        .map(|(idx, rule)| {
            debug_assert!(rule.validates(&prob.my_ticket.0[idx]));
            prob.my_ticket.0[idx]
        })
        .fold(1, |prev, cur| prev * cur)
}

#[derive(Debug)]
struct Ticket(Vec<usize>);
struct Problem {
    rules: Vec<Rule>,
    my_ticket: Ticket,
}

#[derive(PartialEq)]
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
impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?} or {:?}", self.name, self.range1, self.range2)
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
    #[test]
    fn gets_example_2() {
        assert_eq!(1, part2(EXAMPLE))
    }
}
