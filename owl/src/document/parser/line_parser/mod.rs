use super::Event;
use super::ErrorType;
use super::SingleLine;

mod text;
use text::Text;

mod heading;
use heading::Heading;

mod list;
use list::List;

mod item;

pub trait Trait<'a>:
    Iterator<Item = Result<Event, ErrorType>> + From<SingleLine<'a>>
{
    fn wants(&self, line: &SingleLine<'a>) -> bool;
    fn feed(&mut self, line: SingleLine<'a>);
}

#[derive(Debug)]
pub(super) enum LineParser<'a> {
    Heading(Heading<'a>),
    List(List<'a>),
}

impl<'a> From<SingleLine<'a>> for LineParser<'a> {
    fn from(value: SingleLine<'a>) -> Self {
        Self::Heading(value.into())
    }
}

impl<'a> Iterator for LineParser<'a> {
    type Item = Result<Event, ErrorType>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> Trait<'a> for LineParser<'a> {
    fn wants(&self, line: &SingleLine<'a>) -> bool {
        match self {
            LineParser::Heading(heading) => heading.wants(line),
            LineParser::List(list) => list.wants(line),
        }
    }

    fn feed(&mut self, line: SingleLine<'a>) {
        match self {
            LineParser::Heading(heading) => heading.feed(line),
            LineParser::List(list) => list.feed(line),
        }
    }
}
