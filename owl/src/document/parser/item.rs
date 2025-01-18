use super::Event;
use super::Error;
use super::Subparser;

use super::paragraph;
use super::list;

pub(super) struct Parser<'a> {
    resolved: bool,
    lines: Vec<(usize, &'a str)>,
    events: Vec<Result<Event<'a>, Error>>,
}

impl<'a> Subparser<'a> for Parser<'a> {
    fn wants(&self, line: &(usize, &'a str)) -> bool {
        if self.resolved {
            return false;
        }
        if line.1.is_empty() {
            return true;
        }
        line.1.chars().next().unwrap().is_whitespace()
    }

    fn feed(&mut self, line: (usize, &'a str)) {
        assert!(!self.resolved);
        let (line_number, mut line) = line;
        assert_eq!(Some(' '), line.chars().next());
        line = &line[1..];
        assert_eq!(Some(' '), line.chars().next());
        line = &line[1..];
        self.lines.push((line_number, line));
    }
}

impl<'a> Parser<'a> {
    fn resolve(&mut self) {
        assert!(!self.resolved);
        self.lines.reverse();
        self.events.push(Ok(Event::Item));

        while let Some(line) = self.lines.pop() {
            let mut parser: Generic = line.into();

            while let Some(line) = self.lines.iter().rev().next() {
                if !parser.wants(line) {
                    break;
                }
                parser.feed(self.lines.pop().unwrap());
            }

            while let Some(event) = parser.next() {
                let event = match event {
                    Ok(Event::List(level)) => Ok(Event::List(level + 1)),
                    event => event,
                };
                self.events.push(event);
            }
        }

        self.events.push(Ok(Event::ItemEnd));
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
        assert_eq!(Some('-'), line.1.chars().next());
        let (line_number, mut line) = line;

        line = &line[1..].trim();

        Self {
            resolved: false,
            lines: vec![(line_number, line)],
            events: vec![],
        }
    }
}

enum Generic<'a> {
    Paragraph(paragraph::Parser<'a>),
    Empty,
    List(list::Parser<'a>),
}

impl<'a> Subparser<'a> for Generic<'a> {
    fn wants(&self, line: &(usize, &'a str)) -> bool {
        match self {
            Generic::Paragraph(parser) => parser.wants(line),
            Generic::Empty => false,
            Generic::List(parser) => parser.wants(line),
        }
    }

    fn feed(&mut self, line: (usize, &'a str)) {
        match self {
            Generic::Paragraph(parser) => parser.feed(line),
            Generic::Empty => unreachable!(),
            Generic::List(parser) => parser.feed(line),
        }
    }
}

impl<'a> From<(usize, &'a str)> for Generic<'a> {
    fn from(line: (usize, &'a str)) -> Self {
        match line.1.chars().next() {
            Some('-') => Self::List(line.into()),
            Some(_) => Self::Paragraph(line.into()),
            None => Self::Empty,
        }
    }
}

impl<'a> Iterator for Generic<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Generic::Paragraph(parser) => parser.next(),
            Generic::Empty => None,
            Generic::List(parser) => parser.next(),
        }
    }
}
