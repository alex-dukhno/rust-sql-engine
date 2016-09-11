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
    Asterisk,

    Insert,
    Into,
    Values,
    Select,
    From,
    Where,
    Delete,
    Create,
    Table,
    Columns,
    Limit,

    Int,
    VarChar
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
    fn from(token: &'s str) -> Token {
        match token {
            "(" => Token::LParent,
            ")" => Token::RParent,
            "," => Token::Comma,
            "'" => Token::SingleQuote,
            ";" => Token::Semicolon,
            "=" => Token::EqualSign,
            "*" => Token::Asterisk,
            "<>" | "!=" => Token::NotEqualSign,
            "<" => Token::Less,
            ">" => Token::Greater,
            "insert" => Token::Insert,
            "into" => Token::Into,
            "columns" => Token::Columns,
            "values" => Token::Values,
            "select" => Token::Select,
            "from" => Token::From,
            "where" => Token::Where,
            "delete" => Token::Delete,
            "create" => Token::Create,
            "table" => Token::Table,
            "int" => Token::Int,
            "varchar" => Token::VarChar,
            "limit" => Token::Limit,
            _ => Token::Ident(token.into()),
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

pub trait Tokenizer {
    type Token;
    type Item;

    fn tokenize(mut self) -> Vec<Self::Token>;

    fn look_ahead(&mut self) -> Option<Self::Item>;

    fn consume(&mut self);
}

pub trait IntoTokenizer<I: Iterator<Item = Self::Item>> {
    type Token;
    type Item;
    type IntoTokenizer: Tokenizer<Token = Self::Token, Item = Self::Item>;

    fn into_tokenizer(self) -> Self::IntoTokenizer;
}

impl IntoTokenizer<vec::IntoIter<char>> for String {
    type Token = Token;
    type Item = char;
    type IntoTokenizer = StringTokenizer<vec::IntoIter<char>>;

    fn into_tokenizer(self) -> Self::IntoTokenizer {
        StringTokenizer {
            sequence: self.chars().collect::<Vec<char>>().into_iter().peekable()
        }
    }
}

pub struct StringTokenizer<I: Iterator<Item = char>> {
    sequence: Peekable<I>
}

impl <I: Iterator<Item = char>> StringTokenizer<I> {

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
        Token::from(token.to_lowercase().as_str())
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

impl<I: Iterator<Item = char>> Tokenizer for StringTokenizer<I> {
    type Token = Token;
    type Item = char;

    fn tokenize(mut self) -> Vec<Token> {
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
}
