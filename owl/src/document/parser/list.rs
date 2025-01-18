use super::Error;
use super::Event;
use super::Subparser;

use super::paragraph;
use super::item;

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
        let line = line.1;
        if line.is_empty() {
            return true;
        }
        if line.chars().next().unwrap().is_whitespace() {
            return true;
        }
        return line.chars().next().unwrap() == '-';
    }

    fn feed(&mut self, line: (usize, &'a str)) {
        assert!(!self.resolved);
        self.lines.push(line);
    }
}

impl<'a> From<(usize, &'a str)> for Parser<'a> {
    fn from(line: (usize, &'a str)) -> Self {
        Self {
            resolved: false,
            lines: vec![line],
            events: vec![],
        }
    }
}

impl<'a> Parser<'a> {
    fn resolve(&mut self) {
        self.events.push(Ok(Event::List(1)));
        self.lines.reverse();

        loop {
            let line = match self.lines.pop() {
                Some(line) => line,
                None => break,
            };

            let mut generic: Generic = line.into();

            while let Some(line) = self.lines.iter().rev().next() {
                if !generic.wants(line) {
                    break;
                }
                generic.feed(self.lines.pop().unwrap());
            }

            while let Some(result) = generic.next() {
                self.events.push(result);
            }
        }

        self.events.push(Ok(Event::ListEnd));

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

enum Generic<'a> {
    Paragraph(paragraph::Parser<'a>),
    Item(item::Parser<'a>),
    Empty,
}

impl<'a> Subparser<'a> for Generic<'a> {
    fn wants(&self, line: &(usize, &'a str)) -> bool {
        match self {
            Generic::Paragraph(paragraph) => paragraph.wants(line),
            Generic::Empty => false,
            Generic::Item(item) => item.wants(line),
        }
    }

    fn feed(&mut self, line: (usize, &'a str)) {
        match self {
            Generic::Paragraph(paragraph) => paragraph.feed(line),
            Generic::Empty => unreachable!(),
            Generic::Item(item) => item.feed(line),
        }
    }
}

impl<'a> From<(usize, &'a str)> for Generic<'a> {
    fn from(line: (usize, &'a str)) -> Self {
        match line.1.chars().filter(|c| !c.is_whitespace()).next() {
            Some('-') => Generic::Item(line.into()),
            Some(_) => Generic::Paragraph(line.into()),
            None => Generic::Empty,
        }
    }
}

impl<'a> Iterator for Generic<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Result<Event<'a>, Error>> {
        match self {
            Generic::Paragraph(paragraph) => paragraph.next(),
            Generic::Empty => None,
            Generic::Item(item) => item.next(),
        }
    }
}
