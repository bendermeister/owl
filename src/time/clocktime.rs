use std::{fmt::Display, str::FromStr};

use crate::error::Error;

use super::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClockTime {
    hours: i64,
    minutes: i64,
}

impl ClockTime {
    pub fn from_hm(hours: i64, minutes: i64) -> Option<Self> {
        if !(0..=23).contains(&hours) {
            return None;
        }
        if !(0..=59).contains(&minutes) {
            return None;
        }
        Some(Self { hours, minutes })
    }

    pub fn hours(&self) -> i64 {
        self.hours
    }

    pub fn minutes(&self) -> i64 {
        self.minutes
    }
}

impl From<Duration> for ClockTime {
    fn from(duration: Duration) -> Self {
        let seconds = duration.to_seconds();
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let minutes = minutes % 60;

        Self { hours, minutes }
    }
}

impl Display for ClockTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.minutes == 0 && self.hours == 0 {
            write!(f, "     ")
        } else {
            write!(f, "{:>02}:{:>02}", self.hours, self.minutes)
        }
    }
}

impl FromStr for ClockTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split(":").map(|p| p.parse::<i64>());

        let hours = match s.next() {
            Some(Ok(v)) => v,
            _ => return Err(Error::ParsingError(None)),
        };

        let minutes = match s.next() {
            Some(Ok(v)) => v,
            _ => return Err(Error::ParsingError(None)),
        };

        match Self::from_hm(hours, minutes) {
            Some(v) => Ok(v),
            _ => Err(Error::ParsingError(None)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let got = "12:30".parse::<ClockTime>().unwrap();
        let expected = ClockTime::from_hm(12, 30).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_fail() {
        let got = "-1:30".parse::<ClockTime>();
        assert!(got.is_err());

        let got = "24:30".parse::<ClockTime>();
        assert!(got.is_err());

        let got = "12:-1".parse::<ClockTime>();
        assert!(got.is_err());

        let got = "12:60".parse::<ClockTime>();
        assert!(got.is_err());

        let got = "aaa".parse::<ClockTime>();
        assert!(got.is_err());

        let got = "1260".parse::<ClockTime>();
        assert!(got.is_err());

        let got = "".parse::<ClockTime>();
        assert!(got.is_err());
    }
}
