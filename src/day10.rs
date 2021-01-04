use std::collections::HashMap;
use std::str::FromStr;

use multimap::MultiMap;

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
    let mut input = s
        .lines()
        .map(i32::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let edges = build_graph(&mut input);
    walk_graph(&edges)
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

fn build_graph(input: &mut Vec<i32>) -> Vec<(i32, i32)> {
    fn edges(elements: &[i32]) -> impl Iterator<Item = (i32, i32)> + '_ {
        if let [first, rest @ ..] = elements {
            rest.iter()
                .take_while(move |&&i| i <= *first + 3)
                .map(move |&i| (*first, i))
        } else {
            unreachable!()
        }
    }
    format_vec(input);
    input
        .iter()
        .enumerate()
        .map(|(idx, _)| edges(&input[idx..]))
        .flatten()
        .collect::<Vec<_>>()
}

fn walk_graph(edges: &[(i32, i32)]) -> usize {
    let g = {
        let mut output = MultiMap::new();
        for &(src, dest) in edges {
            output.insert(src, dest)
        }
        output
    };
    let mut m = HashMap::new();
    fn walk(src: i32, graph: &MultiMap<i32, i32>, memo: &mut HashMap<i32, usize>) -> usize {
        let mut output = 0;
        if let Some(destinations) = graph.get_vec(&src) {
            for dst in destinations {
                output += if let Some(&val) = memo.get(dst) {
                    val
                } else {
                    walk(*dst, graph, memo)
                }
            }
            if let Some(_) = memo.insert(src, output) {
                panic!("Unexpectedly duplicated node {} while walking", src);
            }
            output
        } else {
            1
        }
    }
    walk(0, &g, &mut m)
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
    fn can_build_graph_1() {
        let mut input = Vec::from(EXAMPLE_1);
        dbg!(build_graph(&mut input));
    }
    #[test]
    fn can_walk_graph_1() {
        let mut input = Vec::from(EXAMPLE_1);
        let edges = build_graph(&mut input);
        assert_eq!(8, walk_graph(&edges));
    }
    #[test]
    fn can_walk_graph_2() {
        let mut input = Vec::from(EXAMPLE_2);
        let edges = build_graph(&mut input);
        assert_eq!(19208, walk_graph(&edges));
    }
}
