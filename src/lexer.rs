use std::iter::Peekable;
use std::str::Chars;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),

    NumConst(String),
    CharsConst(String),

    LParent,
    RParent,

    LAngle,
    RAngle,

    Comma,
    SingleQuote,
    EqualSign,
    Semicolon,
    Asterisk
}

impl Token {
    pub fn ident<I: Into<String>>(indet: I) -> Token {
        Token::Ident(indet.into())
    }

    pub fn number<I: Into<String>>(num: I) -> Token {
        Token::NumConst(num.into())
    }

    pub fn string<I: Into<String>>(string: I) -> Token {
        Token::CharsConst(string.into())
    }
}

impl<'s> From<&'s str> for Token {
    fn from(c: &'s str) -> Token {
        match c {
            "(" => Token::LParent,
            ")" => Token::RParent,
            "," => Token::Comma,
            "'" => Token::SingleQuote,
            ";" => Token::Semicolon,
            "=" => Token::EqualSign,
            "*" => Token::Asterisk,
            "<" => Token::LAngle,
            ">" => Token::RAngle,
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::RParent => ")",
            Token::LParent => "(",
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
                    tokens.push(self.string_token(&mut chars.by_ref()));
                },
                Some('a'...'z') | Some('A'...'Z') => { tokens.push(self.ident_token(&mut chars.by_ref())); },
                Some('0'...'9') => { tokens.push(self.numeric_token(&mut chars.by_ref())); },
                Some(c) => {
                    chars.next();
                    let mut s = String::with_capacity(1);
                    s.push(c);
                    tokens.push(Token::from(s.as_str()));
                },
                None => break,
            }
        }
        tokens
    }

    fn ident_token<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> Token {
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
        Token::ident(token.to_lowercase())
    }

    fn numeric_token<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> Token {
        let mut number = String::default();
        while let Some(d @ '0'...'9') = chars.peek().cloned() {
            chars.next();
            number.push(d);
        }
        Token::number(number)
    }

    fn string_token<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> Token {
        let mut string = String::default();
        loop {
            match chars.peek().cloned() {
                Some('\'') => {
                    chars.next();
                    match chars.peek().cloned() {
                        Some('\'') => {
                            chars.next();
                            string.push('\'');
                        },
                        _ => break,
                    }
                },
                Some(c) => {
                    chars.next();
                    string.push(c);
                },
                None => break,
            }
        }
        Token::string(string)
    }
}
