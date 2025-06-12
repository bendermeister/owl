mod migrations;

trait Migration {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error>;
}

struct BaseMigration {}

impl Migration for BaseMigration {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "CREATE TABLE version (version BIGINT NOT NULL);",
            rusqlite::params![],
        )?;
        db.execute(
            "INSERT INTO version (version) VALUES(1);",
            rusqlite::params![],
        )?;
        Ok(())
    }
}

fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(BaseMigration {}),
        Box::new(migrations::Migration0001 {}),
    ]
}

fn get_version(db: &rusqlite::Connection) -> Result<usize, anyhow::Error> {
    let mut stmt = db.prepare(
        "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='version');",
    )?;
    let version_exists: bool = stmt.query_row(rusqlite::params![], |row| row.get(0))?;
    if !version_exists {
        return Ok(0);
    }

    let mut stmt = db.prepare("SELECT version FROM version LIMIT 1;")?;
    let version: usize = stmt.query_row(rusqlite::params![], |row| row.get(0))?;
    Ok(version)
}

pub fn migrate(db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
    let all_migrations = get_migrations();
    let version = get_version(db)?;
    let migrations = &all_migrations[version..];

    for migration in migrations.iter() {
        migration.up(db)?;
    }

    let _ = db.execute(
        "UPDATE version SET version = ?;",
        rusqlite::params![all_migrations.len()],
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_migrate() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        migrate(&db).unwrap();
    }
}
