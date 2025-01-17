use super::ParserTrait;
use super::Event;
use super::Error;
use super::Context;

mod nothing;
mod heading;

pub(super) enum SubParser<'a> {
    Nothing(nothing::Nothing<'a>),
}

impl<'a> ParserTrait<'a> for SubParser<'a> {
    fn rest(&self) -> &'a str {
        match self {
            SubParser::Nothing(nothing) => nothing.rest(),
        }
    }
}

impl<'a> Iterator for SubParser<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SubParser::Nothing(nothing) => nothing.next(),
        }
    }
}

impl<'a> From<&'a str> for SubParser<'a> {
    fn from(value: &'a str) -> Self {
        if value.is_empty() {
            return SubParser::Nothing(value.into());
        }
        todo!()
    }
}


