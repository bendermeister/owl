mod line_indented {
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct LineIndented<'a> {
        body: &'a str,
        pub indent: u8,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Error {
        NonSpaceIndent,
    }

    impl<'a> Into<&'a str> for LineIndented<'a> {
        fn into(self) -> &'a str {
            self.body
        }
    }

    impl<'a> TryFrom<&'a str> for LineIndented<'a> {
        type Error = Error;

        /// # Asserts
        /// `s` is a single line
        fn try_from(s: &'a str) -> Result<Self, Self::Error> {
            assert!(s.is_empty() || s.lines().count() == 1);
            let mut s = s;
            let mut indent = 0;
            while !s.is_empty() && s.chars().next().unwrap().is_whitespace() {
                if s.chars().next().unwrap() != ' ' {
                    return Err(Error::NonSpaceIndent);
                }
                indent += 1;
                s = &s[1..];
            }
            Ok(LineIndented {
                body: s.trim(),
                indent,
            })
        }
    }

    impl<'a> LineIndented<'a> {
        pub fn chop(&mut self) {
            self.body = &self.body[1..];
        }

        pub fn peek(&self) -> char {
            assert!(!self.body.is_empty());
            self.body.chars().next().unwrap()
        }
    }

    impl<'a> std::ops::Deref for LineIndented<'a> {
        type Target = &'a str;

        fn deref(&self) -> &Self::Target {
            &self.body
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn not_indented() {
            let str: LineIndented = "Indented String".try_into().unwrap();
            let expected = LineIndented {
                indent: 0,
                body: "Indented String",
            };
            assert_eq!(expected, str);
        }

        #[test]
        fn indented() {
            let str: LineIndented = "    Indented String".try_into().unwrap();
            let expected = LineIndented {
                indent: 4,
                body: "Indented String",
            };
            assert_eq!(expected, str);
        }

        #[test]
        fn indented_with_non_whitespace() {
            let result: Result<LineIndented, _> = "\tIndented String".try_into();
            let expected = Error::NonSpaceIndent;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected: {:?}, got: {:?}",
                    Err::<LineIndented, _>(expected),
                    result
                );
            }
        }
    }
}

mod line {
    use super::line_indented;
    use super::line_indented::LineIndented;
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Line<'a> {
        Heading { body: &'a str, level: u8 },
        Item { body: &'a str, level: u8 },
        Text { body: &'a str, indent: u8 },
        Quote { body: &'a str },
        Break,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Error {
        // General Errors
        NonSpaceIndent,

        // Heading Errors
        HeadingWithLeadingSpace,

        // Item Errors
        ItemWithOddLeadingSpace,

        // Quote Errors
        QuoteWithLeadingSpace,
    }

    impl From<line_indented::Error> for Error {
        fn from(value: line_indented::Error) -> Self {
            match value {
                line_indented::Error::NonSpaceIndent => Error::NonSpaceIndent,
            }
        }
    }

    impl<'a> Line<'a> {
        /// Parse a single line into a heading
        ///
        /// # Asserts
        /// `s` is not empty
        /// `s` starts with whitespace or an '#'
        fn parse_heading(s: LineIndented<'a>) -> Result<Self, Error> {
            let mut s = s;
            assert!(!s.is_empty());
            if s.indent > 0 {
                return Err(Error::HeadingWithLeadingSpace);
            }
            assert_eq!(s.peek(), '#');

            let mut level = 0;
            while !s.is_empty() && s.peek() == '#' {
                level += 1;
                s.chop();
            }

            Ok(Self::Heading {
                body: s.trim(),
                level,
            })
        }

        /// Parse a single line ito an item
        ///
        /// # Asserts
        /// `s` is not empty()
        /// `s` is a single line
        /// `s` starts with zero or more (`' '` or `'\t'`) followed by a `'-'`
        fn parse_item(s: LineIndented<'a>) -> Result<Self, Error> {
            let mut s = s;
            assert!(!s.is_empty());
            let level = s.indent;

            if level % 2 == 1 {
                return Err(Error::ItemWithOddLeadingSpace);
            }
            let level = level / 2 + 1;

            assert!(!s.is_empty());
            assert_eq!(s.peek(), '-');
            s.chop();

            Ok(Self::Item {
                body: s.trim(),
                level,
            })
        }

        /// Parse a single line into a Quote
        ///
        /// # Asserts
        /// - `s` is not empty
        /// - `s` starts with zero or more whitespace chars followed by a `'>'`
        fn parse_quote(s: LineIndented<'a>) -> Result<Self, Error> {
            let mut s = s;
            assert!(!s.is_empty());

            if s.indent > 0 {
                return Err(Error::QuoteWithLeadingSpace);
            }

            assert_eq!(s.peek(), '>');
            s.chop();

            Ok(Self::Quote { body: s.trim() })
        }

        fn parse_text(s: LineIndented<'a>) -> Result<Self, Error> {
            assert!(!s.is_empty());
            Ok(Line::Text {
                body: s.trim(),
                indent: s.indent,
            })
        }
    }

    impl<'a> TryFrom<&'a str> for Line<'a> {
        type Error = Error;

        fn try_from(s: &'a str) -> Result<Self, Self::Error> {
            let line: LineIndented = match s.try_into() {
                Ok(line) => line,
                Err(err) => return Err(err.into()),
            };

            if line.is_empty() {
                return Ok(Line::Break);
            }

            match line.peek() {
                '#' => Self::parse_heading(line),
                '-' => Self::parse_item(line),
                '>' => Self::parse_quote(line),
                _ => Self::parse_text(line),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn heading() {
            let line: Line = "# Hello World".try_into().unwrap();
            let expected = Line::Heading {
                body: "Hello World".into(),
                level: 1,
            };
            assert_eq!(expected, line);
        }

        #[test]
        fn heading_with_leading_whitespace() {
            let result: Result<Line, _> = " # Hello World".try_into();
            let expected = Error::HeadingWithLeadingSpace;
            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected: {:?}, gotten: {:?}",
                    Err::<Line, Error>(expected),
                    result
                );
            }
        }

        #[test]
        fn item_level_1() {
            let line: Line = "- Some Item".try_into().unwrap();
            let expected = Line::Item {
                body: "Some Item",
                level: 1,
            };
            assert_eq!(expected, line);
        }

        #[test]
        fn item_with_odd_leading_space() {
            let result: Result<Line, _> = "   - Some Item".try_into();
            let expected = Error::ItemWithOddLeadingSpace;
            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected: {:?}, got: {:?}",
                    Err::<Line, _>(expected),
                    result
                );
            }
        }

        #[test]
        fn item_with_non_space_indent() {
            let result: Result<Line, _> = "\t- Some Item".try_into();
            let expected = Error::NonSpaceIndent;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected: {:?}, got: {:?}",
                    Err::<Line, _>(expected),
                    result
                );
            }
        }

        #[test]
        fn quote() {
            let line: Line = "> Some Quote".try_into().unwrap();
            let expected = Line::Quote { body: "Some Quote" };
            assert_eq!(expected, line);
        }

        #[test]
        fn quote_with_leading_space() {
            let result: Result<Line, _> = " > Quote with Leading Space".try_into();
            let expected = Error::QuoteWithLeadingSpace;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected: {:?}, got: {:?}",
                    Err::<Line, _>(expected),
                    result
                );
            }
        }

        #[test]
        fn text() {
            let line: Line = "This should be some Text".try_into().unwrap();
            let expected = Line::Text {
                body: "This should be some Text",
                indent: 0,
            };
            assert_eq!(expected, line);
        }

        #[test]
        fn text_with_indent() {
            let line: Line = "  This should be some Text".try_into().unwrap();
            let expected = Line::Text {
                body: "This should be some Text",
                indent: 2,
            };
            assert_eq!(expected, line);
        }

        #[test]
        fn text_with_non_space_indent() {
            let result: Result<Line, _> = "\tThis is an Error".try_into();
            let expected = Error::NonSpaceIndent;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected: {:?}, got: {:?}",
                    Err::<Line, _>(expected),
                    result
                );
            }
        }
    }
}

mod liner {
    use super::line;
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Liner<'a> {
        lines: Vec<line::Line<'a>>,
        index: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Item<'a> {
        pub line_number: usize,
        pub line: line::Line<'a>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Error {
        pub line_number: usize,
        pub error: line::Error,
    }

    impl<'a> TryFrom<&'a str> for Liner<'a> {
        type Error = Error;

        fn try_from(s: &'a str) -> Result<Self, Self::Error> {
            let mut lines = vec![];
            for (line_number, line) in s.lines().enumerate() {
                match line.try_into() {
                    Ok(line) => lines.push(line),
                    Err(error) => {
                        return Err(Error {
                            line_number: line_number + 1,
                            error,
                        })
                    }
                }
            }

            Ok(Self { lines, index: 0 })
        }
    }

    impl<'a> Iterator for Liner<'a> {
        type Item = Item<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.lines.is_empty() {
                return None;
            }

            if self.index >= self.lines.len() {
                return None;
            }

            let index = self.index;
            self.index += 1;

            Some(Item {
                line_number: index + 1,
                line: self.lines[index],
            })
        }
    }

    #[cfg(test)]
    mod test {

        use super::*;

        #[test]
        fn liner_full_1() {
            let liner: Liner = "# Some Heading\n- Some Item\n  - Some other Item\n> Quote\nText"
                .try_into()
                .unwrap();

            let lines: Vec<_> = liner.collect();

            assert_eq!(5, lines.len());

            let expected = Item {
                line_number: 1,
                line: line::Line::Heading {
                    level: 1,
                    body: "Some Heading",
                },
            };
            assert_eq!(expected, lines[0]);

            let expected = Item {
                line_number: 2,
                line: line::Line::Item {
                    level: 1,
                    body: "Some Item",
                },
            };
            assert_eq!(expected, lines[1]);

            let expected = Item {
                line_number: 3,
                line: line::Line::Item {
                    level: 2,
                    body: "Some other Item",
                },
            };
            assert_eq!(expected, lines[2]);

            let expected = Item {
                line_number: 4,
                line: line::Line::Quote { body: "Quote" },
            };
            assert_eq!(expected, lines[3]);

            let expected = Item {
                line_number: 5,
                line: line::Line::Text {
                    indent: 0,
                    body: "Text",
                },
            };
            assert_eq!(expected, lines[4]);
        }

        #[test]
        fn heading_with_leading_whitespace() {
            let result: Result<Liner, _> = "# Some Heading\n ## Some other Heading".try_into();
            let expected = Error {
                line_number: 2,
                error: line::Error::HeadingWithLeadingSpace,
            };

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected {:?}, got: {:?}",
                    Err::<Liner, _>(expected),
                    result
                );
            }
        }

        #[test]
        fn item_with_odd_leading_space() {
            let result: Result<Liner, _> = "# Some Heading\n- First Item\n - Seond Item".try_into();
            let expected = Error {
                line_number: 3,
                error: line::Error::ItemWithOddLeadingSpace,
            };

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got {:?}", Err::<Liner, _>(expected), result);
            }
        }

        #[test]
        fn item_with_non_space_indent() {
            let result: Result<Liner, _> = "# Some Heading\n\t- Item".try_into();
            let expected = Error {
                line_number: 2,
                error: line::Error::NonSpaceIndent,
            };

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got {:?}", Err::<Liner, _>(expected), result);
            }
        }

        #[test]
        fn quote_with_leading_space() {
            let result: Result<Liner, _> = "# Some Heading\n > Quote".try_into();
            let expected = Error {
                line_number: 2,
                error: line::Error::QuoteWithLeadingSpace,
            };

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got {:?}", Err::<Liner, _>(expected), result);
            }
        }

        #[test]
        fn text_with_non_space_indent() {
            let result: Result<Liner, _> = "# Some Heading\n> Quote\n\tText Text Text".try_into();
            let expected = Error {
                line_number: 3,
                error: line::Error::NonSpaceIndent,
            };

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got {:?}", Err::<Liner, _>(expected), result);
            }
        }
    }
}

mod blocker {
    use crate::line;
    use crate::liner;
    use line::Line;
    use liner::Liner;
    use std::iter::Peekable;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Block<'a> {
        Heading {
            line_number: usize,
            level: u8,
            body: &'a str,
        },
        Text {
            line_number: usize,
            body: Vec<&'a str>,
        },
        Item {
            line_number: usize,
            level: u8,
            body: Vec<&'a str>,
        },
        Quote {
            line_number: usize,
            body: Vec<&'a str>,
        },
    }

    impl<'a> Block<'a> {
        fn wants(&self, item: &liner::Item) -> bool {
            match (self, item.line) {
                (
                    Block::Item {
                        line_number: _,
                        level,
                        body: _,
                    },
                    Line::Text { body: _, indent },
                ) if indent == 2 * level => true,
                (
                    Block::Text {
                        line_number: _,
                        body: _,
                    },
                    Line::Text { body: _, indent: 0 },
                ) => true,
                (
                    Block::Quote {
                        line_number: _,
                        body: _,
                    },
                    Line::Quote { body: _ },
                ) => true,
                _ => false,
            }
        }

        fn feed_item(&mut self, line: line::Line<'a>) {
            if let Block::Item {
                line_number: _,
                level,
                body,
            } = self
            {
                if let Line::Text {
                    body: line_body,
                    indent,
                } = line
                {
                    assert_eq!(indent, 2 * *level);
                    body.push(line_body);
                } else {
                    panic!("This function must be called with a line::Line::Text");
                }
            } else {
                panic!("This function must be called on a Block::Item");
            }
        }

        fn feed_text(&mut self, text: Line<'a>) {
            if let Block::Text {
                line_number: _,
                body,
            } = self
            {
                if let Line::Text {
                    body: line_body,
                    indent,
                } = text
                {
                    assert_eq!(indent, 0);
                    body.push(line_body);
                } else {
                    panic!("Only Line::Text can be feeded to Block::Text");
                }
            } else {
                panic!("This function may only be called with Block::Text");
            }
        }

        fn feed_quote(&mut self, line: Line<'a>) {
            if let Block::Quote{line_number: _, body} = self {
                if let Line::Quote { body: line_body } = line {
                    body.push(line_body);
                } else {
                    panic!("Only Line::Quote ca be fed to Block::Quote");
                }
            } else {
                panic!("This function may only be called on Block::Quote");
            }
        }

        fn feed(&mut self, item: liner::Item<'a>) {
            match self {
                Block::Heading {
                    line_number: _,
                    level: _,
                    body: _,
                } => panic!("{:?} should not be fed", self),
                Block::Text {
                    line_number: _,
                    body: _,
                } => self.feed_text(item.line),
                Block::Item {
                    line_number: _,
                    level: _,
                    body: _,
                } => self.feed_item(item.line),
                Block::Quote {
                    line_number: _,
                    body: _,
                } => self.feed_quote(item.line),
            }
        }

        fn new_heading(line_number: usize, level: u8, body: &'a str) -> Result<Block<'a>, Error> {
            Ok(Block::Heading {
                line_number,
                body: body.into(),
                level,
            })
        }

        fn new_item(line_number: usize, level: u8, body: &'a str) -> Result<Block<'a>, Error> {
            Ok(Block::Item {
                line_number,
                level,
                body: vec![body],
            })
        }

        fn new_text(line_number: usize, indent: u8, body: &'a str) -> Result<Block<'a>, Error> {
            if indent != 0 {
                return Err(Error::IndentedTextBlock { line_number });
            }
            Ok(Block::Text {
                line_number,
                body: vec![body],
            })
        }

        fn new_quote(line_number: usize, body: &'a str) -> Result<Block<'a>, Error> {
            Ok(Block::Quote {
                line_number,
                body: vec![body],
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Error {
        IndentedTextBlock { line_number: usize },
    }

    impl<'a> TryFrom<liner::Item<'a>> for Block<'a> {
        type Error = Error;

        fn try_from(item: liner::Item<'a>) -> Result<Self, Self::Error> {
            let line_number = item.line_number;
            let line = item.line;

            match line {
                Line::Heading { body, level } => Block::new_heading(line_number, level, body),
                Line::Item { body, level } => Block::new_item(line_number, level, body),
                Line::Text { body, indent } => Block::new_text(line_number, indent, body),
                Line::Quote { body } => Block::new_quote(line_number, body),
                Line::Break => panic!("unreachable"),
            }
        }
    }

    pub struct Blocker<'a> {
        liner: Peekable<Liner<'a>>,
    }

    impl<'a> Blocker<'a> {
        pub fn new(liner: Liner<'a>) -> Blocker<'a> {
            Blocker {
                liner: liner.peekable(),
            }
        }
    }

    impl<'a> Iterator for Blocker<'a> {
        type Item = Result<Block<'a>, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            let line = match self.liner.next() {
                Some(line) => line,
                None => return None,
            };

            if let Line::Break = line.line {
                return self.next();
            }

            let mut block: Block = match line.try_into() {
                Ok(block) => block,
                Err(err) => return Some(Err(err)),
            };

            while let Some(item) = self.liner.peek() {
                if !block.wants(item) {
                    break;
                }
                block.feed(self.liner.next().unwrap());
            }

            Some(Ok(block))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn heading() {
            let liner: liner::Liner = "# Heading".try_into().unwrap();
            let mut blocker = Blocker::new(liner);
            let block = blocker.next().unwrap().unwrap();
            let expected = Block::Heading {
                line_number: 1,
                level: 1,
                body: "Heading".into(),
            };
            assert_eq!(expected, block);
        }

        #[test]
        fn item() {
            let liner: Liner = "- Item\n- Item".try_into().unwrap();
            let mut blocker = Blocker::new(liner);

            let block = blocker.next().unwrap().unwrap();
            let expected = Block::Item {
                line_number: 1,
                body: vec!["Item"],
                level: 1,
            };
            assert_eq!(expected, block);

            let block = blocker.next().unwrap().unwrap();
            let expected = Block::Item {
                line_number: 2,
                body: vec!["Item"],
                level: 1,
            };
            assert_eq!(expected, block);
        }

        #[test]
        fn item_with_text_block() {
            let liner: Liner = "- item\n  more of item".try_into().unwrap();
            let mut blocker = Blocker::new(liner);

            let block = blocker.next().unwrap().unwrap();
            let expected = Block::Item {
                line_number: 1,
                level: 1,
                body: vec!["item", "more of item"],
            };
            assert_eq!(expected, block);
        }

        #[test]
        fn text() {
            let liner = "This should be a\nblock of text".try_into().unwrap();
            let mut blocker = Blocker::new(liner);

            let block = blocker.next().unwrap().unwrap();
            let expected = Block::Text {
                line_number: 1,
                body: vec!["This should be a", "block of text"],
            };
            assert_eq!(expected, block);
        }

        #[test]
        fn indented_text() {
            let liner = "This should be\n an error".try_into().unwrap();
            let mut blocker = Blocker::new(liner);

            let block = blocker.next().unwrap().unwrap();
            let expected = Block::Text {
                line_number: 1,
                body: vec!["This should be"],
            };
            assert_eq!(expected, block);

            let result = blocker.next().unwrap();
            let expected = Error::IndentedTextBlock { line_number: 2 };

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected: {:?}, got: {:?}",
                    Err::<Block, _>(expected),
                    result
                );
            }
        }

        #[test]
        fn quote() {
            let liner = "> Quote 1\n> Quote 2".try_into().unwrap();
            let mut blocker = Blocker::new(liner);

            let block = blocker.next().unwrap().unwrap();
            let expected = Block::Quote {
                line_number: 1,
                body: vec!["Quote 1", "Quote 2"],
            };
            assert_eq!(expected, block);
            assert!(blocker.next().is_none());
        }

        #[test]
        fn heading_quote_quote() {
            let liner = "# Heading\n> Quote 1\n> Quote 1\n\n> Quote 2\n> Quote 2".try_into().unwrap();
            let mut blocker = Blocker::new(liner);

            let got = blocker.next().unwrap().unwrap();
            let expected = Block::Heading{line_number: 1, level: 1, body: "Heading"};
            assert_eq!(expected, got);

            let got = blocker.next().unwrap().unwrap();
            let expected = Block::Quote{line_number: 2, body: vec!["Quote 1", "Quote 1"]};
            assert_eq!(expected, got);
        }

        #[test]
        fn heading_list_indented_text() {
            let liner = "# Heading\n- Item\n  With more Item\n\n  Indented Text".try_into().unwrap();
            let mut blocker = Blocker::new(liner);

            let got = blocker.next().unwrap().unwrap();
            let expected = Block::Heading{line_number: 1, level: 1, body: "Heading"};
            assert_eq!(expected, got);

            let got = blocker.next().unwrap().unwrap();
            let expected = Block::Item{line_number: 2, level: 1, body: vec!["Item", "With more Item"]};
            assert_eq!(expected, got);

            let got = blocker.next().unwrap();
            let expected = Error::IndentedTextBlock{line_number: 5};

            if let Err(got) = got {
                assert_eq!(expected, got);
            } else {
                panic!("Expected: {:?}, got: {:?}", Err::<Block, _>(expected), got);
            }
        }
    }
}
