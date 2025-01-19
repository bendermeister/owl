use super::stage1;

mod item;

pub(super) enum Event<'a> {
    Heading(usize, &'a str),
    Break,
}

pub(super) enum Error {
    IndentedLine,
}

pub(super) fn parse<'a>(stage1: Vec<stage1::Event<'a>>) -> Vec<Result<Event, Error>> {
    todo!()
}
