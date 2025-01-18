use super::Subparser;
use super::Event;
use super::Error;

pub(super) struct Parser<'a> {
    events: Vec<Result<Event<'a>, Error>>,
}

impl<'a> Subparser<'a> for Parser<'a> {
    fn wants(&self, _: &(usize, &'a str)) -> bool {
        false
    }

    fn feed(&mut self, _: (usize, &'a str)) {
        todo!()
    }
}

impl<'a> From<(usize, &'a str)>for Parser<'a> {
    fn from(line: (usize, &'a str)) -> Self {
        let line_number = line.0;
        let mut line = line.1;

        let mut events = Vec::with_capacity(4);

        let mut indent = 0;
        while !line.is_empty() && line.chars().next().unwrap().is_whitespace() {
            line = &line[1..];
            indent += 1;
        }

        if indent > 0 {
            events.push(Err(Error::HeadingWithIndent(line_number)))
        }

        let mut level = 0;
        while let Some('#') = line.chars().next() {
            level += 1;
            line = &line[1..];
        }

        events.push(Ok(Event::Heading(level)));
        events.push(Ok(Event::Text(line.trim())));
        events.push(Ok(Event::HeadingEnd));

        events.reverse();

        Parser {events}
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.events.pop()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn heading() {
        let mut heading: Parser = (0, "# This should be a heading").into();

        let got = heading.next().unwrap().unwrap();
        let expected = Event::Heading(1);
        assert_eq!(expected, got);

        let got = heading.next().unwrap().unwrap();
        let expected = Event::Text("This should be a heading");
        assert_eq!(expected, got);

        let got = heading.next().unwrap().unwrap();
        let expected = Event::HeadingEnd;
        assert_eq!(expected, got);

        assert!(heading.next().is_none());
    }

    #[test]
    fn heading_with_indent() {
        let mut heading: Parser = (0, "  # Heading with Indent").into();

        let got = heading.next().unwrap();
        let expected = Error::HeadingWithIndent(0);
        if let Err(got) = got {
            assert_eq!(expected, got);
        } else {
            panic!("Expected: {:?}, got: {:?}", Err::<Event, _>(expected), got);
        }

        let got = heading.next().unwrap().unwrap();
        let expected = Event::Heading(1);
        assert_eq!(expected, got);

        let got = heading.next().unwrap().unwrap();
        let expected = Event::Text("Heading with Indent");
        assert_eq!(expected, got);

        let got = heading.next().unwrap().unwrap();
        let expected = Event::HeadingEnd;
        assert_eq!(expected, got);

        assert!(heading.next().is_none());
    }
}

