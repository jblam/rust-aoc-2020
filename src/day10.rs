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
pub fn part2(s: &str) -> usize {
    let input = s
        .lines()
        .map(i32::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    count_chains(input)
}

fn format_vec(input: &mut Vec<i32>) {
    input.push(0);
    input.sort();
    input.push(input.last().expect("Unexpected empty set") + 3);
    debug_assert!({
        let initial_length = input.len();
        input.dedup();
        initial_length == input.len()
    })
}

fn get_chain(mut input: Vec<i32>) -> (i32, i32) {
    format_vec(&mut input);
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

fn count_chains(mut input: Vec<i32>) -> usize {
    fn count_chain_parts(next: &[i32]) -> usize {
        fn is_valid(input: i32, output: i32) -> bool {
            output > input && (output - input) < 4
        }
        if let [head, tail @ ..] = next {
            if tail.len() == 0 {
                1
            } else {
                tail.iter()
                    .enumerate()
                    .take_while(|(_, &v)| is_valid(*head, v))
                    .map(|(idx, _)| count_chain_parts(&tail[idx..]))
                    .sum()
            }
        } else {
            unreachable!()
        }
    }
    format_vec(&mut input);
    count_chain_parts(&input)
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE_1: [i32; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const EXAMPLE_2: [i32; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    #[test]
    fn gets_chain_ex1() {
        let input = Vec::from(EXAMPLE_1);
        assert_eq!((7, 5), get_chain(input))
    }
    #[test]
    fn gets_chain_ex2() {
        let input = Vec::from(EXAMPLE_2);
        assert_eq!((22, 10), get_chain(input))
    }

    #[test]
    fn counts_chain_ex1() {
        let input = Vec::from(EXAMPLE_1);
        assert_eq!(8, count_chains(input))
    }
    #[test]
    fn counts_chain_ex2() {
        let input = Vec::from(EXAMPLE_2);
        assert_eq!(19208, count_chains(input))
    }
}
