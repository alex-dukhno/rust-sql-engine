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

pub struct Lexer { }

impl Lexer {

    pub fn tokenize(&self, src: &str) -> Vec<Token> {
        let mut tokens = vec![];
        let mut buffer = String::with_capacity(src.len());
        for c in src.chars() {
            if DELIMETERS.contains(c) {
                self.push_identifier(&mut tokens, &mut buffer);
                if let Some(token) = self.char_to_token(c) {
                    tokens.push(token);
                }
            }
            else {
                buffer.push(c);
            }
        }
        self.push_identifier(&mut tokens, &mut buffer);
        tokens
    }

    fn push_identifier(&self, tokens: &mut Vec<Token>, buffer: &mut String) {
        if !buffer.is_empty() {
            tokens.push(Identifier(buffer.clone()));
            buffer.clear();
        }
    }

    fn char_to_token(&self, c: char) -> Option<Token> {
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
}

impl Default for Lexer {

    fn default() -> Self {
        Lexer { }
    }
}
