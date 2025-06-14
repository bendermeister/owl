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
            "CREATE TABLE todos (
                file        INTEGER NOT NULL,
                title       TEXT NOT NULL,
                deadline    INTEGER,
                scheduled   INTEGER,

                FOREIGN KEY(file) REFERENCES files(id)
            );",
            rusqlite::params![],
        )?;

        Ok(())
    }
}

pub struct Migration0002 {}

impl Migration for Migration0002 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute("DROP TABLE todos", [])?;
        db.execute("DROP TABLE term_frequencies", [])?;
        db.execute("DROP TABLE terms", [])?;
        db.execute("DROP TABLE files", [])?;

        db.execute(
            "CREATE TABLE files (
                id          INTEGER NOT NULL UNIQUE,
                path        TEXT NOT NULL UNIQUE,
                modified    INTEGER NOT NULL,

                PRIMARY KEY(id)
            );",
            rusqlite::params![],
        )?;

        db.execute(
            "CREATE TABLE terms (
                id      INTEGER NOT NULL UNIQUE,
                term    TEXT NOT NULL UNIQUE,
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
            "CREATE TABLE todos (
                file        INTEGER NOT NULL,
                line        INTEGER NOT NULL,
                title       TEXT NOT NULL,
                deadline    INTEGER,
                scheduled   INTEGER,

                FOREIGN KEY(file) REFERENCES files(id)
            );",
            rusqlite::params![],
        )?;

        Ok(())
    }
}
