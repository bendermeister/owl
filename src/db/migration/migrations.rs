use super::Migration;

pub struct Migration0001 {}

impl Migration for Migration0001 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "CREATE TABLE entry (
                id              INTEGER NOT NULL UNIQUE,
                title           TEXT    NOT NULL,
                description     TEXT    NOT NULL,
                opened          INTEGER NOT NULL,
                path            TEXT    NOT NULL,
                last_touched    INTEGER NOT NULL,

                PRIMARY KEY (id)
            );",
            rusqlite::params![],
        )?;

        db.execute(
            "CREATE TABLE todo (
                id          INTEGER NOT NULL UNIQUE,
                body        TEXT    NOT NULL,
                opened      INTEGER NOT NULL,
                closed      INTEGER,
                scheduled   INTEGER,
                deadline    INTEGER,

                FOREIGN KEY(id) REFERENCES entry(id)
            );",
            rusqlite::params![],
        )?;

        db.execute(
            "
            CREATE TABLE tag (
                id      INTEGER NOT NULL UNIQUE,
                name    TEXT    NOT NULL,
                PRIMARY KEY(id)
            );
            ",
            rusqlite::params![],
        )?;

        db.execute(
            "
            CREATE TABLE taggings (
                tag INTEGER NOT NULL,
                entry INTEGER NOT NULL,

                FOREIGN KEY(tag) REFERENCES tag(id),
                FOREIGN KEY(entry) REFERENCES entry(id)
            );
            ",
            rusqlite::params![],
        )?;

        Ok(())
    }
}
