use std::iter::Peekable;
use std::str::Chars;
use self::Token::{Identifier, LeftParenthesis, RightParenthesis, Comma, SingleQuote, EqualSign, Semicolon, Asterisk, StringValue, NumberValue};

#[derive(Debug,PartialEq)]
pub enum Token {
    Identifier(String),

    NumberValue(String),
    StringValue(String),

    LeftParenthesis,
    RightParenthesis,
    Comma,
    SingleQuote,
    EqualSign,
    Semicolon,
    Asterisk
}

pub trait Tokenizer {

    fn tokenize(&self) -> Result<Vec<Token>, String>;
}

impl Tokenizer for String {

    fn tokenize(&self) -> Result<Vec<Token>, String> {
        tokenize_expression(&mut self.chars().peekable())
    }
}

fn tokenize_expression(chars: &mut Peekable<Chars>) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    loop {
        match chars.peek().cloned() {
            Some(' ') => { chars.next(); },
            Some('\'') => { chars.next(); tokens.push(string_token(&mut chars.by_ref())); },
            Some('a'...'z') => { tokens.push(ident_token(&mut chars.by_ref())); },
            Some('0'...'9') => { tokens.push(try!(num_token(&mut chars.by_ref()))); },
            Some(c) => { chars.next(); tokens.push(try!(char_to_token(c))); },
            None => break,
        }
    }
    Ok(tokens)
}

fn ident_token(chars: &mut Peekable<Chars>) -> Token {
    let mut token = String::default();
    loop {
        match chars.peek().cloned() {
            Some(c @ 'a'...'z') | Some(c @ '_') | Some(c @ '0'...'9') => {
                chars.next();
                token.push(c);
            },
            Some(_) | None => break,
        }
    }
    Identifier(token)
}

fn num_token(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut num = String::default();
    let mut float_point = false;
    loop {
        match chars.peek().cloned() {
            Some(d @ '0'...'9') => {
                chars.next();
                num.push(d);
            },
            Some('.') => {
                if !float_point {
                    chars.next();
                    num.push('.');
                    float_point = true;
                }
                else {
                    return Err("Number format error".to_owned());
                }
            },
            Some(_) | None => break,
        }
    }
    Ok(NumberValue(num))
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
            Some(c) => { chars.next(); string.push(c); },
            None => break,
        }
    }
    StringValue(string)
}

fn char_to_token(c: char) -> Result<Token, String> {
    match c {
        '('     => Ok(LeftParenthesis),
        ')'     => Ok(RightParenthesis),
        ','     => Ok(Comma),
        '\''    => Ok(SingleQuote),
        ';'     => Ok(Semicolon),
        '='     => Ok(EqualSign),
        '*'     => Ok(Asterisk),
        _       => Err(format!("Unexpected character - {:?}", c)),
    }
}
