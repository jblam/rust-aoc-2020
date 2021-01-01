use std::collections::HashSet;

pub fn part1(s: &str) -> usize {
    to_union_counts(s).map(Result::unwrap).sum()
}
pub fn part2(s: &str) -> usize {
    to_intersection_counts(s).sum()
}

fn to_union_counts(s: &str) -> impl Iterator<Item = Result<usize, String>> + '_ {
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

fn to_intersection_counts(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.split("\n\n").map(|t| {
        let line_bytes = t.lines().map(|l| l.bytes().collect::<HashSet<_>>());
        let mut intersect: Option<HashSet<u8>> = None;
        for set in line_bytes {
            intersect = Some(if let Some(mut prev) = intersect {
                prev.retain(|e| set.contains(e));
                prev
            } else {
                set
            })
        }
        intersect.map_or(0, |s| s.len())
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
    fn gets_union_counts() {
        assert_eq!(
            vec![3, 3, 3, 1, 1],
            to_union_counts(EXAMPLE)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        )
    }
    #[test]
    fn gets_example_part1() {
        assert_eq!(11, part1(EXAMPLE));
    }

    #[test]
    fn gets_intersect_counts() {
        assert_eq!(
            vec![3, 0, 1, 1, 1],
            to_intersection_counts(EXAMPLE).collect::<Vec<_>>()
        )
    }
    #[test]
    fn gets_example_part2() {
        assert_eq!(6, part2(EXAMPLE))
    }
}
