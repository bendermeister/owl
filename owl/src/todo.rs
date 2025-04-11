use crate::id::ID;
use crate::timestamp::TimeStamp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub id: ID<Todo>,
    pub title: String,
    pub opened: TimeStamp,
    pub closed: Option<TimeStamp>,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: ID::generate(),
            title,
            opened: TimeStamp::now(),
            closed: None,
        }
    }

    pub fn generate_body(&self) -> String {
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

        body.trim().into()
    }

    pub fn update_from_body(&mut self, body: &str) -> Result<(), anyhow::Error> {
        let err = anyhow::anyhow!("failed to parse body");

        let mut body = body.trim().lines();

        let title = match body.next() {
            Some(title) => title,
            None => return Err(err),
        };

        if !title.starts_with("#") {
            return Err(err);
        }

        let title = &title[1..];
        self.title = title.trim().into();

        self.closed = None;

        for line in body {
            if line.starts_with("> OPENED:") {
                let line = &line[9..];
                self.opened = line.parse()?;
            }
            if line.starts_with("> CLOSED:") {
                let line = &line[9..];
                self.closed = Some(line.parse()?);
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
        let got = todo.generate_body();

        let expected = "# This is a Title\n> OPENED: <2024-12-03 14:30>";

        assert_eq!(expected, got);

        todo.closed = Some(TimeStamp::from_ymd_hms(2025, 01, 12, 1, 12).unwrap());
        let expected =
            "# This is a Title\n> OPENED: <2024-12-03 14:30>\n> CLOSED: <2025-01-12 01:12>";
        let got = todo.generate_body();

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
        got.update_from_body(body).unwrap();

        assert_eq!(expected, got);
    }
}
