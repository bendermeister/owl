use super::Migration;

pub struct Migration000 {}

impl Migration for Migration000 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "
            CREATE TABLE todo (
                id      INTEGER NOT NULL UNIQUE,
                title   TEXT NOT NULL,
                opened  INTEGER,
                closed  INTEGER,

                last_change INTEGER NOT NULL,
                path        INTEGER NOT NULL,

                PRIMARY KEY(id)
            );
            ",
            rusqlite::params![],
        )?;

        Ok(())
    }
}
