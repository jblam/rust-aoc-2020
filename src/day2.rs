use anyhow::{anyhow, bail, Result};
#[derive(Debug, PartialEq)]
pub struct Policy<'a> {
    source: &'a str,
    lower: usize,
    upper: usize,
    control: char,
    password_index: usize,
}

pub fn evaluate<T: Validation>(s: &str) -> usize {
    s.lines()
        .map(Policy::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .filter(T::is_valid)
        .count()
}

impl<'a> Policy<'a> {
    fn parse(s: &'a str) -> Result<Policy<'a>> {
        fn single<T>(mut iter: impl Iterator<Item = T>) -> Result<T> {
            if let Some(t) = iter.next() {
                if iter.next().is_none() {
                    return Ok(t);
                }
            }
            bail!("Iterator did not contain a single value")
        }

        let colon_idx = s.find(':').ok_or(anyhow!("Invalid password rule"))?;
        let hyphen_idx = s[..colon_idx]
            .find('-')
            .ok_or(anyhow!("Invalid password rule"))?;
        let space_idx = hyphen_idx
            + s[hyphen_idx..colon_idx]
                .find(' ')
                .ok_or(anyhow!("Invalid password rule"))?;
        Ok(Policy {
            source: s,
            lower: s[..hyphen_idx].parse()?,
            upper: s[hyphen_idx + 1..space_idx].parse()?,
            control: single(s[space_idx + 1..colon_idx].chars())?,
            password_index: colon_idx + 2,
        })
    }
    fn password(&self) -> &'a str {
        &self.source[self.password_index..]
    }
    fn is_valid<T: Validation>(&self) -> bool {
        T::is_valid(self)
    }
}

pub trait Validation {
    fn is_valid(policy: &Policy) -> bool;
}
pub struct PartOne {}
pub struct PartTwo {}

impl Validation for PartOne {
    fn is_valid(policy: &Policy) -> bool {
        let count = policy
            .password()
            .chars()
            .filter(|c| c == &policy.control)
            .count();
        count >= policy.lower && count <= policy.upper
    }
}
impl Validation for PartTwo {
    fn is_valid(policy: &Policy) -> bool {
        (policy.control
            == policy.password()[..=policy.upper - 1]
                .chars()
                .last()
                .unwrap())
            ^ (policy.control
                == policy.password()[policy.lower - 1..]
                    .chars()
                    .next()
                    .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate, PartOne, PartTwo, Policy, Validation};
    const GOOD_POLICY: &str = "1-3 a: aaa";
    const EXAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    #[test]
    fn parses_ok() {
        assert_eq!(
            Policy {
                source: GOOD_POLICY,
                lower: 1,
                upper: 3,
                control: 'a',
                password_index: 7
            },
            Policy::parse(GOOD_POLICY).unwrap()
        );
    }
    #[test]
    fn gets_password() {
        assert_eq!("aaa", Policy::parse(GOOD_POLICY).unwrap().password());
    }
    #[test]
    fn good_policy_is_valid() {
        assert!(Policy::parse(GOOD_POLICY).unwrap().is_valid::<PartOne>())
    }
    #[test]
    fn bad_policy_is_not_valid() {
        assert!(!Policy::parse("1-3 a: bbb").unwrap().is_valid::<PartOne>())
    }
    #[test]
    fn example_parses_expected_validity() {
        let results = EXAMPLE
            .lines()
            .map(Policy::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            vec![true, false, true],
            results.iter().map(PartOne::is_valid).collect::<Vec<_>>()
        )
    }
    #[test]
    fn example_is_ok() {
        assert_eq!(2, evaluate::<PartOne>(EXAMPLE))
    }

    #[test]
    fn second_example_parses_expected() {
        let results = EXAMPLE
            .lines()
            .map(Policy::parse)
            .map(|r| r.map(|v| PartTwo::is_valid(&v)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(vec![true, false, false], results)
    }
}
