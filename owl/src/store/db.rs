use crate::id::ID;
use crate::tag::Tag;
use crate::timestamp::TimeStamp;
use crate::todo::Todo;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

pub fn tag_mapping_clear(db: &rusqlite::Connection, todo: ID<Todo>) -> Result<(), anyhow::Error> {
    let mut stmt = db.prepare("DELETE FROM tag_mapping WHERE todo = ?;")?;
    stmt.execute([todo])?;
    Ok(())
}

pub fn tag_mapping_insert(
    db: &rusqlite::Connection,
    todo: ID<Todo>,
    tag: ID<Tag>,
) -> Result<(), anyhow::Error> {
    let mut stmt = db.prepare(
        "
        INSERT INTO tag_mapping (todo, tag) VALUES (? , ?);
        ",
    )?;
    stmt.execute(rusqlite::params![todo, tag])?;
    Ok(())
}

pub fn todo_insert(
    db: &rusqlite::Connection,
    todo: &Todo,
    path: &Path,
) -> Result<(), anyhow::Error> {
    let mut stmt = db.prepare(
        "
        INSERT INTO todo
            (id, title, opened, closed, deadline, scheduled, last_change, path)
        VALUES
            (?, ?, ?, ?, ?, ?, ?, ?);
        ",
    )?;

    // TODO: converting the path to a string seems dumb
    let path = path.to_string_lossy();

    stmt.execute(rusqlite::params![
        &todo.id,
        &todo.title,
        &todo.opened,
        &todo.closed,
        &todo.deadline,
        &todo.scheduled,
        TimeStamp::now(),
        &path
    ])?;

    Ok(())
}

// TODO: test this
pub fn tag_insert(db: &rusqlite::Connection, tag: &Tag) -> Result<(), anyhow::Error> {
    let mut stmt = db.prepare("INSERT INTO tag (id, name) VALUES (?, ?);")?;
    stmt.execute(rusqlite::params![tag.id, tag.name])?;

    Ok(())
}

pub fn tags_fetch_all(
    db: &rusqlite::Connection,
) -> Result<HashMap<String, ID<Tag>>, anyhow::Error> {
    let mut stmt = db.prepare(
        "
        SELECT id, name FROM tag;
        ",
    )?;

    let tags = stmt
        .query([])?
        .and_then(|row| Ok(Tag::new(row.get(0)?, row.get(1)?)))
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    let mut map = HashMap::new();

    for tag in tags.into_iter() {
        map.insert(tag.name, tag.id);
    }

    Ok(map)
}

pub fn todo_update(db: &rusqlite::Connection, todo: &Todo) -> Result<(), anyhow::Error> {
    let mut stmt = db.prepare(
        "
        UPDATE todo SET
            title = ?,
            opened = ?,
            closed = ?,
            scheduled = ?,
            deadline = ?,
            last_change = ?
        WHERE
            id = ?;
        ",
    )?;

    stmt.execute(rusqlite::params![
        &todo.title,
        &todo.opened,
        &todo.closed,
        &todo.scheduled,
        &todo.deadline,
        TimeStamp::now(),
        &todo.id,
    ])?;

    Ok(())
}

pub fn todo_fetch_all(db: &rusqlite::Connection) -> Result<Vec<Todo>, anyhow::Error> {
    let mut stmt =
        db.prepare("SELECT id, title, opened, closed, scheduled, deadline FROM todo;")?;
    stmt.query([])?
        .and_then(|row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                opened: row.get(2)?,
                closed: row.get(3)?,
                scheduled: row.get(4)?,
                deadline: row.get(5)?,
                tags: HashSet::new(),
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

pub fn tag_mapping_fetch_all(
    db: &rusqlite::Connection,
) -> Result<HashMap<ID<Todo>, HashSet<ID<Tag>>>, anyhow::Error> {
    let mut stmt = db.prepare(
        "
        SELECT tag, todo FROM tag_mapping;
        ",
    )?;

    let rows = stmt
        .query([])?
        .and_then(|row| Ok::<(ID<Tag>, ID<Todo>), anyhow::Error>((row.get(0)?, row.get(1)?)));

    let mut map = HashMap::<ID<Todo>, HashSet<ID<Tag>>>::new();

    for row in rows {
        let (tag, todo) = row?;

        map.entry(todo)
            .or_insert_with(|| HashSet::new())
            .insert(tag);
    }

    Ok(map)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::id::ID;
    use std::path::PathBuf;

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

    #[test]
    fn test_tags_fetch_all() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let _ = tags_fetch_all(&db).unwrap();

        let tag = Tag::new(ID::generate(), "Tag".into());
        tag_insert(&db, &tag).unwrap();

        let tags = tags_fetch_all(&db).unwrap();
        assert_eq!(1, tags.len());
        assert_eq!(tag.id, *tags.get(&tag.name).unwrap());
    }

    #[test]
    fn test_fetch_tag_mapping() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let _ = tag_mapping_fetch_all(&db);

        let tag = Tag::new(ID::generate(), "Tag".into());
        tag_insert(&db, &tag).unwrap();

        let tags = tags_fetch_all(&db).unwrap();
        assert_eq!(1, tags.len());
        assert_eq!(tag.id, *tags.get(&tag.name).unwrap());

        let todo = Todo::new("todo".into());
        todo_insert(&db, &todo, &PathBuf::new()).unwrap();

        tag_mapping_insert(&db, todo.id, tag.id).unwrap();

        let mapping = tag_mapping_fetch_all(&db).unwrap();
        assert_eq!(1, mapping.len());

        tag_mapping_clear(&db, todo.id).unwrap();

        let mapping = tag_mapping_fetch_all(&db).unwrap();
        assert_eq!(0, mapping.len());
    }
}
