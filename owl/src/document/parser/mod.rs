use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Lines;

use super::Error;
use super::Event;

mod heading;
mod paragraph;
mod list;
mod item;

pub(super) struct Parser<'a> {
    iter: Peekable<Enumerate<Lines<'a>>>,
    parser: Generic<'a>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(line) = self.iter.peek() {
            if !self.parser.wants(line) {
                break;
            }
            self.parser.feed(self.iter.next().unwrap());
        }

        match self.parser.next() {
            Some(event) => return Some(event),
            None => (),
        }

        let line = match self.iter.next() {
            Some(line) => line,
            None => return None,
        };

        self.parser = line.into();
        self.next()
    }
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        let iter = value.lines().enumerate().peekable();
        let parser = Generic::Empty;
        Self { iter, parser }
    }
}

trait Subparser<'a>: Iterator<Item = Result<Event<'a>, Error>> + From<(usize, &'a str)> {
    fn wants(&self, line: &(usize, &'a str)) -> bool;
    fn feed(&mut self, line: (usize, &'a str));
}

enum Generic<'a> {
    Heading(heading::Parser<'a>),
    Paragraph(paragraph::Parser<'a>),
    List(list::Parser<'a>),
    Empty,
}

impl<'a> Subparser<'a> for Generic<'a> {
    fn wants(&self, line: &(usize, &'a str)) -> bool {
        match self {
            Generic::Heading(heading) => heading.wants(line),
            Generic::Empty => false,
            Generic::Paragraph(paragraph) => paragraph.wants(line),
            Generic::List(list) => list.wants(line),
        }
    }

    fn feed(&mut self, line: (usize, &'a str)) {
        match self {
            Generic::Heading(heading) => heading.feed(line),
            Generic::Empty => unreachable!(),
            Generic::Paragraph(paragraph) => paragraph.feed(line),
            Generic::List(list) => list.feed(line),
        }
    }
}

impl<'a> Iterator for Generic<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Generic::Heading(heading) => heading.next(),
            Generic::Empty => None,
            Generic::Paragraph(paragraph) => paragraph.next(),
            Generic::List(list) => list.next(),
        }
    }
}

impl<'a> From<(usize, &'a str)> for Generic<'a> {
    fn from(line: (usize, &'a str)) -> Self {
        match line.1.chars().filter(|c| !c.is_whitespace()).next() {
            Some('#') => Generic::Heading(line.into()),
            Some('-') => Generic::List(line.into()),
            Some(_) => Generic::Paragraph(line.into()),
            None => Generic::Empty,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn heading() {
        let parser: Parser = "# Heading".into();
        let events: Vec<_> = parser.map(|r| r.unwrap()).collect();
        let expected = vec![Event::Heading(1), Event::Text("Heading"), Event::HeadingEnd];
        assert_eq!(expected, events);
    }

    #[test]
    fn heading_heading() {
        let parser: Parser = "# Heading\n## Heading 2".into();
        let events: Vec<_> = parser.map(|r| r.unwrap()).collect();
        let expected = vec![
            Event::Heading(1),
            Event::Text("Heading"),
            Event::HeadingEnd,
            Event::Heading(2),
            Event::Text("Heading 2"),
            Event::HeadingEnd,
        ];
        assert_eq!(expected, events);
    }

    #[test]
    fn heading_paragraph() {
        let parser: Parser = "# Heading\nThis should be a\nmulti line paragraph!".into();
        let events: Vec<_> = parser.map(|r| r.unwrap()).collect();
        let expected = vec![
            Event::Heading(1),
            Event::Text("Heading"),
            Event::HeadingEnd,
            Event::Paragraph,
            Event::Text("This should be a"),
            Event::Text("multi line paragraph!"),
            Event::ParagraphEnd,
        ];
        assert_eq!(expected, events);
    }

    #[test]
    fn list() {
        let parser: Parser = "# Heading\n- Item 1\n- Item 2\n- Item 3".into();
        let events: Vec<_> = parser.map(|r| r.unwrap()).collect();
        let expected = vec![
            Event::Heading(1),
            Event::Text("Heading"),
            Event::HeadingEnd,

            Event::List(1),

            Event::Item,
            Event::Paragraph,
            Event::Text("Item 1"),
            Event::ParagraphEnd,
            Event::ItemEnd,

            Event::Item,
            Event::Paragraph,
            Event::Text("Item 2"),
            Event::ParagraphEnd,
            Event::ItemEnd,

            Event::Item,
            Event::Paragraph,
            Event::Text("Item 3"),
            Event::ParagraphEnd,
            Event::ItemEnd,

            Event::ListEnd,
        ];
        assert_eq!(expected, events);
    }

    #[test]
    fn list_with_multiline_paragraph() {
        let parser: Parser = "- Item\n  More from Item\n- Another Item".into();
        let events: Vec<_> = parser.map(|r| r.unwrap()).collect();
        let expected = vec![
            Event::List(1),

            Event::Item,
            Event::Paragraph,
            Event::Text("Item"),
            Event::Text("More from Item"),
            Event::ParagraphEnd,
            Event::ItemEnd,

            Event::Item,
            Event::Paragraph,
            Event::Text("Another Item"),
            Event::ParagraphEnd,
            Event::ItemEnd,

            Event::ListEnd,
        ];
        assert_eq!(expected, events);
    }

    #[test]
    fn list_with_recursive_list() {
        let parser: Parser = "- Item\n  More of Item\n  - Subitem\n  - Subitem".into();
        let events: Vec<_> = parser.map(|r| r.unwrap()).collect();
        let expected = vec![
            Event::List(1),
            Event::Item,

            Event::Paragraph,
            Event::Text("Item"),
            Event::Text("More of Item"),
            Event::ParagraphEnd,

            Event::List(2),
            Event::Item,
            Event::Paragraph,
            Event::Text("Subitem"),
            Event::ParagraphEnd,
            Event::ItemEnd,
            Event::Item,
            Event::Paragraph,
            Event::Text("Subitem"),
            Event::ParagraphEnd,
            Event::ItemEnd,
            Event::ListEnd,

            Event::ItemEnd,
            Event::ListEnd,
        ];
        assert_eq!(expected, events);
    }

    fn recursive_list_with_multiline_subitem() {
    }
}
