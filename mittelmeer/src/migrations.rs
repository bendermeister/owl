use super::Migration;

pub struct Migration000 {}

impl Migration for Migration000 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "
            CREATE TABLE entry (
                id          INTEGER NOT NULL UNIQUE,
                title       TEXT NOT NULL,
                last_update INTEGER NOT NULL,

                PRIMARY KEY(id)
            );
            ",
            rusqlite::params![],
        )?;

        db.execute(
            "
            CREATE TABLE todos (
                entry       INTEGER NOT NULL UNIQUE,
                opened      INTEGER,
                closed      INTEGER NOT NULL,

                FOREIGN KEY(entry) REFERENCES entry(id)
            );
            ",
            rusqlite::params![],
        )?;

        Ok(())
    }
}
