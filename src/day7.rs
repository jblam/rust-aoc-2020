use crate::util;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use anyhow::{bail, Context, Result};
use multimap::MultiMap;

pub fn part1(s: &str) -> usize {
    let map = reverse(s.lines().map(|l| Rule::parse(l).unwrap()));
    let mut output = HashSet::new();
    let initial = (1, Rc::new(Descriptor("shiny gold")));
    output = append(&map, initial.clone(), output);
    fn append<'a>(
        source: &'a MultiMap<Descriptor<'a>, (usize, Rc<Descriptor<'a>>)>,
        key: (usize, Rc<Descriptor<'a>>),
        result: HashSet<Rc<Descriptor<'a>>>,
    ) -> HashSet<Rc<Descriptor<'a>>> {
        let mut r = result;
        if r.insert(key.1.clone()) {
            if let Some(items) = source.get_vec(&key.1) {
                for (qty, item) in items {
                    r = append(source, (*qty, item.clone()), r);
                }
            }
        }
        r
    }
    output.remove(initial.1.as_ref());
    output.len()
}
pub fn part2(s: &str) -> usize {
    fn walk<'m>(
        map: &'m HashMap<Descriptor<'m>, Vec<(usize, Descriptor<'m>)>>,
        m: &'m Descriptor,
        qty: usize,
    ) -> usize {
        let kids = (&map[m]).iter().map(|(k_qty, k)| walk(map, k, k_qty * qty));
        let sum = kids.sum::<usize>();
        qty + sum
    }
    let forward = s.lines().map(|l| Rule::parse(l).unwrap());
    let mut m = HashMap::new();
    for rule in forward {
        let _existing = m.insert(rule.owner, rule.contents);
        debug_assert_eq!(None, _existing, "Unexpected duplicate definition");
    }

    walk(&m, &Descriptor("shiny gold"), 1) - 1
}

fn reverse<'a>(
    rules: impl Iterator<Item = Rule<'a>>,
) -> MultiMap<Descriptor<'a>, (usize, Rc<Descriptor<'a>>)> {
    let mut multimap = MultiMap::new();
    for rule in rules {
        let owner = Rc::new(rule.owner);
        for (qty, content) in rule.contents {
            multimap.insert(content, (qty, owner.clone()));
        }
    }
    multimap
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Descriptor<'a>(&'a str);

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    owner: Descriptor<'a>,
    contents: Vec<(usize, Descriptor<'a>)>,
}

impl Descriptor<'_> {
    fn parse(s: &str) -> Result<Descriptor<'_>> {
        if let Some(end) = s.find(" bag") {
            match &s[end + 4..] {
                "" => Ok(Descriptor(&s[..end])),
                "s" => Ok(Descriptor(&s[..end])),
                _ => bail!("Expected token `bag` or `bags` did not terminate the rule"),
            }
        } else {
            bail!("Did not find the terminating `bag` or `bags` token")
        }
    }
}
impl<'a> Rule<'a> {
    fn parse(s: &'a str) -> Result<Rule<'a>> {
        let parts = s.splitn(2, " contain ").collect::<Vec<_>>();
        if parts.len() != 2 {
            bail!("Expected keyword `contain` not found");
        }
        let owner = Descriptor::parse(parts[0])?;
        let contents = if parts[1].ends_with('.') {
            &parts[1][..parts[1].len() - 1]
        } else {
            bail!("Expected trailing '.' not found in {}", parts[1])
        };
        let contents = if contents == "no other bags" {
            Vec::new()
        } else {
            contents
                .split(", ")
                .map(|content| -> Result<(usize, Descriptor), anyhow::Error> {
                    if let Some((qty, descriptor)) = split_tuple_2(content, " ") {
                        Ok((
                            qty.parse().with_context(|| format!("Parsing {}", qty))?,
                            Descriptor::parse(descriptor)?,
                        ))
                    } else {
                        bail!("Couldn't parse content {}", content)
                    }
                })
                .collect::<Result<Vec<_>>>()?
        };
        Ok(Rule { owner, contents })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_descriptor() {
        assert_eq!("asdf", Descriptor::parse("asdf bag").unwrap().0)
    }
    #[test]
    fn can_parse_plural_descriptor() {
        assert_eq!("asdf", Descriptor::parse("asdf bags").unwrap().0)
    }
    #[test]
    fn can_parse_rule() {
        assert_eq!(
            Rule {
                owner: Descriptor("owner"),
                contents: vec![(1, Descriptor("asdf")), (2, Descriptor("jkl"))]
            },
            Rule::parse("owner bags contain 1 asdf bag, 2 jkl bags.").unwrap()
        )
    }
    const EXAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    #[test]
    fn can_reverse() {
        let map = reverse(EXAMPLE.lines().map(|l| Rule::parse(l).unwrap()));
        let owners = map.get_vec(&Descriptor("shiny gold")).unwrap();
        assert_eq!(2, owners.len());
        assert!(owners
            .iter()
            .any(|x| x.1.as_ref() == &Descriptor("bright white")));
    }

    #[test]
    fn gets_example_part1() {
        assert_eq!(4, part1(EXAMPLE))
    }
    #[test]
    fn gets_single_rule_part2() {
        assert_eq!(0, part2("shiny gold bags contain no other bags."));
    }
    #[test]
    fn gets_simple_rules_part2() {
        assert_eq!(
            1,
            part2(
                "shiny gold bags contain 1 dark red bags.
dark red bags contain no other bags."
            )
        );
    }
    #[test]
    fn gets_multiplying_rules() {
        assert_eq!(
            2,
            part2(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain no other bags."
            )
        );
    }
    #[test]
    fn gets_example_part2() {
        assert_eq!(32, part2(EXAMPLE));
    }
    #[test]
    fn gets_example_part2a() {
        const EXAMPLE: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(126, part2(EXAMPLE));
    }
}
