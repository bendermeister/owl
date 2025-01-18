use super::Error;
use super::Event;
use super::Subparser;

pub(super) struct Parser<'a> {
    resolved: bool,
    events: Vec<Result<Event<'a>, Error>>,
    lines: Vec<(usize, &'a str)>,
}

impl<'a> Subparser<'a> for Parser<'a> {
    fn wants(&self, line: &(usize, &'a str)) -> bool {
        if self.resolved {
            return false;
        }

        let line = line.1;

        match line.chars().next() {
            None => false,
            Some('#') => false,
            Some('-') => false,
            Some(' ') => false,
            Some('\t') => false,
            Some(_) => true,
        }
    }

    fn feed(&mut self, line: (usize, &'a str)) {
        assert!(self.wants(&line));
        self.lines.push(line);
    }
}

impl<'a> Parser<'a> {
    fn resolve(&mut self) {
        assert!(!self.resolved);
        self.events.push(Ok(Event::Paragraph));
        for line in self.lines.iter() {
            if line.1.chars().next().unwrap().is_whitespace() {
                self.events.push(Err(Error::ParagraphWithIndent(line.0)));
            }
            self.events.push(Ok(Event::Text(line.1.trim())));
        }
        self.events.push(Ok(Event::ParagraphEnd));

        self.events.reverse();
        self.resolved = true;
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.resolved {
            self.resolve();
        }

        self.events.pop()
    }
}

impl<'a> From<(usize, &'a str)> for Parser<'a> {
    fn from(line: (usize, &'a str)) -> Self {
        Self {
            events: vec![],
            lines: vec![line],
            resolved: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn single_line() {
        let parser: Parser = (0, "Some Text").into();
        let events: Vec<_> = parser.map(|r| r.unwrap()).collect();
        let expected = vec![
            Event::Paragraph,
            Event::Text("Some Text"),
            Event::ParagraphEnd,
        ];
        assert_eq!(expected, events);
    }

    #[test]
    fn single_line_indented() {
        let parser: Parser = (0, "   This is some indented text").into();

        let got = parser.filter(|r| r.is_err()).next().unwrap();
        let expected = Error::ParagraphWithIndent(0);
        if let Err(got) = got {
            assert_eq!(expected, got);
        } else {
            panic!("Expected: {:?}, got: {:?}", Err::<Event, _>(expected), got);
        }
    }
}
