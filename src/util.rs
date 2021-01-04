pub mod circular_buffer;

pub fn split_tuple_2<'source, 'pattern>(
    s: &'source str,
    pat: &'pattern str,
) -> Option<(&'source str, &'source str)> {
    let mut tokens = s.splitn(2, pat);
    if let (Some(a), Some(b)) = (tokens.next(), tokens.next()) {
        if tokens.next().is_some() {
            panic!("splitn unexpectedly returned too many values")
        }
        Some((a, b))
    } else {
        None
    }
}

pub fn tuples<T>(items: &[T]) -> impl Iterator<Item = (&T, &T)> {
    let len = items.len();
    (0..len).map(move |i| {
        (i + 1 ..len).map(move |j| (&items[i], &items[j]))
    }).flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_tuple_split() {
        assert_eq!(
            ("hello", "this is dog"),
            split_tuple_2("hello this is dog", " ").unwrap()
        )
    }
    #[test]
    fn tuple_split_fails_gracefully() {
        assert_eq!(None, split_tuple_2("nooooope", "oops"))
    }

    #[test]
    fn can_tuple() {
        let items = [1i32, 2, 3];
        let expected = vec![(&1i32, &2i32), (&1, &3), (&2, &3)];
        assert_eq!(expected, tuples(&items).collect::<Vec<_>>())
    }
}
