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
}
