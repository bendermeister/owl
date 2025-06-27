#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    ParsingError(Option<usize>),
}
