mod parser;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Document {
    body: Vec<Event>
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Event {
    Word(String),
    Text,
    TextEnd,
    Heading(u8),
    HeadingEnd,
    WhiteSpace(char),
    Special(char),
    Link(String),
}
