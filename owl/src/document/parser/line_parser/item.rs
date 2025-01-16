use super::SingleLine;
use super::Event;
use super::ErrorType;
use super::Trait;

#[derive(Debug)]
pub(super) struct Item<'a> {
    line: SingleLine<'a>
}

impl<'a> Trait<'a> for Item<'a> {
    fn wants(&self, line: &SingleLine<'a>) -> bool {
        todo!()
    }

    fn feed(&mut self, line: SingleLine<'a>) {
        todo!()
    }
}

impl<'a> Iterator for Item<'a> {
    type Item = Result<Event, ErrorType>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> From<SingleLine<'a>> for Item<'a> {
    fn from(value: SingleLine<'a>) -> Self {
        todo!()
    }
}
