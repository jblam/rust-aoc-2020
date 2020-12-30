use anyhow::{anyhow, bail, Result};
#[derive(Debug, PartialEq)]
struct Policy<'a> {
    source: &'a str,
    lower: usize,
    upper: usize,
    control: char,
    password_index: usize,
}

pub fn part1(s: &str) -> usize {
    s.lines()
        .map(Policy::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .filter(Policy::is_valid)
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
    fn is_valid(&self) -> bool {
        let count = self
            .password()
            .chars()
            .filter(|c| c == &self.control)
            .count();
        count >= self.lower && count <= self.upper
    }
}

#[cfg(test)]
mod tests {
    use super::{Policy, part1};
    const GOOD_POLICY: &str = "1-3 a: aaa";
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
        assert!(Policy::parse(GOOD_POLICY).unwrap().is_valid())
    }
    #[test]
    fn bad_policy_is_not_valid() {
        assert!(!Policy::parse("1-3 a: bbb").unwrap().is_valid())
    }
    #[test]
    fn example_parses_expected_validity() {
        let example = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let results = example
            .lines()
            .map(Policy::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            vec![true, false, true],
            results.iter().map(Policy::is_valid).collect::<Vec<_>>()
        )
    }
    #[test]
    fn example_is_ok() {
        let example = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(2, part1(example))
    }
}
