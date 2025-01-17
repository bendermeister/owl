mod parser;

pub struct Context {
    line_number: usize,
}

pub enum Error {
}

pub struct Document<'a> {
    body: Vec<Event<'a>>,
}

enum Event<'a> {
    Heading(u8),
    HeadingEnd,
    Text(&'a str),
}

impl<'a> TryFrom<&'a str> for Document<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parser: parser::Parser = value.into();
        let mut body = vec![];

        while let Some(result) = parser.next() {
            let event = match result {
                Ok(event) => event,
                Err(error) => return Err(error),
            };
            body.push(event);
        }

        Ok(Document{body})
    }
}
