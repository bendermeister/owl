use super::Event;
use super::SingleLine;
use super::Trait;
use super::ErrorType;

#[derive(Debug, Clone)]
enum State {
    Start,
    Link,
    Body,
    End,
}

#[derive(Debug, Clone)]
pub(super) struct Text<'a> {
    state: State,
    line: SingleLine<'a>,
}

impl<'a> Trait<'a> for Text<'a> {
    fn wants(&self, line: &SingleLine<'a>) -> bool {
        line.get_indent() == 0
    }

    fn feed(&mut self, line: SingleLine<'a>) {
        assert!(self.line.is_empty());
        self.line = line;
    }
}

impl<'a> Iterator for Text<'a> {
    type Item = Result<Event, ErrorType>;

    fn next(&mut self) -> Option<Self::Item> {
        println!("[INFO] {:?}", self);
        match &mut self.state {
            State::Start => {
                if self.line.get_indent() > 0 {
                    todo!()
                }
                self.state = State::Body;
                Some(Ok(Event::Text))
            }
            State::Link => {
                todo!()
            }
            State::Body => {
                if let None = self.line.peek() {
                    self.state = State::End;
                    return Some(Ok(Event::TextEnd));
                }
                let c = self.line.peek().unwrap();
                if c.is_whitespace() {
                    self.line.chop();
                    return Some(Ok(Event::WhiteSpace(c)));
                }

                match c {
                    ']' => {
                        self.line.chop();
                        if self.line.peek().is_some() && self.line.peek().unwrap() == '(' {
                            self.state = State::Link;
                        }
                        return Some(Ok(Event::Special(']')));
                    }
                    '[' => {
                        self.line.chop();
                        return Some(Ok(Event::Special('[')));
                    },
                    _ => (),
                }

                let mut body = String::new();

                let mut c = self.line.peek();
                while c.is_some() && !c.unwrap().is_whitespace() {
                    body.push(c.unwrap());
                    self.line.chop();
                    c = self.line.peek();
                }
                Some(Ok(Event::Word(body)))
            }
            State::End => None,
        }
    }
}

impl<'a> From<SingleLine<'a>> for Text<'a> {
    fn from(line: SingleLine<'a>) -> Self {
        Self { line, state: State::Start }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn link() {
        let line: SingleLine = "[word](destination)".into();
        let text: Text = line.into();

        let got = text.next().unwrap().unwrap();
        let expected = Event::Special('[');
        assert_eq!(expected, got);

        let got = text.next().unwrap().unwrap();
        let expected = Event::Word("word".into());
        assert_eq!(expected, got);
    }
}
