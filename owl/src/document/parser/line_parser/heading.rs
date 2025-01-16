use super::ErrorType;
use super::Event;
use super::SingleLine;
use super::Trait;

use super::Text;

#[derive(Debug, Clone)]
enum State<'a> {
    Begin(SingleLine<'a>),
    Body(Text<'a>),
    End,
}

#[derive(Debug)]
pub(super) struct Heading<'a> {
    state: State<'a>,
}

impl<'a> From<SingleLine<'a>> for Heading<'a> {
    fn from(line: SingleLine<'a>) -> Self {
        Self {
            state: State::Begin(line),
        }
    }
}

impl<'a> Trait<'a> for Heading<'a> {
    fn wants(&self, _: &SingleLine<'a>) -> bool {
        false
    }

    fn feed(&mut self, _: SingleLine<'a>) {
        todo!()
    }
}

impl<'a> Iterator for Heading<'a> {
    type Item = Result<Event, ErrorType>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            State::Begin(line) => {
                if line.get_indent() > 0 {
                    line.chop_left_whitespace();
                    return Some(Err(ErrorType::HeadingIndented));
                }
                let mut level = 0;

                while let Some('#') = line.peek() {
                    line.chop();
                    level += 1;
                }
                line.chop_left_whitespace();
                self.state = State::Body(line.clone().into());
                Some(Ok(Event::Heading(level)))
            }
            State::Body(text) => {
                match text.next() {
                    Some(result) => Some(result),
                    None => {
                        self.state = State::End;
                        Some(Ok(Event::HeadingEnd))
                    }
                }
            }
            State::End => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let line: SingleLine = "# Hello World".into();
        let mut parser: Heading = line.into();

        let got = parser.next().unwrap().unwrap();
        let expected = Event::Heading(1);
        assert_eq!(expected, got);

        let mut last = None;

        while let Some(result) = parser.next() {
            last = Some(result);
        }

        let got = last.unwrap().unwrap();
        let expected = Event::HeadingEnd;
        assert_eq!(expected, got);
    }

    #[test]
    fn indented_heading() {
        let line: SingleLine = " # Heading".into();
        let mut heading: Heading = line.into();

        let got = heading.next().unwrap();
        let expected = ErrorType::HeadingIndented;
        if let Err(got) = got {
            assert_eq!(expected, got);
        } else {
            panic!("Expected: {:?}, {:?}", Err::<Event, _>(expected), got);
        }

        let got = heading.next().unwrap().unwrap();
        let expected = Event::Heading(1);
        assert_eq!(expected, got);

        let mut last = None;

        while let Some(result) = heading.next() {
            last = Some(result);
        }

        let got = last.unwrap().unwrap();
        let expected = Event::HeadingEnd;
        assert_eq!(expected, got);
    }
}
