use super::Event;
use std::iter::Enumerate;
use std::iter::Peekable;

mod single_line;
use single_line::SingleLine;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ErrorType {
    TextIndented,
    HeadingIndented,
}

mod line_parser;
use line_parser::LineParser;
use line_parser::Trait;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Error {
    pub line_number: usize,
    pub error: ErrorType,
}

#[derive(Debug)]
pub(super) struct Parser<'a, LineIterator>
where
    LineIterator: Iterator<Item = SingleLine<'a>>,
{
    line_number: usize,
    parser: LineParser<'a>,
    lines: Peekable<Enumerate<LineIterator>>,
}


impl<'a, LineIterator> Iterator for Parser<'a, LineIterator> 
where
    LineIterator: Iterator<Item = SingleLine<'a>>
{
    type Item = Result<Event, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, line)) = self.lines.peek() {
            if self.parser.wants(line) {
                let (line_number, line) = self.lines.next().unwrap();
                self.parser.feed(line);
                self.line_number = line_number;
            }
        }

        let line_number = self.line_number;

        match self.parser.next() {
            Some(Ok(event)) => return Some(Ok(event)),
            Some(Err(error)) => return Some(Err(Error{line_number, error})),
            None => (),
        }

        let (line_number, line) = match self.lines.next() {
            Some(line) => line,
            None => return None,
        };
        self.line_number = line_number;
        self.parser = line.into();

        self.next()
    }
}
