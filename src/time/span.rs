use std::cmp::Ordering;
use std::str::FromStr;

use super::{ClockTime, Date};

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Span {
    pub date: Date,
    pub start: Option<ClockTime>,
    pub end: Option<ClockTime>,
}

impl FromStr for Span {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let split = s.chars().take_while(|c| !c.is_whitespace()).count();

        let date = &s[..split];
        let rest = &s[split..];

        let date = date.parse()?;
        let mut start = None;
        let mut end = None;

        if !rest.is_empty() {
            let mut rest = rest.split('-');

            if let Some(s) = rest.next() {
                start = Some(s.parse()?);
            }

            if let Some(s) = rest.next() {
                end = Some(s.parse()?);
            }

            if rest.next().is_some() {
                return Err(());
            }
        }

        Ok(Self::new(date, start, end))
    }
}

impl Span {
    /// contructs a new Span instance
    fn new(date: Date, start: Option<ClockTime>, end: Option<ClockTime>) -> Self {
        Self { date, start, end }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.date.cmp(&other.date) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        };

        match (self.start, other.start) {
            (Some(_), None) => return Ordering::Less,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
            _ => (),
        };

        self.start.unwrap().cmp(&other.start.unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let span = "2024-12-13";

        let got: Span = span.parse().unwrap();
        let expected = Span {
            date: Date::from_ymd(2024, 12, 13).unwrap(),
            start: None,
            end: None,
        };

        assert_eq!(expected, got);

        let span = "2024-12-13 12:00";

        let got: Span = span.parse().unwrap();
        let expected = Span {
            date: Date::from_ymd(2024, 12, 13).unwrap(),
            start: Some(ClockTime::from_hm(12, 0).unwrap()),
            end: None,
        };

        assert_eq!(expected, got);

        let span = "2024-12-13 12:00 - 14:30";

        let got: Span = span.parse().unwrap();
        let expected = Span {
            date: Date::from_ymd(2024, 12, 13).unwrap(),
            start: Some(ClockTime::from_hm(12, 0).unwrap()),
            end: Some(ClockTime::from_hm(14, 30).unwrap()),
        };

        assert_eq!(expected, got);
    }
}
