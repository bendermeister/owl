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
            assert_eq!(s.lines().count(), 1);
            let mut s = s;
            let mut indent = 0;
            while !s.is_empty() && s.chars().next().unwrap().is_whitespace() {
                if s.chars().next().unwrap() != ' ' {
                    return Err(Error::NonSpaceIndent);
                }
                indent += 1;
                s = &s[1..];
            }
            Ok(LineIndented {body: s.trim(), indent})
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
            let expected = LineIndented{indent: 0, body: "Indented String"};
            assert_eq!(expected, str);
        }

        #[test]
        fn indented() {
            let str: LineIndented = "    Indented String".try_into().unwrap();
            let expected = LineIndented{indent: 4, body: "Indented String"};
            assert_eq!(expected, str);
        }

        #[test]
        fn indented_with_non_whitespace() {
            let result: Result<LineIndented, _> = "\tIndented String".try_into();
            let expected = Error::NonSpaceIndent;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected: {:?}, got: {:?}", Err::<LineIndented, _>(expected), result);
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
            let mut  s = s;
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

            Ok(Self::Item{body: s.trim(), level})
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

            Ok(Self::Quote{body: s.trim()})
        }

        fn parse_text(s: LineIndented<'a>) -> Result<Self, Error> {
            assert!(!s.is_empty());
            Ok(Line::Text{body: s.trim(), indent: s.indent})
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
            let result: Result<Line, _>  = " # Hello World".try_into();
            let expected = Error::HeadingWithLeadingSpace;
            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected: {:?}, gotten: {:?}", Err::<Line, Error>(expected), result);
            }
        }

        #[test]
        fn item_level_1() {
            let line: Line = "- Some Item".try_into().unwrap();
            let expected = Line::Item{ body: "Some Item", level: 1};
            assert_eq!(expected, line);
        }

        #[test]
        fn item_with_odd_leading_space() {
            let result: Result<Line, _> = "   - Some Item".try_into();
            let expected = Error::ItemWithOddLeadingSpace;
            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected: {:?}, got: {:?}", Err::<Line, _>(expected), result);
            }
        }

        #[test]
        fn item_with_non_space_indent() {
            let result: Result<Line, _> = "\t- Some Item".try_into();
            let expected = Error::NonSpaceIndent;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected: {:?}, got: {:?}", Err::<Line, _>(expected), result);
            }
        }

        #[test]
        fn quote() {
            let line: Line = "> Some Quote".try_into().unwrap();
            let expected = Line::Quote{ body: "Some Quote" };
            assert_eq!(expected, line);
        }

        #[test]
        fn quote_with_leading_space() {
            let result: Result<Line, _> = " > Quote with Leading Space".try_into();
            let expected = Error::QuoteWithLeadingSpace;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected: {:?}, got: {:?}", Err::<Line, _>(expected), result);
            }
        }

        #[test]
        fn text() {
            let line: Line = "This should be some Text".try_into().unwrap();
            let expected = Line::Text{ body: "This should be some Text", indent: 0};
            assert_eq!(expected, line);
        }

        #[test]
        fn text_with_indent() {
            let line: Line = "  This should be some Text".try_into().unwrap();
            let expected = Line::Text{ body: "This should be some Text", indent: 2};
            assert_eq!(expected, line);
        }

        #[test]
        fn text_with_non_space_indent() {
            let result: Result<Line, _> = "\tThis is an Error".try_into();
            let expected = Error::NonSpaceIndent;

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected: {:?}, got: {:?}", Err::<Line, _>(expected), result);
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
                    Err(error) => return Err(Error{line_number: line_number + 1, error})
                }
            }

            Ok(Self{lines, index: 0})
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

            Some(Item{line_number: index + 1, line: self.lines[index]})
        }
    }

    #[cfg(test)]
    mod test {

        use super::*;

        #[test]
        fn liner_full_1() {
            let liner: Liner = "# Some Heading\n- Some Item\n  - Some other Item\n> Quote\nText".try_into().unwrap();

            let lines: Vec<_> = liner.collect();

            assert_eq!(5, lines.len());

            let expected = Item{line_number: 1, line: line::Line::Heading{level: 1, body: "Some Heading"}};
            assert_eq!(expected, lines[0]);

            let expected = Item{line_number: 2, line: line::Line::Item{level: 1, body: "Some Item"}};
            assert_eq!(expected, lines[1]);

            let expected = Item{line_number: 3, line: line::Line::Item{level: 2, body: "Some other Item"}};
            assert_eq!(expected, lines[2]);

            let expected = Item{line_number: 4, line: line::Line::Quote{body: "Quote"}};
            assert_eq!(expected, lines[3]);

            let expected = Item{line_number: 5, line: line::Line::Text{indent: 0, body: "Text"}};
            assert_eq!(expected, lines[4]);
        }

        #[test]
        fn heading_with_leading_whitespace() {
            let result: Result<Liner, _> = "# Some Heading\n ## Some other Heading".try_into();
            let expected = Error{line_number: 2, error: line::Error::HeadingWithLeadingSpace};

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got: {:?}", Err::<Liner, _>(expected), result);
            }
        }

        #[test]
        fn item_with_odd_leading_space() {
            let result: Result<Liner, _> = "# Some Heading\n- First Item\n - Seond Item".try_into();
            let expected = Error{line_number: 3, error: line::Error::ItemWithOddLeadingSpace};

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got {:?}", Err::<Liner, _>(expected), result);
            }
        }
        
        #[test]
        fn item_with_non_space_indent() {
            let result: Result<Liner, _> = "# Some Heading\n\t- Item".try_into();
            let expected = Error{line_number: 2, error: line::Error::NonSpaceIndent};

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got {:?}", Err::<Liner, _>(expected), result);
            }
        }

        #[test]
        fn quote_with_leading_space() {
            let result: Result<Liner, _> = "# Some Heading\n > Quote".try_into();
            let expected = Error{line_number: 2, error: line::Error::QuoteWithLeadingSpace};

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got {:?}", Err::<Liner, _>(expected), result);
            }
        }

        #[test]
        fn text_with_non_space_indent() {
            let result: Result<Liner, _> = "# Some Heading\n> Quote\n\tText Text Text".try_into();
            let expected = Error{line_number: 3, error: line::Error::NonSpaceIndent};

            if let Err(gotten) = result {
                assert_eq!(expected, gotten);
            } else {
                panic!("Expected {:?}, got {:?}", Err::<Liner, _>(expected), result);
            }
        }
    }
}

mod blocker {
}
