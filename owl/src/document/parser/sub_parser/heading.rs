use super::Event;
use super::ParserTrait;
use super::Context;
use super::Event;

pub(in super::super) struct Heading<'a> {
    buf: &'a str,
}
