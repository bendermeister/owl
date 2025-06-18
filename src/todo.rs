use crate::file_format::FileFormat;
use crate::time;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub title: String,
    pub file: PathBuf,
    pub deadline: Option<time::Stamp>,
    pub scheduled: Option<time::Stamp>,
    pub line_number: usize,
}

impl Todo {
    pub fn new(title: &str, file: PathBuf, line_number: usize) -> Self {
        Self {
            title: title.trim().to_owned(),
            deadline: None,
            scheduled: None,
            line_number,
            file,
        }
    }
}

pub fn parse_todos(body: &str, path: &Path) -> Result<Vec<Todo>, anyhow::Error> {
    match FileFormat::new(path) {
        FileFormat::Unknown => Ok(Vec::new()),
        FileFormat::Markdown => parse_todos_markdown(body, path),
        FileFormat::Typst => parse_todos_typst(body, path),
        FileFormat::C => parse_todos_clike(body, path),
        FileFormat::CPP => parse_todos_clike(body, path),
        FileFormat::Rust => parse_todos_clike(body, path),
        FileFormat::Go => parse_todos_clike(body, path),
        FileFormat::Java => parse_todos_clike(body, path),
        FileFormat::JavaScript => parse_todos_clike(body, path),
        FileFormat::TypeScript => parse_todos_clike(body, path),
        FileFormat::CSharp => parse_todos_clike(body, path),
    }
}

fn parse_todos_clike(body: &str, path: &Path) -> Result<Vec<Todo>, anyhow::Error> {
    let mut buf = Vec::new();

    for (line_number, line) in body.lines().map(|l| l.trim()).enumerate() {
        let line_number = line_number + 1;
        if let Some(title) = line.strip_prefix("// TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(stamp) = line.strip_prefix("// - DEADLINE:") {
            let stamp: time::Stamp = stamp.trim().parse()?;
            if let Some(todo) = buf.last_mut() {
                todo.deadline = Some(stamp);
            }
        }

        if let Some(stamp) = line.strip_prefix("// - SCHEDULED:") {
            let stamp: time::Stamp = stamp.trim().parse()?;
            if let Some(todo) = buf.last_mut() {
                todo.scheduled = Some(stamp);
            }
        }
    }

    Ok(buf)
}

fn parse_todos_typst(body: &str, path: &Path) -> Result<Vec<Todo>, anyhow::Error> {
    let mut buf = Vec::new();

    for (line_number, line) in body.lines().enumerate() {
        let line_number = line_number + 1;
        if let Some(title) = line.strip_prefix("= TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("== TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("=== TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("==== TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("===== TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("====== TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(stamp) = line.strip_prefix("- DEADLINE:") {
            let stamp: time::Stamp = stamp.trim().parse()?;
            if let Some(todo) = buf.last_mut() {
                todo.deadline = Some(stamp);
            }
        }

        if let Some(stamp) = line.strip_prefix("- SCHEDULED:") {
            let stamp: time::Stamp = stamp.trim().parse()?;
            if let Some(todo) = buf.last_mut() {
                todo.scheduled = Some(stamp);
            }
        }
    }

    Ok(buf)
}

fn parse_todos_markdown(body: &str, path: &Path) -> Result<Vec<Todo>, anyhow::Error> {
    let mut buf = Vec::new();

    for (line_number, line) in body.lines().enumerate() {
        let line_number = line_number + 1;
        if let Some(title) = line.strip_prefix("# TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("## TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("### TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("#### TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("##### TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(title) = line.strip_prefix("##### TODO:") {
            buf.push(Todo::new(title, path.to_owned(), line_number));
        }

        if let Some(stamp) = line.strip_prefix("> DEADLINE:") {
            let stamp: time::Stamp = stamp.trim().parse()?;
            if let Some(todo) = buf.last_mut() {
                todo.deadline = Some(stamp);
            }
        }

        if let Some(stamp) = line.strip_prefix("> SCHEDULED:") {
            let stamp: time::Stamp = stamp.trim().parse()?;
            if let Some(todo) = buf.last_mut() {
                todo.scheduled = Some(stamp);
            }
        }
    }

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_body_markdown() {
        let body = "
# This should be a normal heading
## TODO: first todo
## TODO: second todo
> DEADLINE: 2025-12-01 12:00
> SCHEDULED: 2025-11-30 14:15
### TODO: third todo
#### TODO: fourth todo
there should be some normal text here
";

        let path: PathBuf = "/some/path/file.typ".into();

        let got = parse_todos_markdown(&body, &path).unwrap();
        let expected = vec![
            Todo {
                title: "first todo".into(),
                file: path.clone(),
                line_number: 3,
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "second todo".into(),
                file: path.clone(),
                line_number: 4,
                deadline: Some(time::Stamp::from_ymd_hm(2025, 12, 1, 12, 0).unwrap()),
                scheduled: Some(time::Stamp::from_ymd_hm(2025, 11, 30, 14, 15).unwrap()),
            },
            Todo {
                title: "third todo".into(),
                line_number: 7,
                file: path.clone(),
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "fourth todo".into(),
                file: path.clone(),
                line_number: 8,
                deadline: None,
                scheduled: None,
            },
        ];

        dbg!(&expected);
        dbg!(&got);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_read_body_typst() {
        let body = "
= This should be a normal heading
= TODO: first todo
== TODO: second todo
- DEADLINE: 2025-12-01 12:00
- SCHEDULED: 2025-11-30 14:15
=== TODO: third todo
==== TODO: fourth todo
there should be some normal text here
";

        let path: PathBuf = "/some/path/file.typ".into();
        let got = parse_todos_typst(&body, &path).unwrap();
        let expected = vec![
            Todo {
                title: "first todo".into(),
                file: path.clone(),
                line_number: 3,
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "second todo".into(),
                file: path.clone(),
                line_number: 4,
                deadline: Some(time::Stamp::from_ymd_hm(2025, 12, 1, 12, 0).unwrap()),
                scheduled: Some(time::Stamp::from_ymd_hm(2025, 11, 30, 14, 15).unwrap()),
            },
            Todo {
                title: "third todo".into(),
                line_number: 7,
                file: path.clone(),
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "fourth todo".into(),
                file: path.clone(),
                line_number: 8,
                deadline: None,
                scheduled: None,
            },
        ];

        dbg!(&expected);
        dbg!(&got);
        assert_eq!(expected, got);
    }
}
