use std::collections::HashSet;

pub fn part1(s: &str) -> usize {
    to_groups(s).map(Result::unwrap).sum()
}
pub fn part2(_: &str) {}

fn to_groups(s: &str) -> impl Iterator<Item = Result<usize, String>> + '_ {
    s.split("\n\n").map(|t| {
        t.bytes()
            .filter(|b| !b.is_ascii_whitespace())
            .map(|b| {
                if b.is_ascii_lowercase() {
                    Ok(b)
                } else {
                    Err(format!("Bad: {}", b))
                }
            })
            .collect::<Result<HashSet<_>, String>>()
            .map(|s| s.len())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    #[test]
    fn gets_groups() {
        assert_eq!(
            vec![3, 3, 3, 1, 1],
            to_groups(EXAMPLE).collect::<Result<Vec<_>, _>>().unwrap()
        )
    }
    #[test]
    fn gets_example_part1() {
        assert_eq!(11, part1(EXAMPLE));
    }
}
