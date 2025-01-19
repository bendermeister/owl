use super::stage1;

mod empty;
mod heading;
mod item;
mod paragraph;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Event<'a> {
    Heading(usize, &'a str),

    Text(&'a str),

    Paragraph,
    ParagraphEnd,

    Item(usize),
    ItemEnd,

    Break,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Error {
    HeadingIndented,
    ParagraphIndented,
}

pub(super) fn parse<'a, 'b>(
    stage1: &'a Vec<stage1::Event<'b>>,
) -> Vec<Result<Event<'b>, Error>> 
where 
    'b: 'a,
{
    let mut stage1 = stage1.iter().peekable();
    let mut event = vec![];

    while let Some(token) = stage1.clone().filter(|e| !e.is_indent()).next() {
        match token {
            stage1::Event::Heading(_, _) => stage1 = heading::parse(&mut event, stage1),
            stage1::Event::Text(_) => stage1 = paragraph::parse(&mut event, stage1),
            stage1::Event::Indent(_) => unreachable!(),
            stage1::Event::Break => stage1 = empty::parse(&mut event, stage1),
            stage1::Event::Item(_) => todo!(),
            stage1::Event::Quote(_) => todo!(),
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
        let stage1 = stage1::parse(buf);
        let stage2: Vec<_> = parse(&stage1).into_iter().map(|r| r.unwrap()).collect();
        let expected = vec![
            Event::Heading(1, "Heading"),
            Event::Break,
            Event::Heading(2, "Heading 2"),
        ];
        assert_eq!(expected, stage2);
    }

    #[test]
    #[should_panic]
    fn heading_with_indent() {
        let buf = "  # Heading\n## Heading 2";
        let stage1 = stage1::parse(buf);
        let _: Vec<_> = parse(&stage1).into_iter().map(|r| r.unwrap()).collect();
    }

    #[test]
    fn paragraph() {
        let buf = "This should be some\nnice multiline\nParagraph";
        let stage1 = stage1::parse(buf);
        let stage2: Vec<_> = parse(&stage1).into_iter().map(|r| r.unwrap()).collect();
        let exected = vec![
            Event::Paragraph,
            Event::Text("This should be some"),
            Event::Break,
            Event::Text("nice multiline"),
            Event::Break,
            Event::Text("Paragraph"),
            Event::ParagraphEnd,
        ];
        assert_eq!(exected, stage2);
    }

    #[test]
    fn  paragraph_paragraph() {
        let buf = "Para 1\nPara 1\n\nPara 2\nPara 2";
        let stage1 = stage1::parse(buf);
        let stage2: Vec<_> = parse(&stage1).into_iter().map(|r| r.unwrap()).collect();

        let exected = vec![
            Event::Paragraph,
            Event::Text("Para 1"),
            Event::Break,
            Event::Text("Para 1"),
            Event::Break,
            Event::ParagraphEnd,

            Event::Break,

            Event::Paragraph,
            Event::Text("Para 2"),
            Event::Break,
            Event::Text("Para 2"),
            Event::ParagraphEnd,
        ];
        assert_eq!(exected, stage2);
    }
}
