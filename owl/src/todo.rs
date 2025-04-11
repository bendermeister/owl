use crate::id::ID;
use crate::tag::Tag;
use crate::timestamp::TimeStamp;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub id: ID<Todo>,
    pub title: String,
    pub opened: TimeStamp,
    pub closed: Option<TimeStamp>,
    pub scheduled: Option<TimeStamp>,
    pub deadline: Option<TimeStamp>,
    pub tags: HashSet<ID<Tag>>,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: ID::generate(),
            title,
            opened: TimeStamp::now(),
            closed: None,
            deadline: None,
            scheduled: None,
            tags: HashSet::new(),
        }
    }

    pub fn generate_body(&self, tags: &HashMap<ID<Tag>, &str>) -> String {
        let mut body = String::new();

        // generate title
        body.push_str("# ");
        body.push_str(&self.title);
        body.push('\n');

        body.push_str("> OPENED: ");
        body.push_str(&self.opened.to_string());
        body.push('\n');

        if let Some(closed) = self.closed {
            body.push_str("> CLOSED: ");
            body.push_str(&closed.to_string());
            body.push('\n');
        }

        if !self.tags.is_empty() {
            body.push_str("> TAGS: ");
            for tag in self.tags.iter() {
                // TODO: is this unwrap ok?
                body.push_str(tags.get(tag).unwrap());
                body.push_str(", ");
            }
            body.pop();
            body.pop();
            body.push('\n');
        }

        body.trim().into()
    }

    pub fn update_from_body(
        &mut self,
        tags: &HashMap<String, ID<Tag>>,
        body: &str,
    ) -> Result<(), anyhow::Error> {
        let err = anyhow::anyhow!("failed to parse body");

        let mut body = body.trim().lines();

        let title = match body.next() {
            Some(title) => title,
            None => return Err(err),
        };

        if !title.starts_with("#") {
            return Err(err);
        }

        self.tags.clear();

        let title = &title[1..];
        self.title = title.trim().into();
        self.closed = None;
        self.scheduled = None;
        self.deadline = None;

        for line in body {
            if let Some(stamp) = line.strip_prefix("> OPENED:") {
                self.opened = stamp.trim().parse()?;
            }
            if let Some(stamp) = line.strip_prefix("> CLOSED:") {
                self.closed = Some(stamp.trim().parse()?);
            }
            if let Some(stamp) = line.strip_prefix("> DEADLINE:") {
                self.deadline = Some(stamp.trim().parse()?);
            }
            if let Some(stamp) = line.strip_prefix("> SCHEDULED:") {
                self.scheduled = Some(stamp.trim().parse()?);
            }
            if let Some(tag) = line.strip_prefix("> TAGS:") {
                let tag = tag.trim().split(",");
                for tag_name in tag {
                    let tag = match tags.get(tag_name.trim()) {
                        Some(t) => t,
                        None => return Err(anyhow::anyhow!("tag: '{}' is invalid", tag_name)),
                    };
                    self.tags.insert(*tag);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_body() {
        let mut todo = Todo::new("This is a Title".into());
        todo.opened = TimeStamp::from_ymd_hms(2024, 12, 03, 14, 30).unwrap();
        let got = todo.generate_body(&HashMap::new());

        let expected = "# This is a Title\n> OPENED: <2024-12-03 14:30>";

        assert_eq!(expected, got);

        todo.closed = Some(TimeStamp::from_ymd_hms(2025, 01, 12, 1, 12).unwrap());
        let expected =
            "# This is a Title\n> OPENED: <2024-12-03 14:30>\n> CLOSED: <2025-01-12 01:12>";
        let got = todo.generate_body(&HashMap::new());

        assert_eq!(expected, got);
    }

    #[test]
    fn test_update_from_body() {
        let mut expected = Todo::new("".into());
        expected.title = "Title".into();
        expected.opened = TimeStamp::from_ymd_hms(2024, 02, 13, 12, 30).unwrap();
        expected.closed = Some(TimeStamp::from_ymd_hms(2024, 02, 13, 12, 14).unwrap());

        let mut got = Todo::new("".into());
        got.id = expected.id;
        let body =
            "# Title\n\nSome Words\n> OPENED: <2024-02-13 12:30>\n\n> CLOSED: <2024-02-13 12:14>";
        got.update_from_body(&HashMap::new(), body).unwrap();

        assert_eq!(expected, got);

        let mut tags = HashMap::new();
        tags.insert("TagA".into(), ID::new(1));
        tags.insert("TagB".into(), ID::new(2));

        let body = "# Title\n\nSome Words\n> OPENED: <2024-02-13 12:30>\n\n> CLOSED: <2024-02-13 12:14>\n> TAGS: TagA, TagB";

        expected.tags.insert(ID::new(1));
        expected.tags.insert(ID::new(2));

        got.update_from_body(&tags, body).unwrap();

        assert_eq!(expected, got);
    }
}
