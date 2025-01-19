use super::super::util::*;
use super::Event;

pub(super) fn parse<'a>(event: &mut Vec<Event<'a>>, line: &'a str) -> &'a str {
    let (indent, line) = indent(line);
    if !indent.is_empty() {
        event.push(Event::Indent(indent));
    }
    let (item, line) = chop(line, |c| *c != '\n');

    assert_eq!("-", &item[..1]);
    let item = &item[1..];

    event.push(Event::Item(item.trim()));

    return line;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let line = "- Some Item";
        let mut event = vec![];
        let line = parse(&mut event, line);
        assert_eq!("", line);
        assert_eq!(vec![Event::Item("Some Item")], event);
    }

    #[test]
    fn indent() {
        let line = "  - Some Item";
        let mut event = vec![];
        let line = parse(&mut event, line);
        assert_eq!("", line);
        assert_eq!(vec![Event::Indent("  "), Event::Item("Some Item")], event);
    }

}
