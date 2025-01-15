use super::Error;

mod indented {
    use super::Error;
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Indented<'a> {
        body: &'a str,
        pub indent: u8,
    }

    impl<'a> Into<&'a str> for Indented<'a> {
        fn into(self) -> &'a str {
            self.body
        }
    }

    impl<'a> TryFrom<&'a str> for Indented<'a> {
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
            Ok(Indented {
                body: s.trim(),
                indent,
            })
        }
    }

    impl<'a> Indented<'a> {
        pub fn chop(&mut self) {
            self.body = &self.body[1..];
        }

        pub fn peek(&self) -> char {
            assert!(!self.body.is_empty());
            self.body.chars().next().unwrap()
        }
    }

    impl<'a> std::ops::Deref for Indented<'a> {
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
            let str: Indented = "Indented String".try_into().unwrap();
            let expected = Indented {
                indent: 0,
                body: "Indented String",
            };
            assert_eq!(expected, str);
        }

        #[test]
        fn indented() {
            let str: Indented = "    Indented String".try_into().unwrap();
            let expected = Indented {
                indent: 4,
                body: "Indented String",
            };
            assert_eq!(expected, str);
        }

        #[test]
        fn indented_with_non_whitespace() {
            let result: Result<Indented, _> = "\tIndented String".try_into();
            let expected = Error::NonSpaceIndent;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!(
                    "Expected: {:?}, got: {:?}",
                    Err::<Indented, _>(expected),
                    result
                );
            }
        }
    }
}

use indented::Indented;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Heading<'a> {
    pub body: &'a str,
    pub level: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Item<'a> {
    pub body: &'a str,
    pub level: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Text<'a> {
    pub body: &'a str,
    pub indent: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Quote<'a> {
    pub body: &'a str,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Line<'a> {
    Heading(Heading<'a>),
    Item(Item<'a>),
    Text(Text<'a>),
    Quote(Quote<'a>),
    Break,
}

impl<'a> Line<'a> {
    /// Parse a single line into a heading
    ///
    /// # Asserts
    /// `s` is not empty
    /// `s` starts with whitespace or an '#'
    fn parse_heading(s: Indented<'a>) -> Result<Self, Error> {
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

        Ok(Self::Heading(Heading {
            body: s.trim(),
            level,
        }))
    }

    /// Parse a single line ito an item
    ///
    /// # Asserts
    /// `s` is not empty()
    /// `s` is a single line
    /// `s` starts with zero or more (`' '` or `'\t'`) followed by a `'-'`
    fn parse_item(s: Indented<'a>) -> Result<Self, Error> {
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

        Ok(Self::Item(Item {
            body: s.trim(),
            level,
        }))
    }

    /// Parse a single line into a Quote
    ///
    /// # Asserts
    /// - `s` is not empty
    /// - `s` starts with zero or more whitespace chars followed by a `'>'`
    fn parse_quote(s: Indented<'a>) -> Result<Self, Error> {
        let mut s = s;
        assert!(!s.is_empty());

        if s.indent > 0 {
            return Err(Error::QuoteWithLeadingSpace);
        }

        assert_eq!(s.peek(), '>');
        s.chop();

        Ok(Self::Quote(Quote { body: s.trim() }))
    }

    fn parse_text(s: Indented<'a>) -> Result<Self, Error> {
        assert!(!s.is_empty());
        Ok(Self::Text(Text {
            body: s.trim(),
            indent: s.indent,
        }))
    }
}

impl<'a> TryFrom<&'a str> for Line<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let line: Indented = match s.try_into() {
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
        let expected = Line::Heading(Heading {
            body: "Hello World".into(),
            level: 1,
        });
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
        let expected = Line::Item(Item {
            body: "Some Item",
            level: 1,
        });
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
        let expected = Line::Quote(Quote { body: "Some Quote" });
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
        let expected = Line::Text(Text {
            body: "This should be some Text",
            indent: 0,
        });
        assert_eq!(expected, line);
    }

    #[test]
    fn text_with_indent() {
        let line: Line = "  This should be some Text".try_into().unwrap();
        let expected = Line::Text(Text {
            body: "This should be some Text",
            indent: 2,
        });
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
