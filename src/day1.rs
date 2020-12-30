use std::num::ParseIntError;

fn compute(s: &str, sum: i32) -> Result<Option<(i32, i32)>, ParseIntError> {
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
    use std::fmt::Debug;

    use super::compute;

    fn yup<T, E: Debug>(val: Result<Option<T>, E>) -> T {
        val.unwrap().unwrap()
    }
    #[test]
    fn no_solution_for_short_input() {
        assert_eq!(Ok(None), compute("", 2020));
    }
    #[test]
    fn err_when_parse_fails() {
        assert!(compute("asdf\r\nasdf", 2020).is_err());
    }
    #[test]
    fn solution_when_sum() {
        assert_eq!((1, 2), yup(compute("1\r\n2", 3)));
    }
    #[test]
    fn finds_example() {
        assert_eq!((1721, 299), yup(compute(r"1721
979
366
299
675
1456", 2020)));
    }
}
