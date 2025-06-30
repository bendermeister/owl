use super::Duration;
use chrono::Datelike;
use chrono::NaiveDate;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.year != other.year {
            self.year.cmp(&other.year)
        } else if self.month != other.month {
            self.month.cmp(&other.month)
        } else {
            self.day.cmp(&other.day)
        }
    }
}

const MONTH_LITERALS: [&str; 13] = [
    "", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Date {
    // TODO: doc this
    pub fn from_ymd(year: u16, month: u8, day: u8) -> Option<Self> {
        if !is_date_valid(year, month, day) {
            return None;
        }
        Some(Self { year, month, day })
    }

    pub fn add_duration(&self, duration: Duration) -> Option<Self> {
        let date = self.to_naive_date();

        let date = match duration {
            Duration::Day(d) => date.checked_add_days(chrono::Days::new(d))?,
            Duration::Week(w) => date.checked_add_days(chrono::Days::new(w * 7))?,
            Duration::Month(m) => date.checked_add_months(chrono::Months::new(m as u32))?,
            Duration::Year(y) => date.checked_add_months(chrono::Months::new(y as u32 * 12))?,
        };

        Some(Self::from_naive_date(date))
    }

    pub fn to_naive_date(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year as i32, self.month as u32, self.day as u32).unwrap()
    }

    pub fn from_naive_date(date: NaiveDate) -> Self {
        let year = date.year() as u16;
        let month = date.month() as u8;
        let day = date.day() as u8;

        Self { year, month, day }
    }

    pub fn today() -> Self {
        let today = chrono::Local::now().naive_local().date();
        Self::from_naive_date(today)
    }

    pub fn to_pretty_string(&self) -> String {
        let dt = self.to_naive_date();

        format!(
            "{} {} {} {}",
            dt.weekday(),
            self.day,
            MONTH_LITERALS[self.month as usize],
            self.year
        )
    }
}

fn is_leap_year(year: u16) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    if year % 4 == 0 {
        return true;
    }
    false
}

const MONTH_LENGTHS: [[u8; 13]; 2] = [
    [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

fn is_date_valid(year: u16, month: u8, day: u8) -> bool {
    if year < 1950 {
        return false;
    }
    if month == 0 {
        return false;
    }
    if day == 0 {
        return false;
    }
    if month > 12 {
        return false;
    }
    let i = if is_leap_year(year) { 1 } else { 0 };
    if day > MONTH_LENGTHS[i][month as usize] {
        return false;
    }

    true
}

impl FromStr for Date {
    // TODO: this is not good error
    type Err = ();

    fn from_str(date: &str) -> Result<Self, Self::Err> {
        let mut date = date.trim().split("-");

        let year = date.next().ok_or(())?;
        let month = date.next().ok_or(())?;
        let day = date.next().ok_or(())?;

        let year: u16 = year.parse().ok().ok_or(())?;
        let month: u8 = month.parse().ok().ok_or(())?;
        let day: u8 = day.parse().ok().ok_or(())?;

        if !is_date_valid(year, month, day) {
            return Err(());
        }
        Ok(Date { year, month, day })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_date_valid() {
        assert_eq!(
            "2024-12-01".parse::<Date>().unwrap(),
            Date {
                year: 2024,
                month: 12,
                day: 1,
            }
        );
        assert_eq!(
            "2024-11-30".parse::<Date>().unwrap(),
            Date {
                year: 2024,
                month: 11,
                day: 30,
            }
        );
        assert_eq!(
            "2024-02-29".parse::<Date>().unwrap(),
            Date {
                year: 2024,
                month: 2,
                day: 29,
            }
        );
        assert_eq!(
            "2024-02-28".parse::<Date>().unwrap(),
            Date {
                year: 2024,
                month: 02,
                day: 28,
            }
        );
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!("2025-02-29".parse::<Date>().is_err());
        assert!("2025-01-100".parse::<Date>().is_err());
        assert!("2025-13-29".parse::<Date>().is_err());
        assert!("2025-00-29".parse::<Date>().is_err());
        assert!("2025-01-0".parse::<Date>().is_err());
        assert!("2025-10-33".parse::<Date>().is_err());
    }
}
