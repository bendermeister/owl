use crate::error::Error;
use crate::file_format::FileFormat;
use crate::store::Todo;
use crate::time;

pub fn parse_todos(body: &str, format: FileFormat) -> Result<Vec<Todo>, Error> {
    match format {
        FileFormat::Unknown => Ok(Vec::new()),
        FileFormat::Markdown => parse_todos_markdown(body),
        FileFormat::Typst => parse_todos_typst(body),
        FileFormat::C => parse_todos_clike(body),
        FileFormat::CPP => parse_todos_clike(body),
        FileFormat::Rust => parse_todos_clike(body),
        FileFormat::Go => parse_todos_clike(body),
        FileFormat::Java => parse_todos_clike(body),
        FileFormat::JavaScript => parse_todos_clike(body),
        FileFormat::TypeScript => parse_todos_clike(body),
        FileFormat::CSharp => parse_todos_clike(body),
    }
}

fn todo(title: String, line_number: usize) -> Todo {
    Todo {
        line_number,
        title,
        deadline: None,
        scheduled: None,
    }
}

fn parse_todos_clike(body: &str) -> Result<Vec<Todo>, Error> {
    let mut buf = Vec::new();

    for (line_number, line) in body.lines().map(|l| l.trim()).enumerate() {
        let line_number = line_number + 1;
        if let Some(title) = line.strip_prefix("// TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(stamp) = line.strip_prefix("// - DEADLINE:") {
            let stamp: time::Stamp = match stamp.trim().parse() {
                Ok(stamp) => stamp,
                Err(_) => return Err(Error::FailedToParse(line_number)),
            };
            if let Some(todo) = buf.last_mut() {
                todo.deadline = Some(stamp);
            }
        }

        if let Some(stamp) = line.strip_prefix("// - SCHEDULED:") {
            let stamp: time::Stamp = match stamp.trim().parse() {
                Ok(stamp) => stamp,
                Err(_) => return Err(Error::FailedToParse(line_number)),
            };
            if let Some(todo) = buf.last_mut() {
                todo.scheduled = Some(stamp);
            }
        }
    }

    Ok(buf)
}

fn parse_todos_typst(body: &str) -> Result<Vec<Todo>, Error> {
    let mut buf = Vec::new();

    for (line_number, line) in body.lines().enumerate() {
        let line_number = line_number + 1;
        if let Some(title) = line.strip_prefix("= TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("== TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("=== TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("==== TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("===== TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("====== TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(stamp) = line.strip_prefix("- DEADLINE:") {
            let stamp: time::Stamp = match stamp.trim().parse() {
                Ok(stamp) => stamp,
                Err(_) => return Err(Error::FailedToParse(line_number)),
            };
            if let Some(todo) = buf.last_mut() {
                todo.deadline = Some(stamp);
            }
        }

        if let Some(stamp) = line.strip_prefix("- SCHEDULED:") {
            let stamp: time::Stamp = match stamp.trim().parse() {
                Ok(stamp) => stamp,
                Err(_) => return Err(Error::FailedToParse(line_number)),
            };
            if let Some(todo) = buf.last_mut() {
                todo.scheduled = Some(stamp);
            }
        }
    }

    Ok(buf)
}

fn parse_todos_markdown(body: &str) -> Result<Vec<Todo>, Error> {
    let mut buf = Vec::new();

    for (line_number, line) in body.lines().enumerate() {
        let line_number = line_number + 1;
        if let Some(title) = line.strip_prefix("# TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("## TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("### TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("#### TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("##### TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(title) = line.strip_prefix("##### TODO:") {
            buf.push(todo(title.into(), line_number));
        }

        if let Some(stamp) = line.strip_prefix("> DEADLINE:") {
            let stamp: time::Stamp = match stamp.trim().parse() {
                Ok(stamp) => stamp,
                Err(_) => return Err(Error::FailedToParse(line_number)),
            };
            if let Some(todo) = buf.last_mut() {
                todo.deadline = Some(stamp);
            }
        }

        if let Some(stamp) = line.strip_prefix("> SCHEDULED:") {
            let stamp: time::Stamp = match stamp.trim().parse() {
                Ok(stamp) => stamp,
                Err(_) => return Err(Error::FailedToParse(line_number)),
            };
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

        let got = parse_todos_markdown(&body).unwrap();
        let expected = vec![
            Todo {
                title: "first todo".into(),
                line_number: 3,
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "second todo".into(),
                line_number: 4,
                deadline: Some(time::Stamp::from_ymd_hm(2025, 12, 1, 12, 0).unwrap()),
                scheduled: Some(time::Stamp::from_ymd_hm(2025, 11, 30, 14, 15).unwrap()),
            },
            Todo {
                title: "third todo".into(),
                line_number: 7,
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "fourth todo".into(),
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

        let got = parse_todos_typst(&body).unwrap();
        let expected = vec![
            Todo {
                title: "first todo".into(),
                line_number: 3,
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "second todo".into(),
                line_number: 4,
                deadline: Some(time::Stamp::from_ymd_hm(2025, 12, 1, 12, 0).unwrap()),
                scheduled: Some(time::Stamp::from_ymd_hm(2025, 11, 30, 14, 15).unwrap()),
            },
            Todo {
                title: "third todo".into(),
                line_number: 7,
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "fourth todo".into(),
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
