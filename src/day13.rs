pub fn part1(s: &str) -> i32 {
    let (current, ids) = parse(s);
    let (id, departure) = ids
        .map(|(_, i)| (i, i.next_departure(current)))
        .min_by_key(|(_, t)| t.0)
        .unwrap();
    id.0 * (departure.0 - current.0)
}
pub fn part2(s: &str) -> i32 {
    let ids = parse(s)
        .1
        .map(|(offset, BusId(id))| Equation { divisor: (offset as i32) % id, modulus: id });
    Equation::solve_set(ids)
}

/*
   0: 13,
   7: 37,
   13: 401
   27: 17
   32: 19
   36: 23
   42: 29
   44: 613
   85: 41

   (x + k) % v = 0
   x % v + k % v = 0

   x == ki' (mod vi); ki' = ki % vi


   ----------------------

   0: 7
   1: 13
   4: 59
   6: 31
   7: 19

   x == 0 (mod 7)
   x == 1 (mod 13)
     (2) * 7 + (-1) * 13 = 1
     x = 0 * -1 * 13 + 1 * 2 * 7
       = 14

   add x == 14 (mod 7*13→91)

   x == 7 (mod 19)
   x == 6 (mod 31)


   ==> 1068781
*/

fn bezout_identity(a: i32, b: i32) -> (i32, i32) {
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut t = (0, 1);
    fn mutate(x: &mut (i32, i32), quotient: i32) {
        *x = (x.1, x.0 - quotient * x.1);
    }
    loop {
        if r.1 == 0 {
            return (s.0, t.0)
        }
        let q = r.0 / r.1;
        mutate(&mut r, q);
        mutate(&mut s, q);
        mutate(&mut t, q);
    }
}

#[derive(Clone, Copy)]
struct Equation {
    divisor: i32,
    modulus: i32,
}
impl Equation {
    fn new(divisor: i32, modulus: i32) -> Equation {
        Equation {
            divisor: if divisor < 0 {
                divisor + modulus
            } else {
                divisor
            },
            modulus,
        }
    }
    fn reduce(u: &Equation, v: &Equation) -> Equation {
        let &Equation {
            divisor: a1,
            modulus: n1,
        } = u;
        let &Equation {
            divisor: a2,
            modulus: n2,
        } = v;
        let (m1, m2) = bezout_identity(n1, n2);
        debug_assert!(m1 * n1 + m2 * n2 == 1);
        let x = a1 * m2 * n2 + a2 * m1 * n1;
        Equation::new(x, n1 * n2)
    }
    fn solve_set(set: impl Iterator<Item = Equation>) -> i32 {

        set.fold(None, |prev, cur| {
            if let Some(prev) = prev {
                Some(Equation::reduce(&cur, &prev))
            } else {
                Some(cur.clone())
            }
        }).unwrap().divisor
    }
    fn is_satisfied(&self, x: i32) -> bool {
        x % self.modulus == self.divisor
    }
}
impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x ≡ {} (mod {})", self.divisor, self.modulus)
    }
}
impl std::fmt::Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x ≡ {} (mod {})", self.divisor, self.modulus)
    }
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
    fn gets_bezout() {
        fn assert_identity(n1: i32, n2: i32) {
            let (m1, m2) = bezout_identity(n1, n2);
            assert_eq!(1, m1 * n1 + m2 * n2)
        }
        assert_identity(5, 12);
        assert_identity(3, 4);
    }

    #[test]
    fn finds_wikipedia_example() {
        let u1 = Equation {
            divisor: 0,
            modulus: 3,
        };
        let u2 = Equation {
            divisor: 3,
            modulus: 4,
        };
        let u3 = Equation {
            divisor: 4,
            modulus: 5,
        };
        let u4 = dbg!(Equation::reduce(&u1, &u2));
        let u5 = dbg!(Equation::reduce(&u3, &u4));
        assert!(u1.is_satisfied(u4.divisor));
        assert!(u2.is_satisfied(u4.divisor));
        assert!(u3.is_satisfied(u5.divisor));
        assert_eq!(39, u5.divisor)
    }
    #[test]
    fn solves_wikipedia_example() {
        let eqs = [
            Equation {
                divisor: 0,
                modulus: 3,
            },
            Equation {
                divisor: 3,
                modulus: 4,
            },
            Equation {
                divisor: 4,
                modulus: 5,
            },
        ];
        assert_eq!(39, Equation::solve_set(eqs.iter().copied()))
    }

    #[test]
    fn finds_example_2() {
        assert_eq!(1068781, part2(EXAMPLE))
    }
}
