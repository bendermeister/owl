use std::{cmp::Ordering, fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ClockTime {
    pub hour: u8,
    pub minute: u8,
}

impl Display for ClockTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>02}:{:>02}", self.hour, self.minute)
    }
}

impl PartialOrd for ClockTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClockTime {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hour != other.hour {
            self.hour.cmp(&other.hour)
        } else {
            self.minute.cmp(&other.minute)
        }
    }
}

impl ClockTime {
    /// Constructs a new `ClockTime` from `hour` and `minute`
    ///
    /// # Returns
    /// - `None` if `hour` and `minute` do not form a valid `ClockTime`
    /// - `Some(_)` otherwise
    ///
    /// # Example
    /// ```
    /// use owl::time::ClockTime;
    ///
    /// let time = ClockTime::from_hm(20, 51).unwrap();
    ///
    /// assert_eq!(time, ClockTime {hour: 20, minute: 51});
    ///
    /// let time = ClockTime::from_hm(100, 0);
    /// assert!(time.is_none());
    /// ```
    pub fn from_hm(hour: u8, minute: u8) -> Option<Self> {
        if !is_valid_clocktime(hour, minute) {
            return None;
        }
        Some(Self { hour, minute })
    }
}

/// checks whether or not a clocktime is valid
///
/// a valid clocktime is defined as 00:00 - 23:59
///
/// # Returns
/// - `true` if `hour` and `minute` form valid clocktime
/// - `false` otherwise
fn is_valid_clocktime(hour: u8, minute: u8) -> bool {
    if hour > 23 {
        return false;
    }
    if minute > 59 {
        return false;
    }
    true
}

impl FromStr for ClockTime {
    // TODO: this is not good error handling
    type Err = ();

    // TODO: should we support am/pm tomfoolery?

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(":").map(|n| n.parse());

        let hour = match parts.next() {
            Some(Ok(v)) => v,
            _ => return Err(()),
        };

        let minute = match parts.next() {
            Some(Ok(v)) => v,
            _ => return Err(()),
        };

        if parts.next().is_some() {
            return Err(());
        }

        if !is_valid_clocktime(hour, minute) {
            return Err(());
        }

        Ok(ClockTime { hour, minute })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_valid() {
        assert_eq!(
            "12:24".parse::<ClockTime>().unwrap(),
            ClockTime {
                hour: 12,
                minute: 24
            }
        );
        assert_eq!(
            "01:30".parse::<ClockTime>().unwrap(),
            ClockTime {
                hour: 1,
                minute: 30
            }
        );
        assert_eq!(
            "23:59".parse::<ClockTime>().unwrap(),
            ClockTime {
                hour: 23,
                minute: 59
            }
        );
    }

    #[test]
    fn test_parse_invalid() {
        assert!("00::10".parse::<ClockTime>().is_err());
        assert!("00::10".parse::<ClockTime>().is_err());
        assert!("24:00".parse::<ClockTime>().is_err());
        assert!("-23:00".parse::<ClockTime>().is_err());
        assert!("00:60".parse::<ClockTime>().is_err());
    }
}
