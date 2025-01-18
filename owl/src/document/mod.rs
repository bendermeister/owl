mod parser;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Document<'a> {
    body: Vec<Event<'a>>,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    HeadingWithIndent(usize),
    ParagraphWithIndent(usize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Event<'a> {
    Heading(u8),
    HeadingEnd,
    Text(&'a str),
    Paragraph,
    ParagraphEnd,

    List(u8),
    ListEnd,
    Item,
    ItemEnd,
}

#[cfg(test)] 
mod test {
}
