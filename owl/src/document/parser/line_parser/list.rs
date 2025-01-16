use super::SingleLine;
use super::Trait;
use super::Event;
use super::ErrorType;

#[derive(Debug)]
pub(super) struct List<'a> {
    level: u8,
    line: SingleLine<'a>,
}

impl<'a> Trait<'a> for List<'a> {
    fn wants(&self, line: &SingleLine<'a>) -> bool {
        todo!()
    }

    fn feed(&mut self, line: SingleLine<'a>) {
        todo!()
    }
}

impl<'a> Iterator for List<'a> {
    type Item = Result<Event, ErrorType>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> From<SingleLine<'a>> for List<'a> {
    fn from(value: SingleLine<'a>) -> Self {
        todo!()
    }
}
