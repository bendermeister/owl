use std::str::FromStr;

#[derive(Debug)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    LeadingWhiteSpaceBeforeHeading {
        line_number: usize,
    },
    UnexpectedHeadingLevel {
        line_number: usize,
        expected: u8,
        got: u8,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paragraph {
    pub body: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Block {
    Heading { level: u8, body: String },
    Paragraph { body: String },
}

impl Block {
    pub fn default_heading() -> Block {
        Block::Heading {
            level: 1,
            body: "Hello World".into(),
        }
    }
}

struct BlockParser<'a> {
    input: &'a str,
    line_number: usize,
    heading_level: u8,
}

impl<'a> BlockParser<'a> {
    /// Remove first character from `self.input` and update `self.line_number` when necessary
    ///
    /// # Asserts
    /// `!self.input.is_empty()`
    fn chop(&mut self) {
        assert!(!self.input.is_empty());
        if self.peek() == '\n' {
            self.line_number += 1;
        }
        self.input = &self.input[1..];
    }

    /// find first non whitespace character in `self.input`
    fn find_non_whitespace(&self) -> Option<usize> {
        self.input
            .chars()
            .enumerate()
            .filter(|(_, c)| !c.is_whitespace())
            .map(|(i, _)| i)
            .next()
    }

    /// Chop `self.input` at `len` and return the chopped part and update `self.line_number` when
    /// necessary
    ///
    /// # Asserts
    /// `len <= self.input.len()`
    fn chop_len(&mut self, len: usize) -> &str {
        assert!(len <= self.input.len());
        let chopped = &self.input[..len];
        self.input = &self.input[len..];
        self.line_number += chopped.chars().filter(|c| *c == '\n').count();
        chopped
    }

    /// get first `char`
    ///
    /// # Asserts
    /// `!self.input.is_empty()`
    fn peek(&self) -> char {
        self.peek_at(0)
    }

    /// get `char` at position `index`
    ///
    /// # Asserts
    /// `self.input.len() > index`
    fn peek_at(&self, index: usize) -> char {
        assert!(self.input.len() > index);
        self.input.chars().nth(index).unwrap()
    }

    /// Find the index of the next newline character
    fn find_newline(&self) -> Option<usize> {
        let index = self
            .input
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '\n')
            .next();
        match index {
            Some((index, _)) => Some(index),
            None => None,
        }
    }

    /// Consumes input until it generates either an `Block::Heading` or an `ParsingError`
    ///
    /// # Asserts
    /// that `self.input` is not empty
    fn parse_header(&mut self) -> Result<Block, ParsingError> {
        assert!(!self.input.is_empty());
        if self.peek().is_whitespace() {
            return Err(ParsingError::LeadingWhiteSpaceBeforeHeading {
                line_number: self.line_number,
            });
        }

        let mut level = 0;
        // Chop away level indicators
        while !self.input.is_empty() && self.peek() == '#' {
            level += 1;
            self.chop();
        }

        if level > self.heading_level && level != self.heading_level + 1 {
            return Err(ParsingError::UnexpectedHeadingLevel {
                line_number: self.line_number,
                expected: self.heading_level + 1,
                got: level,
            });
        }
        self.heading_level = level;

        let body = match self.find_newline() {
            Some(index) => {
                let body = self.chop_len(index).trim();
                let body = String::from(body);
                // chop away newline
                self.chop();

                body
            }
            None => String::from(self.chop_len(self.input.len()).trim()),
        };

        Ok(Block::Heading{ level, body })
    }

    fn new(input: &'a str) -> BlockParser<'a> {
        BlockParser {
            line_number: 1,
            heading_level: 0,
            input,
        }
    }
}

impl<'a> Iterator for BlockParser<'a> {
    type Item = Result<Block, ParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let index = match self.find_non_whitespace() {
            Some(index) => index,
            None => return None,
        };

        match self.peek_at(index) {
            '#' => Some(self.parse_header()),
            _ => todo!(),
        }
    }
}

impl FromStr for Document {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parser = BlockParser::new(input);
        let mut doc = Document { blocks: vec![] };

        for block in parser {
            match block {
                Ok(block) => doc.blocks.push(block),
                Err(err) => return Err(err),
            }
        }

        Ok(doc)
    }
}
