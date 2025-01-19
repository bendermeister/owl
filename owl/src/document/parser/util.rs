pub(super) fn chop<'a>(line: &'a str, f: fn(&char) -> bool) -> (&'a str, &'a str) {
    let len = line.chars().take_while(|c| f(c)).count();
    let first = &line[..len];
    let last = &line[len..];
    (first, last)
}

pub(super) fn indent<'a>(line: &'a str) -> (&'a str, &'a str) {
    chop(line, |c| c.is_whitespace())
}

pub(super) fn assert_err<T, E>(expected: E, result: Result<T, E>)
where
    E: std::fmt::Debug + PartialEq + Eq,
    T: std::fmt::Debug,
{
    if let Err(got) = result {
        assert_eq!(expected, got);
    } else {
        panic!(
            "Expected {:?}, got: {:?}",
            Err::<T, _>(expected),
            result
        );
    }
}
