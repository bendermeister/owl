use rusqlite::types::{FromSql, ToSql, ToSqlOutput};
use std::fmt::Display;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ID<T> {
    id: i64,
    marker: PhantomData<T>,
}

impl<T> Clone for ID<T> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<T> Copy for ID<T> {}

impl<T> ID<T> {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            marker: PhantomData {},
        }
    }

    pub fn generate() -> Self {
        Self::new(chrono::Utc::now().timestamp())
    }
}

impl<T> FromSql for ID<T> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let id = <i64 as FromSql>::column_result(value)?;
        Ok(Self::new(id))
    }
}

impl<T> ToSql for ID<T> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.id.to_sql()
    }
}

impl<T> Display for ID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}
