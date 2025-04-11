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

pub struct Migration001 {}

impl Migration for Migration001 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "
            CREATE TABLE tag (
                id      INTEGER NOT NULL UNIQUE,
                name    TEXT NOT NULL,

                PRIMARY KEY(id)
            );
            ",
            [],
        )?;

        db.execute(
            "
            CREATE TABLE tag_mapping (
                todo INTEGER NOT NULL,
                tag  INTEGER NOT NULL,
                
                FOREIGN KEY(todo) REFERENCES todo(id),
                FOREIGN KEY(tag) REFERENCES tag(id)
            );
            ",
            [],
        )?;

        Ok(())
    }
}

pub struct Migration002 {}
impl Migration for Migration002 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "
            ALTER TABLE todo ADD COLUMN deadline INTEGER;
            ",
            [],
        )?;
        db.execute(
            "
            ALTER TABLE todo ADD COLUMN scheduled INTEGER;
            ",
            [],
        )?;
        Ok(())
    }
}
