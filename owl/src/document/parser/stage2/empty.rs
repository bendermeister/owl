use super::stage1;
use super::Event;
use super::Error;

pub(super) fn parse<'a, 'b, I>(event: &mut Vec<Result<Event<'b>, Error>>, mut stage1: I) -> I
where
    'b: 'a,
    I: Iterator<Item = &'a stage1::Event<'b>>
{
    event.push(Ok(Event::Break));
    while let Some(token) = stage1.next() {
        if let stage1::Event::Break = token {
            break;
        }
    }

    return stage1;
}
