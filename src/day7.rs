use anyhow::{bail, Result};

pub fn part1(_: &str) {}
pub fn part2(_: &str) {}

#[derive(Debug, PartialEq)]
struct Descriptor<'a>(&'a str);

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    owner: Descriptor<'a>,
    contents: Vec<(usize, Descriptor<'a>)>,
}

fn split_tuple_2<'source, 'pattern>(
    s: &'source str,
    pat: &'pattern str,
) -> Option<(&'source str, &'source str)> {
    let mut tokens = s.splitn(2, pat);
    if let (Some(a), Some(b)) = (tokens.next(), tokens.next()) {
        if tokens.next().is_some() {
            panic!("splitn unexpectedly returned too many values")
        }
        Some((a, b))
    } else {
        None
    }
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
        let contents = parts[1]
            .split(", ")
            .map(|content| {
                if let Some((qty, descriptor)) = split_tuple_2(content, " ") {
                    Ok((qty.parse()?, Descriptor::parse(descriptor)?))
                } else {
                    bail!("Couldn't parse content {}", content)
                }
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Rule { owner, contents })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_tuple_split() {
        assert_eq!(
            ("hello", "this is dog"),
            split_tuple_2("hello this is dog", " ").unwrap()
        )
    }
    #[test]
    fn tuple_split_fails_gracefully() {
        assert_eq!(None, split_tuple_2("nooooope", "oops"))
    }
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
                contents: vec![(1, Descriptor("asdf")), (2, Descriptor("jkl;"))]
            },
            Rule::parse("owner bags contain 1 asdf bag, 2 jkl; bags").unwrap()
        )
    }
}
