use chrono::prelude::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use rusqlite::types::{FromSql, ToSql, ToSqlOutput, Value};
use std::str::FromStr;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeStamp {
    stamp: i64,
}

impl TimeStamp {
    /// Create a new TimeStamp
    pub fn new(stamp: i64) -> Self {
        Self { stamp }
    }

    /// Creates a new Timestamp from the current time
    pub fn now() -> Self {
        Self::new(chrono::Local::now().timestamp())
    }

    /// Creates a new TimeStamp from a given date
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, anyhow::Error> {
        Self::from_ymd_hm(year, month, day, 0, 0)
    }

    pub fn from_ymd_hm(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> Result<Self, anyhow::Error> {

        let date = match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => date,
            None => return Err(anyhow::anyhow!("provided date is not a date")),
        };

        let time = match NaiveTime::from_hms_opt(hour, minute, 0) {
            Some(time) => time,
            None => return Err(anyhow::anyhow!("provided time is not a time")),
        };

        let datetime = NaiveDateTime::new(date, time);
        Ok(datetime.and_local_timezone(chrono::Local).unwrap().into())
    }
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: the unwrap here is not good
        let date: DateTime<Local> = match DateTime::from_timestamp(self.stamp, 0) {
            Some(date) => date,
            None => return Err(std::fmt::Error {}), // TODO: I don't understand this line
        }
        .into();

        write!(
            f,
            "<{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}>",
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute()
        )?;

        Ok(())
    }
}

impl FromStr for TimeStamp {
    type Err = anyhow::Error;

    // TODO: this function seems retarded
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = &s[1..s.len() - 1];
        let mut parts = s.split_whitespace();

        let err = anyhow::anyhow!("could not parse timestamp");

        let date = match parts.next() {
            Some(date) => date,
            None => return Err(err),
        };

        let time = match parts.next() {
            Some(time) => time,
            None => return Err(err),
        };

        if parts.next().is_some() {
            return Err(err);
        }

        let mut date = date.split("-");
        let mut time = time.split(":");

        let year = match date.next() {
            Some(year) => year,
            None => return Err(err),
        };

        let month = match date.next() {
            Some(month) => month,
            None => return Err(err),
        };

        let day = match date.next() {
            Some(day) => day,
            None => return Err(err),
        };

        let hour = match time.next() {
            Some(hour) => hour,
            None => return Err(err),
        };

        let minute = match time.next() {
            Some(minute) => minute,
            None => return Err(err),
        };

        if time.next().is_some() || date.next().is_some() {
            return Err(err);
        }

        let stamp = TimeStamp::from_ymd_hm(
            year.parse()?,
            month.parse()?,
            day.parse()?,
            hour.parse()?,
            minute.parse()?,
        )?;

        Ok(stamp)
    }
}

impl<Tz> From<DateTime<Tz>> for TimeStamp
where
    Tz: chrono::TimeZone,
{
    fn from(value: DateTime<Tz>) -> Self {
        Self::new(value.timestamp())
    }
}

impl From<TimeStamp> for i64 {
    fn from(value: TimeStamp) -> Self {
        value.stamp
    }
}

impl From<TimeStamp> for DateTime<chrono::Local> {
    fn from(value: TimeStamp) -> Self {
        DateTime::from_timestamp(value.stamp, 0).unwrap().into()
    }
}

impl From<i64> for TimeStamp {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl FromSql for TimeStamp {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let stamp: i64 = <i64 as FromSql>::column_result(value)?;
        Ok(Self::new(stamp))
    }
}

impl ToSql for TimeStamp {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(self.stamp)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let stamp = TimeStamp::from_ymd_hm(2024, 02, 03, 12, 13).unwrap();

        let got = stamp.to_string();
        let expected = "<2024-02-03 12:13>";
        assert_eq!(expected, got);
    }

    #[test]
    fn test_from_str() {
        let expected = TimeStamp::from_ymd_hm(2024, 02, 03, 12, 13).unwrap();
        let got = expected.to_string().parse().unwrap();
        assert_eq!(expected, got);

        // TODO: this is not enough testing
        let fail = "asdfasdfsdf";
        assert!(fail.parse::<TimeStamp>().is_err());
    }
}
