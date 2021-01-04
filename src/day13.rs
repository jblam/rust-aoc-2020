pub fn part1(s: &str) -> i32 {
    let (current, ids) = parse(s);
    let (id, departure) = ids
        .map(|i| (i, i.next_departure(current)))
        .min_by_key(|(_, t)| t.0)
        .unwrap();
    id.0 * (departure.0 - current.0)
}
pub fn part2(_: &str) {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
impl BusId {
    fn next_departure(&self, reference: Timestamp) -> Timestamp {
        let id = self.0;
        let reference = reference.0;
        let candidate = id * (reference / id);
        Timestamp(if candidate < reference {
            candidate + id
        } else {
            candidate
        })
    }
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
        assert_eq!(
            vec![BusId(7), BusId(13), BusId(59), BusId(31), BusId(19)],
            i.collect::<Vec<_>>()
        )
    }
    #[test]
    fn gets_immediate_departure() {
        let current = Timestamp(50);
        let id = BusId(10);
        assert_eq!(current, id.next_departure(current))
    }
    #[test]
    fn gets_next_departure() {
        let current = Timestamp(51);
        let id = BusId(10);
        assert_eq!(Timestamp(60), id.next_departure(current));
    }
}
