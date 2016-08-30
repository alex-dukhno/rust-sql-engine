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

pub trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for str {
    fn tokenize(&self) -> Vec<Token> {
        tokenize_expression(&mut self.chars().peekable())
    }
}

fn tokenize_expression(chars: &mut Peekable<Chars>) -> Vec<Token> {
    let mut tokens = vec![];
    loop {
        match chars.peek().cloned() {
            Some(' ') | Some('\n') | Some('\t') => { chars.next(); },
            Some('\'') => {
                chars.next();
                tokens.push(string_token(&mut chars.by_ref()));
            },
            Some('a'...'z') | Some('A'...'Z') => { tokens.push(ident_token(&mut chars.by_ref())); },
            Some('0'...'9') => { tokens.push(num_token(&mut chars.by_ref())); },
            Some(c) => {
                chars.next();
                tokens.push(char_to_token(c));
            },
            None => break,
        }
    }
    tokens
}

fn ident_token(chars: &mut Peekable<Chars>) -> Token {
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
    Token::Ident(token.to_lowercase())
}

fn num_token(chars: &mut Peekable<Chars>) -> Token {
    let mut num = String::default();
    while let Some(d @ '0'...'9') = chars.peek().cloned() {
        chars.next();
        num.push(d);
    }
    Token::NumericConstant(num)
}

fn string_token(chars: &mut Peekable<Chars>) -> Token {
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
            _ => break,
        }
    }
    Token::CharactersConstant(string)
}

fn char_to_token(c: char) -> Token {
    match c {
        '('     => Token::LeftParenthesis,
        ')'     => Token::RightParenthesis,
        ','     => Token::Comma,
        '\''    => Token::SingleQuote,
        ';'     => Token::Semicolon,
        '='     => Token::EqualSign,
        '*'     => Token::Asterisk,
        _       => unimplemented!(),
    }
}
