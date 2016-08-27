use std::iter::Peekable;
use std::str::Chars;
use std::fmt;

use self::Token::{IdentT, NumberT, StringT, LeftParenthesis, RightParenthesis, Comma, SingleQuote, EqualSign, Semicolon, Asterisk};

#[derive(Debug, PartialEq)]
pub enum Token {
    IdentT(String),

    NumberT(String),
    StringT(String),

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
            RightParenthesis => ")",
            LeftParenthesis => "(",
            Semicolon => ";",
            Comma => ",",
            IdentT(ref id) => id.as_str(),
            _ => "unimplemented formatting",
        }.fmt(f)
    }
}

pub trait Tokenizer {
    fn tokenize(&self) -> Result<Vec<Token>, String>;
}

impl Tokenizer for str {
    fn tokenize(&self) -> Result<Vec<Token>, String> {
        tokenize_expression(&mut self.chars().peekable())
    }
}

fn tokenize_expression(chars: &mut Peekable<Chars>) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    loop {
        match chars.peek().cloned() {
            Some(' ') | Some('\n') | Some('\t') => { chars.next(); },
            Some('\'') => {
                chars.next();
                tokens.push(try!(string_token(&mut chars.by_ref())));
            },
            Some('a'...'z') | Some('A'...'Z') => { tokens.push(ident_token(&mut chars.by_ref())); },
            Some('0'...'9') => { tokens.push(try!(num_token(&mut chars.by_ref()))); },
            Some(c) => {
                chars.next();
                tokens.push(try!(char_to_token(c)));
            },
            None => break,
        }
    }
    Ok(tokens)
}

fn ident_token(chars: &mut Peekable<Chars>) -> Token {
    let mut token = String::default();
    loop {
        match chars.peek().cloned() {
            Some(c @ 'A'...'Z') => {
                chars.next();
                token.push_str(c.to_lowercase().collect::<String>().as_str());
            },
            Some(c @ 'a'...'z') | Some(c @ '_') | Some(c @ '0'...'9') => {
                chars.next();
                token.push(c);
            },
            Some(_) | None => break,
        }
    }
    IdentT(token)
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
                } else {
                    return Err("Number format error".to_owned());
                }
            },
            Some(_) | None => break,
        }
    }
    Ok(NumberT(num))
}

fn string_token(chars: &mut Peekable<Chars>) -> Result<Token, String> {
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
            None => return Err("string const should be closed by \'".to_owned()),
        }
    }
    Ok(StringT(string))
}

fn char_to_token(c: char) -> Result<Token, String> {
    match c {
        '(' => Ok(LeftParenthesis),
        ')' => Ok(RightParenthesis),
        ',' => Ok(Comma),
        '\'' => Ok(SingleQuote),
        ';' => Ok(Semicolon),
        '=' => Ok(EqualSign),
        '*' => Ok(Asterisk),
        _ => Err(format!("Unexpected character - {:?}", c)),
    }
}
