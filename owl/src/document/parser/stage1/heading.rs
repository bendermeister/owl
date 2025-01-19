use super::super::util::*;
use super::Event;

pub(super) fn parse<'a>(event: &mut Vec<Event<'a>>, line: &'a str) -> &'a str {
    let (indent, line) = indent(line);
    let (level, line) = chop(line, |c| *c == '#');
    let (text, line) = chop(line, |c| *c != '\n');

    if !indent.is_empty() {
        event.push(Event::Indent(indent));
    }
    event.push(Event::Heading(level.len(), text.trim()));
    return line;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn heading() {
        let mut events = vec![];
        let line = "# This should be a heading";
        let line = parse(&mut events, line);
        let expected = "";
        assert_eq!(expected, line);
        let expected = vec![
            Event::Heading(1, "This should be a heading"),
        ];
        assert_eq!(expected, events);
    }

    #[test]
    fn heading_with_indent() {
        let mut event = vec![];
        let line = " # Heading with indent";
        let line = parse(&mut event, line);
        assert_eq!("", line);
        let expected = vec![
            Event::Indent(" "),
            Event::Heading(1, "Heading with indent"),
        ];
        assert_eq!(expected, event);
    }
}
