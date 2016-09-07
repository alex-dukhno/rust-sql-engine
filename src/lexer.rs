use std::iter::Peekable;
use std::fmt;
use std::vec;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),

    NumConst(String),
    CharsConst(String),

    LParent,
    RParent,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualSign,
    NotEqualSign,

    Comma,
    SingleQuote,
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
            "<>" => Token::NotEqualSign,
            "<" => Token::Less,
            ">" => Token::Greater,
            _ => unimplemented!(),
        }
    }
}

impl From<char> for Token {

    fn from(c: char) -> Token {
        match c {
            '(' => Token::LParent,
            ')' => Token::RParent,
            ',' => Token::Comma,
            '\'' => Token::SingleQuote,
            ';' => Token::Semicolon,
            '=' => Token::EqualSign,
            '*' => Token::Asterisk,
            '<' => Token::Less,
            '>' => Token::Greater,
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

pub struct Tokenizer<I: Iterator<Item = char>> {
    sequence: Peekable<I>
}

impl<I: Into<String>> From<I> for Tokenizer<vec::IntoIter<char>> {
    fn from(source: I) -> Tokenizer<vec::IntoIter<char>> {
        Tokenizer {
            sequence: source.into().chars().collect::<Vec<char>>().into_iter().peekable()
        }
    }
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            match self.look_ahead() {
                Some(' ') | Some('\n') | Some('\t') => { self.consume(); },
                Some('\'') => {
                    self.consume();
                    tokens.push(self.string_token());
                },
                Some('a'...'z') | Some('A'...'Z') => { tokens.push(self.ident_token()); },
                Some('0'...'9') => { tokens.push(self.numeric_token()); },
                Some('<') => {
                    self.consume();
                    match self.look_ahead() {
                        Some('>') => {
                            self.consume();
                            tokens.push(Token::NotEqualSign);
                        },
                        Some('=') => {
                            self.consume();
                            tokens.push(Token::LessEqual);
                        },
                        _ => tokens.push(Token::Less),
                    }
                },
                Some('>') => {
                    self.consume();
                    match self.look_ahead() {
                        Some('=') => {
                            self.consume();
                            tokens.push(Token::GreaterEqual);
                        },
                        _ => tokens.push(Token::Greater),
                    }
                }
                Some('!') => {
                    self.consume();
                    match self.look_ahead() {
                        Some('=') => {
                            self.consume();
                            tokens.push(Token::NotEqualSign);
                        },
                        _ => unimplemented!(),
                    }
                },
                Some(c) => {
                    self.consume();
                    tokens.push(Token::from(c));
                },
                None => break,
            }
        }
        tokens
    }

    fn look_ahead(&mut self) -> Option<char> {
        self.sequence.peek().cloned()
    }

    fn consume(&mut self) {
        self.sequence.next();
    }

    fn ident_token(&mut self) -> Token {
        let mut token = String::default();
        loop {
            match self.look_ahead() {
                Some(c @ 'A'...'Z') |
                Some(c @ 'a'...'z') |
                Some(c @ '_') |
                Some(c @ '0'...'9') => {
                    self.consume();
                    token.push(c);
                },
                _ => break,
            }
        }
        Token::ident(token.to_lowercase())
    }

    fn numeric_token(&mut self) -> Token {
        let mut number = String::default();
        while let Some(d @ '0'...'9') = self.look_ahead() {
            self.consume();
            number.push(d);
        }
        Token::number(number)
    }

    fn string_token(&mut self) -> Token {
        let mut string = String::default();
        loop {
            match self.look_ahead() {
                Some('\'') => {
                    self.consume();
                    match self.look_ahead() {
                        Some('\'') => {
                            self.consume();
                            string.push('\'');
                        },
                        _ => break,
                    }
                },
                Some(c) => {
                    self.consume();
                    string.push(c);
                },
                None => break,
            }
        }
        Token::string(string)
    }
}
