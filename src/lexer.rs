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
        fn push_identifier(tokens: &mut Vec<Token>, buffer: &mut String) {
            if !buffer.is_empty() {
                tokens.push(Identifier(buffer.clone()));
                buffer.clear();
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


        let mut tokens = vec![];
        let mut buffer = String::with_capacity(self.len());
        for c in self.chars() {
            if DELIMETERS.contains(c) {
                push_identifier(&mut tokens, &mut buffer);
                if let Some(token) = char_to_token(c) {
                    tokens.push(token);
                }
            }
            else {
                buffer.push(c);
            }
        }
        push_identifier(&mut tokens, &mut buffer);
        tokens
    }
    
}
