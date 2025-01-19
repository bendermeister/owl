use super::super::util::*;
use super::Event;

pub(super) fn parse<'a>(events: &mut Vec<Event<'a>>, line: &'a str) -> &'a str {
    let (_, line) = chop(line, |c| *c != '\n');
    let line = &line[1..];
    events.push(Event::Break);
    return line;
}
