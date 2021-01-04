pub fn part1(_: &str) {}
pub fn part2(_: &str) {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Timestamp(i32);
#[derive(Debug, Clone, Copy, PartialEq)]
struct BusId(i32);

fn parse(s: &str) -> (Timestamp, impl Iterator<Item = BusId> + '_) {
    let mut lines = s.lines();
    let timestamp = Timestamp(lines.next().unwrap().parse::<i32>().unwrap());
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|c| "x" != *c)
        .map(|s| BusId(s.parse().unwrap()));
    (timestamp, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";
        let (t, i) = parse(EXAMPLE);
        assert_eq!(t, Timestamp(939));
        assert_eq!(vec![BusId(7), BusId(13), BusId(59), BusId(31), BusId(19)], i.collect::<Vec<_>>())
    }
}
