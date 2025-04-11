use crate::id::ID;
use crate::timestamp::TimeStamp;
use crate::todo::Todo;
use std::collections::HashMap;
use std::path::Path;

pub fn todo_exists(db: &rusqlite::Connection, todo: &Todo) -> Result<bool, anyhow::Error> {
    let mut stmt = db.prepare("SELECT EXISTS(SELECT 1 FROM todo WHERE id = ? LIMIT 1);")?;
    let exists: bool = stmt.query_row(rusqlite::params![&todo.id], |row| row.get(0))?;
    Ok(exists)
}

pub fn todo_insert(
    db: &rusqlite::Connection,
    todo: &Todo,
    path: &Path,
) -> Result<(), anyhow::Error> {
    let mut stmt = db.prepare(
        "
        INSERT INTO todo
            (id, title, opened, closed, last_change, path)
        VALUES
            (?, ?, ?, ?, ?, ?);
        ",
    )?;

    // TODO: converting the path to a string seems dumb
    let path = path.to_string_lossy();

    stmt.execute(rusqlite::params![
        &todo.id,
        &todo.title,
        &todo.opened,
        &todo.closed,
        TimeStamp::now(),
        &path
    ])?;

    Ok(())
}

pub fn todo_update(db: &rusqlite::Connection, todo: &Todo) -> Result<(), anyhow::Error> {
    let mut stmt = db.prepare(
        "
        UPDATE todo SET
            title = ?,
            opened = ?,
            closed = ?,
            last_change = ?
        WHERE
            id = ?;
        ",
    )?;

    stmt.execute(rusqlite::params![
        &todo.title,
        &todo.opened,
        &todo.closed,
        TimeStamp::now(),
        &todo.id,
    ])?;

    Ok(())
}

pub fn todo_fetch_all(db: &rusqlite::Connection) -> Result<Vec<Todo>, anyhow::Error> {
    let mut stmt = db.prepare("SELECT id, title, opened, closed FROM todo;")?;
    stmt.query([])?
        .and_then(|row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                opened: row.get(2)?,
                closed: row.get(3)?,
            })
        })
        .collect()
}

pub fn todo_path_map(
    db: &rusqlite::Connection,
) -> Result<HashMap<ID<Todo>, (String, TimeStamp)>, anyhow::Error> {
    let mut stmt = db.prepare(
        "
        SELECT id, path, last_change FROM todo;
        ",
    )?;

    let v: Result<Vec<(ID<Todo>, String, TimeStamp)>, anyhow::Error> = stmt
        .query([])?
        .and_then(|row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .collect();

    let v = v?;

    let mut map = HashMap::new();

    for (id, path, stamp) in v.into_iter() {
        map.insert(id, (path, stamp));
    }

    Ok(map)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::id::ID;
    use std::path::PathBuf;

    #[test]
    fn test_todo_exists() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let todo = Todo::new("title".into());

        let exists = todo_exists(&db, &todo).unwrap();
        assert_eq!(false, exists);

        todo_insert(&db, &todo, &PathBuf::new()).unwrap();

        let exists = todo_exists(&db, &todo).unwrap();
        assert_eq!(true, exists);
    }

    #[test]
    fn test_todo_update() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let mut expected = Todo::new("title".into());
        todo_insert(&db, &expected, &PathBuf::new()).unwrap();

        expected.title = "Some other title".into();
        expected.opened = TimeStamp::from_ymd_hms(2020, 1, 2, 12, 13).unwrap();
        expected.closed = Some(TimeStamp::from_ymd_hms(2021, 1, 2, 11, 15).unwrap());
        todo_update(&db, &expected).unwrap();
        let got = todo_fetch_all(&db).unwrap().into_iter().next().unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn test_todo_fetch_all() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let _ = todo_fetch_all(&db).unwrap();

        let mut a = Todo::new("a".into());
        let mut b = Todo::new("b".into());
        let mut c = Todo::new("c".into());

        a.id = ID::new(1);
        b.id = ID::new(2);
        c.id = ID::new(3);

        let mut path = PathBuf::new();
        path.push("hello");
        path.push("world");

        todo_insert(&db, &a, &path).unwrap();
        todo_insert(&db, &b, &path).unwrap();
        todo_insert(&db, &c, &path).unwrap();

        let expected = vec![a, b, c];
        let mut got = todo_fetch_all(&db).unwrap();
        got.sort_by(|a, b| a.title.cmp(&b.title));
        assert_eq!(expected, got);
    }

    #[test]
    fn test_todo_path_map() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let map = todo_path_map(&db).unwrap();
        assert_eq!(0, map.len());

        let todo = Todo::new("hello".into());
        todo_insert(&db, &todo, &PathBuf::new()).unwrap();

        let map = todo_path_map(&db).unwrap();
        assert_eq!(1, map.len());
        assert!(map.get(&todo.id).is_some());
    }
}
