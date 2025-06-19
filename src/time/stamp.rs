use super::ClockTime;
use std::fmt::Display;
use std::str::FromStr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use super::Duration;
use chrono::prelude::*;

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize, PartialOrd, Ord,
)]
pub struct Stamp {
    stamp: i64,
}

impl Display for Stamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt: DateTime<Local> = self.into();

        if dt.hour() == 0 && dt.minute() == 0 {
            write!(f, "{:0>4}-{:0>2}-{:0>2}", dt.year(), dt.month(), dt.day(),)
        } else {
            write!(
                f,
                "{:0>4}-{:0>2}-{:0>2} {:>02}:{:>02}",
                dt.year(),
                dt.month(),
                dt.day(),
                dt.hour(),
                dt.minute(),
            )
        }
    }
}

impl Stamp {
    pub fn new(stamp: i64) -> Self {
        Self { stamp }
    }

    pub fn to_pretty_string(&self) -> String {
        let dt: DateTime<Local> = self.into();

        let months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        if dt.hour() == 0 && dt.minute() == 0 {
            format!(
                "{} {} {} {}",
                dt.weekday(),
                dt.day(),
                months[dt.month0() as usize],
                dt.year()
            )
        } else {
            format!(
                "{} {} {} {} {:>02}:{:>02}",
                dt.weekday(),
                dt.day(),
                months[dt.month0() as usize],
                dt.year(),
                dt.hour(),
                dt.minute(),
            )
        }
    }

    pub fn now() -> Self {
        Self::new(chrono::Local::now().timestamp())
    }

    pub fn to_day(&self) -> Self {
        let dt: chrono::DateTime<Utc> = self.into();
        let dt: chrono::DateTime<Local> = dt.into();

        Self::from_ymd(dt.year(), dt.month(), dt.day()).unwrap()
    }

    pub fn today() -> Self {
        let dt = chrono::Utc::now();
        Self::from_ymd(dt.year(), dt.month(), dt.day()).unwrap()
    }

    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        Self::from_ymd_hm(year, month, day, 0, 0)
    }

    pub fn from_ymd_hm(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Option<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, day)?;
        let time = NaiveTime::from_hms_opt(hour, minute, 0)?;
        let stamp = NaiveDateTime::new(date, time)
            .and_local_timezone(Local)
            .unwrap()
            .timestamp();
        Some(Self::new(stamp))
    }

    pub fn add_duration(&self, d: Duration) -> Self {
        Self::new(self.stamp + d.to_seconds())
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for Stamp {
    fn from(dt: DateTime<Tz>) -> Self {
        Self::new(dt.timestamp())
    }
}

impl From<Stamp> for DateTime<Utc> {
    // TODO: can this realisticly fail?
    fn from(stamp: Stamp) -> Self {
        DateTime::from_timestamp(stamp.stamp, 0).unwrap()
    }
}

fn parse_yyyymmdd(s: &str) -> Option<(i16, i16, i16)> {
    let mut s = s.trim().split("-").map(|s| s.parse::<i16>());

    let year = match s.next() {
        Some(Ok(y)) => y,
        _ => return None,
    };

    let month = match s.next() {
        Some(Ok(y)) => y,
        _ => return None,
    };

    let day = match s.next() {
        Some(Ok(y)) => y,
        _ => return None,
    };

    if s.next().is_some() {
        return None;
    }

    Some((year, month, day))
}

impl From<&Stamp> for DateTime<Utc> {
    // TODO: can this realisticly fail?
    fn from(stamp: &Stamp) -> Self {
        DateTime::from_timestamp(stamp.stamp, 0).unwrap()
    }
}

impl From<&Stamp> for DateTime<Local> {
    fn from(s: &Stamp) -> Self {
        let dt: DateTime<Utc> = s.into();
        dt.into()
    }
}

impl From<Stamp> for DateTime<Local> {
    fn from(s: Stamp) -> Self {
        let dt: DateTime<Utc> = s.into();
        dt.into()
    }
}

impl From<SystemTime> for Stamp {
    fn from(value: SystemTime) -> Self {
        // TODO: can this fail
        Stamp::new(value.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
    }
}

impl FromStr for Stamp {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();

        let base = match s.next() {
            Some("today") => Some(Self::today()),
            Some("tomorrow") => Some(Self::today().add_duration(Duration::days(1))),
            Some("yesterday") => Some(Self::today().add_duration(Duration::days(-1))),
            Some(a) => match parse_yyyymmdd(a) {
                Some((y, m, d)) => Self::from_ymd(y as i32, m as u32, d as u32),
                None => return Err(anyhow::anyhow!("could not parse date")),
            },
            None => return Err(anyhow::anyhow!("cannot parse empty string into time stamp")),
        };

        let mut base = match base {
            Some(base) => base,
            None => return Err(anyhow::anyhow!("could not parse date")),
        };

        if let Some(time) = s.next() {
            let time = time.parse::<ClockTime>()?;
            base = base.add_duration(time.into());
        }

        if s.next().is_some() {
            return Err(anyhow::anyhow!("could not parse date"));
        }

        Ok(base)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_stamp_today() {
        let got = "today".parse::<Stamp>().unwrap();
        let expected = Stamp::today();
        assert_eq!(expected, got);

        let got = "today 12:30".parse::<Stamp>().unwrap();
        let expected = Stamp::today()
            .add_duration(Duration::hours(12))
            .add_duration(Duration::minutes(30));
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_stamp_yesterday() {
        let got = "yesterday".parse::<Stamp>().unwrap();
        let expected = Stamp::today().add_duration(Duration::days(-1));
        assert_eq!(expected, got);

        let got = "yesterday 12:30".parse::<Stamp>().unwrap();
        let expected = Stamp::today()
            .add_duration(Duration::days(-1))
            .add_duration(Duration::hours(12))
            .add_duration(Duration::minutes(30));
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_stamp_tomorrow() {
        let got = "tomorrow".parse::<Stamp>().unwrap();
        let expected = Stamp::today().add_duration(Duration::days(1));
        assert_eq!(expected, got);

        let got = "tomorrow 12:30".parse::<Stamp>().unwrap();
        let expected = Stamp::today()
            .add_duration(Duration::days(1))
            .add_duration(Duration::hours(12))
            .add_duration(Duration::minutes(30));
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_stamp_date() {
        let got = "2020-01-20".parse::<Stamp>().unwrap();
        let expected = Stamp::from_ymd(2020, 1, 20).unwrap();
        assert_eq!(expected, got);

        let got = "2020-01-20 12:30".parse::<Stamp>().unwrap();
        let expected = Stamp::from_ymd(2020, 1, 20)
            .unwrap()
            .add_duration(Duration::hours(12))
            .add_duration(Duration::minutes(30));
        assert_eq!(expected, got);
    }
}
