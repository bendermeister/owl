use crate::error::Error;

use super::ClockTime;
use std::ops::Add;
use std::str::FromStr;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, PartialOrd, Ord,
)]
pub struct Duration {
    seconds: i64,
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suffix = match s.trim().chars().last() {
            Some('s') => 's',
            Some('m') => 'm',
            Some('h') => 'h',
            Some('D') => 'D',
            Some('W') => 'W',
            Some('M') => 'M',
            Some('Y') => 'Y',
            _ => return Err(Error::FailedToParse(0)),
        };

        let time: i64 = match s.trim().strip_suffix(suffix).unwrap().parse() {
            Ok(time) => time,
            Err(_) => return Err(Error::FailedToParse(0)),
        };

        match suffix {
            's' => Ok(Self::seconds(time)),
            'm' => Ok(Self::minutes(time)),
            'h' => Ok(Self::hours(time)),
            'D' => Ok(Self::days(time)),
            'W' => Ok(Self::weeks(time)),
            'M' => Ok(Self::months(time)),
            'Y' => Ok(Self::years(time)),
            _ => unreachable!(),
        }
    }
}

impl Duration {
    pub fn seconds(seconds: i64) -> Self {
        Self { seconds }
    }

    pub fn minutes(minutes: i64) -> Self {
        Self::seconds(minutes * 60)
    }

    pub fn hours(hours: i64) -> Self {
        Self::minutes(hours * 60)
    }

    pub fn days(days: i64) -> Self {
        Self::hours(days * 24)
    }

    pub fn weeks(weeks: i64) -> Self {
        Self::days(weeks * 7)
    }

    pub fn months(months: i64) -> Self {
        Self::days(months * 31)
    }

    pub fn years(years: i64) -> Self {
        Self::days(years * 366)
    }

    pub fn to_seconds(&self) -> i64 {
        self.seconds
    }

    pub fn is_positive(&self) -> bool {
        self.seconds > 0
    }

    pub fn is_zero(&self) -> bool {
        self.seconds == 0
    }

    pub fn is_negative(&self) -> bool {
        self.seconds < 0
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            seconds: self.seconds + rhs.seconds,
        }
    }
}

impl From<ClockTime> for Duration {
    fn from(time: ClockTime) -> Self {
        let hours = Duration::hours(time.hours());
        let minutes = Duration::minutes(time.minutes());
        hours + minutes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_day() {
        let got: Duration = "7D".parse().unwrap();
        assert_eq!(Duration::days(7), got);

        let got: Duration = "-7D".parse().unwrap();
        assert_eq!(Duration::days(-7), got);

        let got: Duration = "+7D".parse().unwrap();
        assert_eq!(Duration::days(7), got);

        let got: Duration = "7 D".parse().unwrap();
        assert_eq!(Duration::days(7), got);

        let got: Duration = "-7 D".parse().unwrap();
        assert_eq!(Duration::days(-7), got);

        let got: Duration = "+7 D".parse().unwrap();
        assert_eq!(Duration::days(7), got);
    }

    #[test]
    fn test_parse_week() {
        let got: Duration = "7W".parse().unwrap();
        assert_eq!(Duration::weeks(7), got);

        let got: Duration = "-7W".parse().unwrap();
        assert_eq!(Duration::weeks(-7), got);

        let got: Duration = "+7W".parse().unwrap();
        assert_eq!(Duration::weeks(7), got);

        let got: Duration = "7 W".parse().unwrap();
        assert_eq!(Duration::weeks(7), got);

        let got: Duration = "-7 W".parse().unwrap();
        assert_eq!(Duration::weeks(-7), got);

        let got: Duration = "+7 W".parse().unwrap();
        assert_eq!(Duration::weeks(7), got);
    }

    #[test]
    fn test_parse_month() {
        let got: Duration = "7M".parse().unwrap();
        assert_eq!(Duration::months(7), got);

        let got: Duration = "-7M".parse().unwrap();
        assert_eq!(Duration::months(-7), got);

        let got: Duration = "+7M".parse().unwrap();
        assert_eq!(Duration::months(7), got);

        let got: Duration = "7 M".parse().unwrap();
        assert_eq!(Duration::months(7), got);

        let got: Duration = "-7 M".parse().unwrap();
        assert_eq!(Duration::months(-7), got);

        let got: Duration = "+7 M".parse().unwrap();
        assert_eq!(Duration::months(7), got);
    }

    #[test]
    fn test_parse_year() {
        let got: Duration = "7Y".parse().unwrap();
        assert_eq!(Duration::years(7), got);

        let got: Duration = "-7Y".parse().unwrap();
        assert_eq!(Duration::years(-7), got);

        let got: Duration = "+7Y".parse().unwrap();
        assert_eq!(Duration::years(7), got);

        let got: Duration = "7 Y".parse().unwrap();
        assert_eq!(Duration::years(7), got);

        let got: Duration = "-7 Y".parse().unwrap();
        assert_eq!(Duration::years(-7), got);

        let got: Duration = "+7 Y".parse().unwrap();
        assert_eq!(Duration::years(7), got);
    }

    #[test]
    fn test_parse_fail() {
        let got = "7am".parse::<Duration>();
        assert!(got.is_err());

        let got = "7g".parse::<Duration>();
        assert!(got.is_err());

        let got = "+".parse::<Duration>();
        assert!(got.is_err());

        let got = "+m".parse::<Duration>();
        assert!(got.is_err());
    }

    #[test]
    fn test_is_positive() {
        let d = Duration::days(1);
        assert!(d.is_positive());
        assert!(!d.is_negative());
        assert!(!d.is_zero());

        let d = Duration::weeks(1);
        assert!(d.is_positive());
        assert!(!d.is_negative());
        assert!(!d.is_zero());

        let d = Duration::months(1);
        assert!(d.is_positive());
        assert!(!d.is_negative());
        assert!(!d.is_zero());

        let d = Duration::years(1);
        assert!(d.is_positive());
        assert!(!d.is_negative());
        assert!(!d.is_zero());
    }

    #[test]
    fn test_is_negative() {
        let d = Duration::days(-1);
        assert!(d.is_negative());
        assert!(!d.is_positive());
        assert!(!d.is_zero());

        let d = Duration::weeks(-1);
        assert!(d.is_negative());
        assert!(!d.is_positive());
        assert!(!d.is_zero());

        let d = Duration::months(-1);
        assert!(d.is_negative());
        assert!(!d.is_positive());
        assert!(!d.is_zero());

        let d = Duration::years(-1);
        assert!(d.is_negative());
        assert!(!d.is_positive());
        assert!(!d.is_zero());
    }

    #[test]
    fn test_is_zero() {
        let d = Duration::days(0);
        assert!(d.is_zero());
        assert!(!d.is_negative());
        assert!(!d.is_positive());

        let d = Duration::weeks(0);
        assert!(d.is_zero());
        assert!(!d.is_negative());
        assert!(!d.is_positive());

        let d = Duration::months(0);
        assert!(d.is_zero());
        assert!(!d.is_negative());
        assert!(!d.is_positive());

        let d = Duration::years(0);
        assert!(d.is_zero());
        assert!(!d.is_negative());
        assert!(!d.is_positive());
    }
}
