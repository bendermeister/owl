mod line;
mod block;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    line_number: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NonSpaceIndent,
    HeadingWithLeadingSpace,
    ItemWithOddLeadingSpace,
    QuoteWithLeadingSpace,
    IndentedTextBlock,
}
