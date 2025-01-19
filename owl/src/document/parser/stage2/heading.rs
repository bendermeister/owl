use super::stage1;
use super::Error;
use super::Event;

pub(super) fn parse<'a, 'b,I>(event: &mut Vec<Result<Event<'b>, Error>>, mut stage1: I) -> I
where
    I: Iterator<Item = &'a stage1::Event<'b>>,
    'b: 'a,
{
    let mut token = stage1.next().unwrap();

    if let stage1::Event::Indent(_) = token {
        event.push(Err(Error::HeadingIndented));
        token = stage1.next().unwrap();
    }

    match token {
        stage1::Event::Heading(l, t) => event.push(Ok(Event::Heading(*l, *t))),
        e => panic!(
            "Expected {:?}, got: {:?}",
            stage1::Event::Heading(1, "Some Heading"),
            e
        ),
    }

    return stage1;
}
