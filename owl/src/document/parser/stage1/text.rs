use super::super::util::*;
use super::Event;

pub(super) fn parse<'a>(event: &mut Vec<Event<'a>>, line: &'a str) -> &'a str {
    let (indent, line) = indent(line);
    if !indent.is_empty() {
        event.push(Event::Indent(indent));
    }
    let (line, buf) = chop(line, |c| *c != '\n');
    event.push(Event::Text(line.trim()));

    return buf;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let line = "Hello World";
        let mut event = vec![];
        let line = parse(&mut event, line);
        assert_eq!("", line);
        let expected = vec![
            Event::Text("Hello World"),
        ];
        assert_eq!(expected, event);
    }

    #[test]
    fn indent() {
        let line = "   Hello World";
        let mut event = vec![];
        let line = parse(&mut event, line);
        assert_eq!("", line);
        let expected = vec![Event::Indent("   "), Event::Text("Hello World")];
        assert_eq!(expected, event);
    }
}
