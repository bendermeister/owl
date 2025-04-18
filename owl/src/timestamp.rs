use chrono::DateTime;
use chrono::Local;
use chrono::prelude::*;
use rusqlite::types::{FromSql, ToSql, ToSqlOutput};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeStamp {
    stamp: i64,
}

impl TimeStamp {
    pub fn now() -> Self {
        Self::new(chrono::Utc::now().timestamp())
    }

    pub fn new(stamp: i64) -> Self {
        Self { stamp }
    }

    pub fn from_ymd_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> Result<Self, anyhow::Error> {
        let time = match NaiveTime::from_hms_opt(hour, minute, 0) {
            Some(time) => time,
            None => return Err(anyhow::anyhow!("timestamp does not exist")),
        };

        let date = match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => date,
            None => return Err(anyhow::anyhow!("timestamp does not exist")),
        };

        let date = match NaiveDateTime::new(date, time)
            .and_local_timezone(Local)
            .earliest()
        {
            Some(date) => date,
            None => return Err(anyhow::anyhow!("timestamp does not exist")),
        };

        Ok(Self::new(date.timestamp()))
    }
}

impl TryFrom<std::time::SystemTime> for TimeStamp {
    type Error = anyhow::Error;

    fn try_from(time: std::time::SystemTime) -> Result<Self, Self::Error> {
        let stamp: i64 = time
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs()
            .try_into()?;
        Ok(Self::new(stamp))
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

impl ToSql for TimeStamp {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.stamp.to_sql()
    }
}

impl FromSql for TimeStamp {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let stamp = <i64 as FromSql>::column_result(value)?;
        Ok(Self::new(stamp))
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

        let stamp = TimeStamp::from_ymd_hms(
            year.parse()?,
            month.parse()?,
            day.parse()?,
            hour.parse()?,
            minute.parse()?,
        )?;

        Ok(stamp)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let stamp = TimeStamp::from_ymd_hms(2024, 02, 03, 12, 13).unwrap();

        let got = stamp.to_string();
        let expected = "<2024-02-03 12:13>";
        assert_eq!(expected, got);
    }

    #[test]
    fn test_from_str() {
        let expected = TimeStamp::from_ymd_hms(2024, 02, 03, 12, 13).unwrap();
        let got = expected.to_string().parse().unwrap();
        assert_eq!(expected, got);

        // TODO: this is not enough testing
        let fail = "asdfasdfsdf";
        assert!(fail.parse::<TimeStamp>().is_err());
    }
}
