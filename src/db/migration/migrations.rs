use super::Migration;

pub struct Migration0001 {}

impl Migration for Migration0001 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "CREATE TABLE files (
                id              INTEGER NOT NULL UNIQUE,
                path            TEXT NOT NULL UNIQUE,
                last_touched    INTEGER NOT NULL,

                PRIMARY KEY(id)
            );",
            rusqlite::params![],
        )?;

        db.execute(
            "CREATE TABLE terms (
                id INTEGER NOT NULL UNIQUE,
                term TEXT NOT NULL UNIQUE,
                PRIMARY KEY(id)
            );",
            rusqlite::params![],
        )?;

        db.execute(
            "CREATE TABLE term_frequencies (
                term    INTEGER NOT NULL,
                file    INTEGER NOT NULL,
                tf      REAL    NOT NULL,

                FOREIGN KEY(term) REFERENCES terms(id),
                FOREIGN KEY(file) REFERENCES files(id)
            );",
            rusqlite::params![],
        )?;

        db.execute(
            "CREATE TABLE todo (
                file        INTEGER NOT NULL,
                title       TEXT NOT NULL,
                deadline    INTEGER,
                scheduled   INTEGER,

                FOREIGN KEY(file) REFERENCES file(id)
            );",
            rusqlite::params![],
        )?;

        Ok(())
    }
}
