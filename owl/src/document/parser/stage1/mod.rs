mod empty;
mod heading;
mod item;
mod quote;
mod text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Event<'a> {
    Heading(usize, &'a str),
    Text(&'a str),
    Indent(&'a str),
    Break,
    Item(&'a str),
    Quote(&'a str),
}

impl<'a> Event<'a> {
    pub(super) fn is_indent(&self) -> bool {
        match self {
            Event::Indent(_) => true,
            _ => false,
        }
    }
}

pub(super) fn parse<'a>(buf: &'a str) -> Vec<Event<'a>> {
    let mut event = vec![];
    let mut buf = buf;
    while let Some(c) = buf
        .chars()
        .filter(|c| *c == '\n' || !c.is_whitespace())
        .next()
    {
        match c {
            '#' => buf = heading::parse(&mut event, buf),
            '\n' => buf = empty::parse(&mut event, buf),
            '>' => buf = quote::parse(&mut event, buf),
            '-' => buf = item::parse(&mut event, buf),
            _ => buf = text::parse(&mut event, buf),
        }
    }

    return event;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn heading() {
        let buf = "# Heading\n## Heading 2";
        let event = parse(&buf);
        let expected = vec![
            Event::Heading(1, "Heading"),
            Event::Break,
            Event::Heading(2, "Heading 2"),
        ];
        assert_eq!(expected, event);
    }

    #[test]
    fn heading_item_item_heading() {
        let buf = "# Heading\n- Item 1\n- Item 2";
        let event = parse(buf);
        let expected = vec![
            Event::Heading(1, "Heading"),
            Event::Break,
            Event::Item("Item 1"),
            Event::Break,
            Event::Item("Item 2"),
        ];
        assert_eq!(expected, event);
    }

    #[test]
    fn heading_quote_quote_quote() {
        let buf = "# Heading\n>Quote 1\n>Quote 2\n> Quote 3";
        let event = parse(buf);
        let expected = vec![
            Event::Heading(1, "Heading"),
            Event::Break,
            Event::Quote("Quote 1"),
            Event::Break,
            Event::Quote("Quote 2"),
            Event::Break,
            Event::Quote("Quote 3"),
        ];
        assert_eq!(expected, event);
    }
}
