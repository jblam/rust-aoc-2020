use std::num::ParseIntError;

pub fn get_part1() -> Result<Option<i64>, ParseIntError> {
    compute(include_str!("data\\day1-input.txt"))
}

fn compute(s: &str) -> Result<Option<i64>, ParseIntError> {
    find_pair(s, 2020).map(|o| o.map(|(u, v)| u as i64 * v as i64))
}

fn find_pair(s: &str, sum: i32) -> Result<Option<(i32, i32)>, ParseIntError> {
    let vec: Result<Vec<_>, _> = s.lines().map(|l| l.parse::<i32>()).collect();
    let vec = vec?;
    if vec.len() < 2 {
        return Ok(None);
    }
    for (idx, n) in vec.iter().enumerate() {
        let rest = &vec[idx..];
        for m in rest {
            if m + n == sum {
                return Ok(Some((*n, *m)));
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
        assert_eq!((1, 2), yup(find_pair("1\r\n2", 3)));
    }
    #[test]
    fn finds_example() {
        assert_eq!((1721, 299), yup(find_pair(EXAMLPE_INPUT, 2020)));
    }
    #[test]
    fn computes_example() {
        assert_eq!(514579, yup(compute(EXAMLPE_INPUT)))
    }
}
