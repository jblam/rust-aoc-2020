use std::num::ParseIntError;

pub fn get_part1() -> Result<Option<i64>, ParseIntError> {
    compute(include_str!("data\\day1-input.txt"), 2020, find_pair)
}
pub fn get_part2() -> Result<Option<i64>, ParseIntError> {
    compute(include_str!("data\\day1-input.txt"), 2020, find_triplet)
}

fn compute(s: &str, sum: i32, f: impl FnOnce(&str, i32) -> Result<Option<Vec<i32>>, ParseIntError>) -> Result<Option<i64>, ParseIntError> {
    f(s, sum).map(|o| o.map(|vec| vec.iter().fold(1i64, |u, v| u * *v as i64)))
}

fn find_pair(s: &str, sum: i32) -> Result<Option<Vec<i32>>, ParseIntError> {
    let vec: Result<Vec<_>, _> = s.lines().map(|l| l.parse::<i32>()).collect();
    let vec = vec?;
    if vec.len() < 2 {
        return Ok(None);
    }
    for (idx, n) in vec.iter().enumerate() {
        let rest = &vec[idx..];
        for m in rest {
            if m + n == sum {
                return Ok(Some(vec!(*n, *m)));
            }
        }
    }
    Ok(None)
}

fn find_triplet(s: &str, sum: i32) -> Result<Option<Vec<i32>>, ParseIntError> {
    let vec: Result<Vec<_>, _> = s.lines().map(|l| l.parse::<i32>()).collect();
    let vec = vec?;
    if vec.len() < 3 {
        return Ok(None);
    }
    for (m_idx, m) in vec.iter().enumerate() {
        let r1 = &vec[m_idx..];
        for (n_idx, n) in r1.iter().enumerate() {
            let r2 = &vec[n_idx..];
            for o in r2 {
                if m + n + o == sum {
                    return Ok(Some(vec!(*n, *m, *o)));
                }
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;
    const EXAMLPE_INPUT: &str = "1721
979
366
299
675
1456";

    fn yup<T, E: Debug>(val: Result<Option<T>, E>) -> T {
        val.unwrap().unwrap()
    }
    #[test]
    fn no_solution_for_short_input() {
        assert_eq!(Ok(None), find_pair("", 2020));
    }
    #[test]
    fn err_when_parse_fails() {
        assert!(find_pair("asdf\r\nasdf", 2020).is_err());
    }
    #[test]
    fn solution_when_sum() {
        assert_eq!(vec!(1, 2), yup(find_pair("1\r\n2", 3)));
    }
    #[test]
    fn finds_example() {
        assert_eq!(vec!(1721, 299), yup(find_pair(EXAMLPE_INPUT, 2020)));
    }
    #[test]
    fn computes_example() {
        assert_eq!(514579, yup(compute(EXAMLPE_INPUT, 2020, find_pair)));
    }
    #[test]
    fn finds_part2_example() {
        let mut val = yup(find_triplet(EXAMLPE_INPUT, 2020));
        val.sort();
        assert_eq!(vec!(366, 675, 979), val);
    }
}
