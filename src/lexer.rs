use self::Token::{Identifier, LeftParenthesis, RightParenthesis, Comma, SingleQuote, EqualSign, Semicolon, Asterisk};

const DELIMETERS: &'static str = "(),\';=* ";

#[derive(Debug,PartialEq)]
pub enum Token {
    Identifier(String),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    SingleQuote,
    EqualSign,
    Semicolon,
    Asterisk
}

pub trait Tokenizer {
    
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for String {

    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut end = 0;
        let mut start = end;
        for c in self.chars() {
            if DELIMETERS.contains(c) {
                if start < end {
                    tokens.push(Identifier(self.chars().skip(start).take(end - start).collect::<String>()));
                }
                end += 1;
                start = end;
                if let Some(token) = char_to_token(c) {
                    tokens.push(token);
                }
            }
            else {
                end += 1;
            }
        }
        if start < end {
            tokens.push(Identifier(self.chars().skip(start).take(end - start).collect::<String>()));
        }
        tokens
    }
    
}

fn char_to_token(c: char) -> Option<Token> {
    match c {
        '('     => Some(LeftParenthesis),
        ')'     => Some(RightParenthesis),
        ','     => Some(Comma),
        '\''    => Some(SingleQuote),
        ';'     => Some(Semicolon),
        '='     => Some(EqualSign),
        '*'     => Some(Asterisk),
        _       => None
    }
}
