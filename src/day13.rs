pub fn part1(s: &str) -> i32 {
    let (current, ids) = parse(s);
    let (id, departure) = ids
        .map(|(_, i)| (i, i.next_departure(current)))
        .min_by_key(|(_, t)| t.0)
        .unwrap();
    id.0 * (departure.0 - current.0)
}
pub fn part2(s: &str) -> i32 {
    let ids = parse(s).1.collect::<Vec<_>>();
    let (_, BusId(first)) = ids[0];
    std::iter::successors(Some(0), |&i| Some(i + first))
        .filter(|&t| {
            ids.iter()
                .all(|&(idx, id)| Timestamp(t).is_valid_reference(idx, id))
        })
        .next()
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Timestamp(i32);
#[derive(Debug, Clone, Copy, PartialEq)]
struct BusId(i32);

fn parse(s: &str) -> (Timestamp, impl Iterator<Item = (usize, BusId)> + '_) {
    let mut lines = s.lines();
    let timestamp = Timestamp(lines.next().unwrap().parse::<i32>().unwrap());
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, c)| "x" != *c)
        .map(move |(idx, c)| (idx, BusId(c.parse().unwrap())));
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
impl Timestamp {
    fn is_valid_reference(&self, index: usize, id: BusId) -> bool {
        let offset = self.0 + index as i32;
        offset % id.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";
    #[test]
    fn can_parse() {
        let (t, i) = parse(EXAMPLE);
        assert_eq!(t, Timestamp(939));
        assert_eq!(
            vec![
                (0, BusId(7)),
                (1, BusId(13)),
                (4, BusId(59)),
                (6, BusId(31)),
                (7, BusId(19))
            ],
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

    #[test]
    fn example_is_valid() {
        let fellas = [(0usize, BusId(7)), (1, BusId(13)), (4, BusId(59))];
        let reference = Timestamp(1068781);
        assert!(fellas
            .iter()
            .all(|&(idx, id)| reference.is_valid_reference(idx, id)))
    }

    #[test]
    fn finds_valid_example() {
        let ids = parse(EXAMPLE).1.collect::<Vec<_>>();
        let (_, BusId(first)) = ids[0];
        assert_eq!(
            std::iter::successors(Some(0), |&i| Some(i + first))
                .filter(|&t| {
                    ids.iter()
                        .all(|&(idx, id)| Timestamp(t).is_valid_reference(idx, id))
                })
                .next()
                .unwrap(),
            1068781
        )
    }
}
