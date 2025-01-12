use std::str::FromStr;

#[derive(Debug)]
pub struct Section {
    level: u32,
    heading: Vec<Text>,
    body: Vec<Part>,
}

#[derive(Debug)]
pub struct TodoSection {}

#[derive(Debug)]
pub struct Paragraph {
    pub body: Vec<Text>,
}

#[derive(Debug)]
pub struct Quote {}

#[derive(Debug)]
pub struct SpecialQuote {}

#[derive(Debug)]
pub struct List {
    pub items: Vec<Paragraph>,
}

#[derive(Debug)]
pub struct Link {
    pub header: String,
    pub path: String,
}

#[derive(Debug)]
pub enum Text {
    Plain(String),
    Link(Link),
}

/// Part: every logical unit in a owl-markdown file is a `Part` a `Part` may consist of other
/// `Parts`
#[derive(Debug)]
pub enum Part {
    Section(Section),
    TodoSection(TodoSection),
    Paragraph(Paragraph),
    Quote(Quote),
    SpecialQuote(SpecialQuote),
    OrderedList(List),
    UnorderedList(List),
}

pub struct Document {
    pub parts: Vec<Part>,
}

#[derive(Debug)]
enum Token {
    Word(u32, String),
    Pound(u32),
    Space(u32),
    Tab(u32),
    NewLine(u32),
}

struct Tokenizer<'a> {
    body: &'a str,
    line_number: u32,
}

impl<'a> Tokenizer<'a> {
    fn new(s: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            body: s.trim(),
            line_number: 1,
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn peek(&self) -> char {
        assert!(!self.body.is_empty());
        self.body.chars().next().unwrap()
    }
    fn chop(&mut self) {
        assert!(!self.body.is_empty());
        self.body = &self.body[1..];
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.body.is_empty() {
            return None;
        }
        let c = self.peek();
        match c {
            '#' => {
                self.chop();
                return Some(Token::Pound(self.line_number));
            }
            '\n' => {
                self.chop();
                self.line_number += 1;
                return Some(Token::NewLine(self.line_number));
            }
            ' ' => {
                self.chop();
                return Some(Token::Space(self.line_number));
            }
            '\t' => {
                self.chop();
                return Some(Token::Tab(self.line_number));
            }
            _ => (),
        }

        let mut word = String::new();
        while !self.body.is_empty() && self.peek().is_alphanumeric() {
            word.push(self.peek());
            self.chop();
        }
        Some(Token::Word(self.line_number, word))
    }
}

impl FromStr for Document {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens = Tokenizer::new(input).collect::<Vec<_>>();
        todo!();
    }
}
