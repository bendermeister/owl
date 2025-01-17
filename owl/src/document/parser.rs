use super::Context;
use super::Error;
use super::Event;

mod sub_parser;
use sub_parser::SubParser;

trait ParserTrait<'a>: Iterator<Item = Result<Event<'a>, Error>> + From<&'a str> {
    fn rest(&self) -> &'a str;
}

pub(super) struct Parser<'a> {
    parser: SubParser<'a>,
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            parser: value.into(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parser.next() {
            Some(result) => return Some(result),
            None => (),
        }
        let rest = self.parser.rest();
        if rest.is_empty() {
            return None;
        }
        self.parser = rest.into();
        self.next()
    }
}
