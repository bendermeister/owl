use super::ParserTrait;
use super::Event;
use super::Error;

pub(in super::super) struct Nothing<'a> {
    buf: &'a str
}

impl<'a> ParserTrait<'a> for Nothing<'a> {
    fn rest(&self) -> &'a str {
        self.buf
    }
}

impl<'a> Iterator for Nothing<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'a> From<&'a str> for Nothing<'a> {
    fn from(buf: &'a str) -> Self {
        Self {buf}
    }
}
