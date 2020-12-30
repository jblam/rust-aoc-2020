use anyhow::{bail, Result};

pub fn get_part1() -> Result<i64> {
    compute(include_str!("data\\day1-input.txt"), 2020, find_pair)
}
pub fn get_part2() -> Result<i64> {
    compute(include_str!("data\\day1-input.txt"), 2020, find_triplet)
}

fn compute(s: &str, sum: i32, f: impl FnOnce(&str, i32) -> Result<Vec<i32>>) -> Result<i64> {
    f(s, sum).map(|vec| vec.iter().fold(1i64, |u, v| u * *v as i64))
}

// JB 2020-12-30: so this is pretty inelegant; apparently itertools::tuple_combinations
// can build us a nice tuple, which would be interesting to write, but is less interesting
// to use.
// ¯\_(ツ)_/¯

fn find_pair(s: &str, sum: i32) -> Result<Vec<i32>> {
    let vec: Result<Vec<_>, _> = s.lines().map(|l| l.parse::<i32>()).collect();
    let vec = vec?;
    if vec.len() < 2 {
        bail!("Not enough members to make a pair");
    }
    for (idx, n) in vec.iter().enumerate() {
        let rest = &vec[idx..];
        for m in rest {
            if m + n == sum {
                return Ok(vec![*n, *m]);
            }
        }
    }
    bail!("No pair of members sums to the expected value");
}

fn find_triplet(s: &str, sum: i32) -> Result<Vec<i32>> {
    let vec: Result<Vec<_>, _> = s.lines().map(|l| l.parse::<i32>()).collect();
    let vec = vec?;
    if vec.len() < 3 {
        bail!("Not enough members to make a triplet");
    }
    for (m_idx, m) in vec.iter().enumerate() {
        let r1 = &vec[m_idx..];
        for (n_idx, n) in r1.iter().enumerate() {
            let r2 = &vec[n_idx..];
            for o in r2 {
                if m + n + o == sum {
                    return Ok(vec![*n, *m, *o]);
                }
            }
        }
    }
    bail!("No triplet of members sums to the desired value");
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMLPE_INPUT: &str = "1721
979
366
299
675
1456";
    #[test]
    fn no_solution_for_short_input() {
        assert!(find_pair("", 2020).is_err());
    }
    #[test]
    fn err_when_parse_fails() {
        assert!(find_pair("asdf\r\nasdf", 2020).is_err());
    }
    #[test]
    fn solution_when_sum() {
        assert_eq!(vec!(1, 2), find_pair("1\r\n2", 3).unwrap());
    }
    #[test]
    fn finds_example() {
        assert_eq!(vec!(1721, 299), find_pair(EXAMLPE_INPUT, 2020).unwrap());
    }
    #[test]
    fn computes_example() {
        assert_eq!(514579, compute(EXAMLPE_INPUT, 2020, find_pair).unwrap());
    }
    #[test]
    fn finds_part2_example() {
        let mut val = find_triplet(EXAMLPE_INPUT, 2020).unwrap();
        val.sort();
        assert_eq!(vec!(366, 675, 979), val);
    }
}
