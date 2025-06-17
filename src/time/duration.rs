use std::str::FromStr;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, PartialOrd, Ord,
)]
pub struct Duration {
    seconds: i64,
}

impl FromStr for Duration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().chars().last() {
            Some('d') => Ok(Self::days(s.strip_suffix("d").unwrap().trim().parse()?)),
            Some('w') => Ok(Self::weeks(s.strip_suffix("w").unwrap().trim().parse()?)),
            Some('m') => Ok(Self::months(s.strip_suffix("m").unwrap().trim().parse()?)),
            Some('y') => Ok(Self::years(s.strip_suffix("y").unwrap().trim().parse()?)),
            _ => Err(anyhow::anyhow!("could not parse '{:?}' as duration", s)),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_day() {
        let got: Duration = "7d".parse().unwrap();
        assert_eq!(Duration::days(7), got);

        let got: Duration = "-7d".parse().unwrap();
        assert_eq!(Duration::days(-7), got);

        let got: Duration = "+7d".parse().unwrap();
        assert_eq!(Duration::days(7), got);

        let got: Duration = "7 d".parse().unwrap();
        assert_eq!(Duration::days(7), got);

        let got: Duration = "-7 d".parse().unwrap();
        assert_eq!(Duration::days(-7), got);

        let got: Duration = "+7 d".parse().unwrap();
        assert_eq!(Duration::days(7), got);
    }

    #[test]
    fn test_parse_week() {
        let got: Duration = "7w".parse().unwrap();
        assert_eq!(Duration::weeks(7), got);

        let got: Duration = "-7w".parse().unwrap();
        assert_eq!(Duration::weeks(-7), got);

        let got: Duration = "+7w".parse().unwrap();
        assert_eq!(Duration::weeks(7), got);

        let got: Duration = "7 w".parse().unwrap();
        assert_eq!(Duration::weeks(7), got);

        let got: Duration = "-7 w".parse().unwrap();
        assert_eq!(Duration::weeks(-7), got);

        let got: Duration = "+7 w".parse().unwrap();
        assert_eq!(Duration::weeks(7), got);
    }

    #[test]
    fn test_parse_month() {
        let got: Duration = "7m".parse().unwrap();
        assert_eq!(Duration::months(7), got);

        let got: Duration = "-7m".parse().unwrap();
        assert_eq!(Duration::months(-7), got);

        let got: Duration = "+7m".parse().unwrap();
        assert_eq!(Duration::months(7), got);

        let got: Duration = "7 m".parse().unwrap();
        assert_eq!(Duration::months(7), got);

        let got: Duration = "-7 m".parse().unwrap();
        assert_eq!(Duration::months(-7), got);

        let got: Duration = "+7 m".parse().unwrap();
        assert_eq!(Duration::months(7), got);
    }

    #[test]
    fn test_parse_year() {
        let got: Duration = "7y".parse().unwrap();
        assert_eq!(Duration::years(7), got);

        let got: Duration = "-7y".parse().unwrap();
        assert_eq!(Duration::years(-7), got);

        let got: Duration = "+7y".parse().unwrap();
        assert_eq!(Duration::years(7), got);

        let got: Duration = "7 y".parse().unwrap();
        assert_eq!(Duration::years(7), got);

        let got: Duration = "-7 y".parse().unwrap();
        assert_eq!(Duration::years(-7), got);

        let got: Duration = "+7 y".parse().unwrap();
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
