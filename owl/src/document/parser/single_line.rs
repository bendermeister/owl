use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleLine<'a> {
    body: &'a str,
}

impl<'a> From<&'a str> for SingleLine<'a> {
    fn from(line: &'a str) -> Self {
        let line_count = line.lines().count();
        assert!(line_count == 0 || line_count == 1);
        SingleLine{
            body: line
        }
    }
}

impl<'a> SingleLine<'a> {
    pub fn get_indent(&self) -> usize {
        self.chars().take_while(|c| *c == ' ').count()
    }

    pub fn peek(&self) -> Option<char> {
        self.chars().next()
    }

    pub fn chop_left_whitespace(&mut self) {
        while !self.is_empty() && self.peek().unwrap().is_whitespace() {
            let _ = self.chop();
        }
    }

    pub fn chop(&mut self) {
        assert!(!self.is_empty());
        self.body = &self.body[1..];
    }
}

impl<'a> Deref for SingleLine<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn into_with_multi_line() {
        let _: SingleLine = "Hello\nSecond Line".into();
    }

    #[test]
    fn next() {
        let mut line: SingleLine = "Hello".into();
        line.chop();
        let expected: SingleLine = "ello".into();
        assert_eq!(expected, line);
    }

    #[test]
    fn chop_left_whitespace() {
        let mut line: SingleLine = "  Hello".into();
        line.chop_left_whitespace();
        let expected: SingleLine = "Hello".into();
        assert_eq!(expected, line);
    }
}
