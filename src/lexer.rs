use std::iter::Peekable;
use std::str::Chars;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),

    NumericConstant(String),
    CharactersConstant(String),

    LeftParenthesis,
    RightParenthesis,
    Comma,
    SingleQuote,
    EqualSign,
    Semicolon,
    Asterisk
}

impl From<char> for Token {
    fn from(c: char) -> Token {
        match c {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            ',' => Token::Comma,
            '\'' => Token::SingleQuote,
            ';' => Token::Semicolon,
            '=' => Token::EqualSign,
            '*' => Token::Asterisk,
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::RightParenthesis => ")",
            Token::LeftParenthesis => "(",
            Token::Semicolon => ";",
            Token::Comma => ",",
            Token::Ident(ref id) => id.as_str(),
            _ => unimplemented!(),
        }.fmt(f)
    }
}

pub struct Tokenizer {
    source: String
}

impl<'s> From<&'s str> for Tokenizer {
    fn from(source: &'s str) -> Tokenizer {
        Tokenizer { source: source.to_owned() }
    }
}

impl<'t> IntoIterator for &'t Tokenizer {
    type Item = char;
    type IntoIter = Chars<'t>;

    fn into_iter(self) -> Self::IntoIter {
        self.source.chars()
    }
}

impl Tokenizer {
    pub fn tokenize(self) -> Vec<Token> {
        let mut chars = self.into_iter().peekable();
        let mut tokens = vec![];
        loop {
            match chars.peek().cloned() {
                Some(' ') | Some('\n') | Some('\t') => { chars.next(); },
                Some('\'') => {
                    chars.next();
                    tokens.push(Token::CharactersConstant(self.string_token(&mut chars.by_ref())));
                },
                Some('a'...'z') | Some('A'...'Z') => { tokens.push(Token::Ident(self.ident_token(&mut chars.by_ref()))); },
                Some('0'...'9') => { tokens.push(Token::NumericConstant(self.numeric_token(&mut chars.by_ref()))); },
                Some(c) => {
                    chars.next();
                    tokens.push(Token::from(c));
                },
                None => break,
            }
        }
        tokens
    }

    fn ident_token<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> String {
        let mut token = String::default();
        loop {
            match chars.peek().cloned() {
                Some(c @ 'A'...'Z') |
                Some(c @ 'a'...'z') |
                Some(c @ '_') |
                Some(c @ '0'...'9') => {
                    chars.next();
                    token.push(c);
                },
                _ => break,
            }
        }
        token.to_lowercase()
    }

    fn numeric_token<I: Iterator<Item = char>>(&self, chars: &mut I) -> String {
        chars.take_while(|d| d.is_digit(10)).collect::<String>()
    }

    fn string_token<I: Iterator<Item = char>>(&self, chars: &mut I) -> String {
        let mut string = String::default();
        while let Some(c) = chars.next() {
            if c == '\'' { break; }
            string.push(c);
        }
        match chars.next() {
            Some('\'') => string + "\'" + self.string_token(chars.by_ref()).as_str(),
            _ => string,
        }
    }
}
