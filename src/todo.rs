use crate::types::time_stamp::TimeStamp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub title: String,
    pub deadline: Option<TimeStamp>,
    pub scheduled: Option<TimeStamp>,
}

impl Todo {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.trim().to_owned(),
            deadline: None,
            scheduled: None,
        }
    }
}

pub fn parse_todos(body: &str) -> Result<Vec<Todo>, anyhow::Error> {
    let mut buf = Vec::new();

    for line in body.lines() {
        if let Some(title) = line.strip_prefix("# TODO:") {
            buf.push(Todo::new(title));
        }

        if let Some(title) = line.strip_prefix("## TODO:") {
            buf.push(Todo::new(title));
        }

        if let Some(title) = line.strip_prefix("### TODO:") {
            buf.push(Todo::new(title));
        }

        if let Some(title) = line.strip_prefix("#### TODO:") {
            buf.push(Todo::new(title));
        }

        if let Some(title) = line.strip_prefix("##### TODO:") {
            buf.push(Todo::new(title));
        }

        if let Some(title) = line.strip_prefix("##### TODO:") {
            buf.push(Todo::new(title));
        }

        if let Some(stamp) = line.strip_prefix("> DEADLINE:") {
            let stamp: TimeStamp = stamp.trim().parse()?;
            if let Some(todo) = buf.last_mut() {
                todo.deadline = Some(stamp);
            }
        }

        if let Some(stamp) = line.strip_prefix("> SCHEDULED:") {
            let stamp: TimeStamp = stamp.trim().parse()?;
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
    fn test_read_body() {
        let body = "
# This should be a normal heading
# TODO: first todo
## TODO: second todo
> DEADLINE: <2025-12-01 12:00>
> SCHEDULED: <2025-11-30 14:15>
### TODO: third todo
#### TODO: fourth todo
there should be some normal text here
";
        let got = parse_todos(body).unwrap();
        let expected = vec![
            Todo {
                title: "first todo".into(),
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "second todo".into(),
                deadline: Some(TimeStamp::from_ymd_hm(2025, 12, 1, 12, 0).unwrap()),
                scheduled: Some(TimeStamp::from_ymd_hm(2025, 11, 30, 14, 15).unwrap()),
            },
            Todo {
                title: "third todo".into(),
                deadline: None,
                scheduled: None,
            },
            Todo {
                title: "fourth todo".into(),
                deadline: None,
                scheduled: None,
            },
        ];

        dbg!(&expected);
        dbg!(&got);
        assert_eq!(expected, got);
    }
}
