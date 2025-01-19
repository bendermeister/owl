use super::Event;
use super::super::util::*;

pub(super) fn parse<'a>(event: &mut Vec<Event<'a>>, line: &'a str) -> &'a str {
    let (indent, line) = indent(line);
    if !indent.is_empty() {
        event.push(Event::Indent(indent));
    }
    let (quote, line) = chop(line, |c| *c != '\n');

    assert_eq!(">", &quote[..1]);
    let quote = &quote[1..];

    event.push(Event::Quote(quote.trim()));
    return line;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let line = "> Quote";
        let mut event = vec![];
        let line = parse(&mut event, line);
        assert_eq!("", line);
        assert_eq!(vec![Event::Quote("Quote")], event);
    }

    #[test]
    fn indent() {
        let line = "   > Quote";
        let mut event = vec![];
        let line = parse(&mut event, line);
        assert_eq!("", line);
        assert_eq!(vec![Event::Indent("   "), Event::Quote("Quote")], event);
    }
}
