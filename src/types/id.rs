use crate::types::time_stamp::TimeStamp;
use rusqlite::types::{FromSql, ToSql, ToSqlOutput, Value};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Id {
    id: i64,
}

impl Id {
    pub fn generate() -> Self {
        Self::new(TimeStamp::now().into())
    }

    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl From<Id> for i64 {
    fn from(value: Id) -> Self {
        value.id
    }
}

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.id)
    }
}

impl FromStr for Id {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = &s[1..s.len() - 1];
        let i: i64 = s.parse()?;
        Ok(i.into())
    }
}

impl ToSql for Id {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(self.id)))
    }
}

impl FromSql for Id {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(<i64 as FromSql>::column_result(value)?.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let id = Id::new(69);
        let expected = "<69>";
        let got = id.to_string();
        assert_eq!(got, expected);
    }

    #[test]
    fn test_from_string() {
        let got: Id = "<69>".parse().unwrap();
        let expected = Id::new(69);
        assert_eq!(expected, got);
    }
}
