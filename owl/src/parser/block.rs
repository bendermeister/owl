use super::Error;

use super::line;
use super::line::Line;

use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub struct Heading<'a, M> {
    level: u8,
    body: Vec<(M, &'a str)>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a, M> {
    body: Vec<(M, &'a str)>,
}

impl<'a, M> Text<'a, M> {
    fn feed(&mut self, meta_info: M, text: line::Text<'a>) {
        assert_eq!(text.indent, 0);
        self.body.push((meta_info, text.body));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item<'a, M> {
    level: u8,
    body: Vec<(M, &'a str)>,
}

impl<'a, M> Item<'a, M> {
    fn feed(&mut self, meta_info: M, text: line::Text<'a>) {
        assert_eq!(text.indent, 2 * self.level);
        self.body.push((meta_info, text.body));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Quote<'a, M> {
    body: Vec<(M, &'a str)>,
}

impl<'a, M> Quote<'a, M> {
    fn feed(&mut self, meta_info: M, quote: line::Quote<'a>) {
        self.body.push((meta_info, quote.body));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Block<'a, M> {
    Heading(Heading<'a, M>),
    Text(Text<'a, M>),
    Item(Item<'a, M>),
    Quote(Quote<'a, M>),
}

impl<'a, M> Block<'a, M> {
    fn wants(&self, line: &Line) -> bool {
        match (self, line) {
            (Block::Heading(_), _) => false,
            (Block::Text(_), Line::Text(line)) => line.indent == 0,
            (Block::Quote(_), Line::Quote(_)) => true,
            (Block::Item(item), Line::Text(text)) if item.level * 2 == text.indent => true,
            _ => false,
        }
    }

    fn feed(&mut self, meta_info: M, line: Line<'a>) {
        match (self, line) {
            (Block::Quote(block), Line::Quote(line)) => block.feed(meta_info, line),
            (Block::Text(block), Line::Text(line)) => block.feed(meta_info, line),
            (Block::Item(block), Line::Text(line)) => block.feed(meta_info, line),
            _ => unreachable!(),
        }
    }
}

impl<'a, M> TryFrom<(M, line::Heading<'a>)> for Block<'a, M> {
    type Error = Error;

    fn try_from(line: (M, line::Heading<'a>)) -> Result<Self, Self::Error> {
        Ok(Block::Heading(Heading {
            level: line.1.level,
            body: vec![(line.0, line.1.body)],
        }))
    }
}

impl<'a, M> TryFrom<(M, line::Item<'a>)> for Block<'a, M> {
    type Error = Error;

    fn try_from(line: (M, line::Item<'a>)) -> Result<Self, Self::Error> {
        Ok(Block::Item(Item {
            level: line.1.level,
            body: vec![(line.0, line.1.body)],
        }))
    }
}

impl<'a, M> TryFrom<(M, line::Quote<'a>)> for Block<'a, M> {
    type Error = Error;

    fn try_from(line: (M, line::Quote<'a>)) -> Result<Self, Self::Error> {
        Ok(Block::Quote(Quote {
            body: vec![(line.0, line.1.body)],
        }))
    }
}

impl<'a, M> TryFrom<(M, line::Text<'a>)> for Block<'a, M> {
    type Error = Error;

    fn try_from(line: (M, line::Text<'a>)) -> Result<Self, Self::Error> {
        if line.1.indent != 0 {
            return Err(Error::IndentedTextBlock);
        }
        Ok(Block::Text(Text {
            body: vec![(line.0, line.1.body)],
        }))
    }
}

impl<'a, M> TryFrom<(M, Line<'a>)> for Block<'a, M> {
    type Error = Error;

    fn try_from((meta_info, line): (M, Line<'a>)) -> Result<Self, Self::Error> {
        match line {
            Line::Heading(heading) => (meta_info, heading).try_into(),
            Line::Item(item) => (meta_info, item).try_into(),
            Line::Text(text) => (meta_info, text).try_into(),
            Line::Quote(quote) => (meta_info, quote).try_into(),
            Line::Break => panic!("unreachable"),
        }
    }
}

pub struct LineParser<'a, I, M>
where
    M: Clone,
    I: Iterator<Item = (M, &'a str)>
{
    iter: I,
}

impl<'a, I, M> Iterator for LineParser<'a, I, M>
where
    M: Clone,
    I: Iterator<Item = (M, &'a str)>
{
    type Item = (M, Result<Line<'a>, Error>);

    fn next(&mut self) -> Option<Self::Item> {
        let (meta_info, line) = match self.iter.next() {
            Some(line) => line,
            None => return None,
        };
        Some((meta_info, line.try_into()))
    }
}

pub struct Blocker<'a, I, M> 
where
    M: Clone,
    I: Iterator<Item = (M, &'a str)>
{
    iter: Peekable<LineParser<'a, I, M>>,
}

impl<'a, I, M> Blocker<'a, I, M>
where
    M: Clone,
    I: Iterator<Item = (M, &'a str)>
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: LineParser{iter}.peekable(),
        }
    }
}

impl<'a, I, M> Iterator for Blocker<'a, I, M>
where
    M: Clone,
    I: Iterator<Item = (M, &'a str)>,
{

    type Item = (M, Result<Block<'a, M>, Error>);

    fn next(&mut self) -> Option<Self::Item> {
        let (meta_info, line) = match self.iter.next() {
            Some(n) => n,
            None => return None,
        };

        let line = match line {
            Ok(Line::Break) => return self.next(),
            Ok(line) => line,
            Err(err) => return Some((meta_info, Err(err))),
        };

        let mut block: Block<M> = match (meta_info.clone(), line).try_into() {
            Ok(block) => block,
            Err(err) => return Some((meta_info, Err(err))),
        };

        while let Some((_, Ok(line))) = self.iter.peek() {
            if !block.wants(line) {
                break;
            }

            let (meta_info, line) = self.iter.next().unwrap();
            block.feed(meta_info, line.unwrap());
        }

        Some((meta_info, Ok(block)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn heading() {
        //let liner: liner::Liner = "# Heading".try_into().unwrap();
        let mut blocker = Blocker::new("# Heading".lines().enumerate());

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Heading(Heading {
            level: 1,
            body: vec![(0, "Heading")],
        });
        assert_eq!(0, line_number);
        assert_eq!(expected, block);
        assert!(blocker.next().is_none());
    }

    #[test]
    fn item() {
        let mut blocker = Blocker::new("- Item\n- Item".lines().enumerate());

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Item(Item {
            body: vec![(0, "Item")],
            level: 1,
        });
        assert_eq!(line_number, 0);
        assert_eq!(expected, block);

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Item(Item {
            body: vec![(1, "Item")],
            level: 1,
        });
        assert_eq!(line_number, 1);
        assert_eq!(expected, block);

        assert!(blocker.next().is_none());
    }

    #[test]
    fn item_with_text_block() {
        let mut blocker = Blocker::new("- item\n  more of item".lines().enumerate());

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Item(Item {
            level: 1,
            body: vec![(0, "item"), (1, "more of item")],
        });
        assert_eq!(line_number, 0);
        assert_eq!(expected, block);

        assert!(blocker.next().is_none());
    }

    #[test]
    fn text() {
        let mut blocker = Blocker::new("This should be a\nblock of text".lines().enumerate());

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Text(Text {
            body: vec![(0, "This should be a"), (1, "block of text")],
        });

        assert_eq!(line_number, 0);
        assert_eq!(expected, block);

        assert!(blocker.next().is_none());
    }

    #[test]
    fn indented_text() {
        let mut blocker = Blocker::new("This should be\n an error".lines().enumerate());

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Text(Text {
            body: vec![(0, "This should be")],
        });
        assert_eq!(line_number, 0);
        assert_eq!(expected, block);

        let (line_number, result) = blocker.next().unwrap();
        let expected = Error::IndentedTextBlock;
        assert_eq!(line_number, 1);

        if let Err(gotten) = result {
            assert_eq!(expected, gotten);
        } else {
            panic!(
                "Expected: {:?}, got: {:?}",
                Err::<Block<usize>, _>(expected),
                result
            );
        }

        assert!(blocker.next().is_none());
    }

    #[test]
    fn quote() {
        let mut blocker = Blocker::new("> Quote 1\n> Quote 2".lines().enumerate());

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();

        let expected = Block::Quote(Quote {
            body: vec![(0, "Quote 1"), (1, "Quote 2")],
        });
        assert_eq!(line_number, 0);
        assert_eq!(expected, block);

        assert!(blocker.next().is_none());
    }

    #[test]
    fn heading_quote_quote() {
        let mut blocker = Blocker::new("# Heading\n> Quote 1\n> Quote 1\n\n> Quote 2\n> Quote 2".lines().enumerate());

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Heading(Heading {
            level: 1,
            body: vec![(0, "Heading")],
        });
        assert_eq!(0, line_number);
        assert_eq!(expected, block);

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Quote(Quote {
            body: vec![(1, "Quote 1"), (2, "Quote 1")],
        });
        assert_eq!(1, line_number);
        assert_eq!(expected, block);

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Quote(Quote {
            body: vec![(4, "Quote 2"), (5, "Quote 2")],
        });
        assert_eq!(4, line_number);
        assert_eq!(expected, block);

        assert!(blocker.next().is_none());
    }

    #[test]
    fn heading_list_indented_text() {
        let mut blocker = Blocker::new("# Heading\n- Item\n  With more Item\n\n  Indented Text".lines().enumerate());

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Heading(Heading {
            level: 1,
            body: vec![(0, "Heading")],
        });
        assert_eq!(0, line_number);
        assert_eq!(expected, block);

        let (line_number, block) = blocker.next().unwrap();
        let block = block.unwrap();
        let expected = Block::Item(Item {
            level: 1,
            body: vec![(1, "Item"), (2, "With more Item")],
        });
        assert_eq!(1, line_number);
        assert_eq!(expected, block);

        let (line_number, result) = blocker.next().unwrap();
        let expected = Error::IndentedTextBlock;

        assert_eq!(4, line_number);

        if let Err(gotten) = result {
            assert_eq!(expected, gotten);
        } else {
            panic!(
                "Expected: {:?}, got: {:?}",
                Err::<Block<usize>, _>(expected),
                result
            );
        }
    }
}
