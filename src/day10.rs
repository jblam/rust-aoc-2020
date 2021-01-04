use std::collections::HashMap;
use std::str::FromStr;

pub fn part1(s: &str) -> i32 {
    let input = s
        .lines()
        .map(i32::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let (one, three) = get_chain(input);
    one * three
}
pub fn part2(_: &str) {}

fn get_chain(mut input: Vec<i32>) -> (i32, i32) {
    input.push(0);
    input.sort();
    input.push(input.last().expect("Unexpected empty set") + 3);
    let mut dict = HashMap::new();
    for diff in input.iter().zip(input[1..].iter()).map(|(&u, &v)| v - u) {
        if let Some(count) = dict.get_mut(&diff) {
            *count += 1;
        } else {
            dict.insert(diff, 1);
        }
    }
    (dict[&1], dict[&3])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn gets_chain_ex1() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!((7, 5), get_chain(input))
    }
    #[test]
    fn gets_chain_ex2() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!((22, 10), get_chain(input))
    }
}
