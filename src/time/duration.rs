use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Duration {
    Day(u64),
    Week(u64),
    Month(u64),
    Year(u64),
}

impl FromStr for Duration {
    // TODO: this is not good error
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let num = &s[..s.len() - 1];
        let num: u64 = num.parse().ok().ok_or(())?;

        match s.chars().last() {
            Some('d') => Ok(Self::Day(num)),
            Some('w') => Ok(Self::Week(num)),
            Some('m') => Ok(Self::Month(num)),
            Some('y') => Ok(Self::Year(num)),
            _ => Err(()),
        }
    }
}
