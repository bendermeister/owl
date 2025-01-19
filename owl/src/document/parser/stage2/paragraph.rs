use super::stage1;
use super::Error;
use super::Event;
use std::iter::Peekable;

pub(super) fn parse<'a, 'b, I>(event: &mut Vec<Result<Event<'b>, Error>>, mut stage1: Peekable<I>) -> Peekable<I>
where
    'b: 'a,
    I: Iterator<Item = &'a stage1::Event<'b>> + Clone,
{
    let mut buffer = vec![];
    
    while let Some(stage1::Event::Indent(_)) = stage1.peek() {
        event.push(Err(Error::ParagraphIndented));
        stage1.next();
    }

    'main: loop {
        match stage1.peek() {
            Some(stage1::Event::Text(_)) => buffer.push(stage1.next().unwrap()),
            _ => break 'main,
        }
        match stage1.peek() {
            Some(stage1::Event::Break) => buffer.push(stage1.next().unwrap()),
            _ => break 'main,
        }
    }

    from_buffered(event, buffer);
    return stage1;
}

pub(super) fn from_buffered<'b>(
    event: &mut Vec<Result<Event<'b>, Error>>,
    stage1: Vec<&stage1::Event<'b>>,
) {
    event.push(Ok(Event::Paragraph));
    for token in stage1.iter() {
        match token {
            stage1::Event::Text(text) => event.push(Ok(Event::Text(text))),
            stage1::Event::Break => event.push(Ok(Event::Break)),
            _ => panic!("Unexpected token"),
        }
    }
    event.push(Ok(Event::ParagraphEnd));
}
