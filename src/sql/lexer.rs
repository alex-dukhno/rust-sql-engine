use std::option::Option;
use std::str::Chars;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum Token {
    NewLine,
    WhiteSpace,
    KeyWord(String),
    Table(String)
}

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>
}

impl <'a> Lexer<'a> {

    pub fn new(sql: &'a str) -> Lexer {
        Lexer { src: sql.chars().peekable() }
    }

    pub fn is_empty(&mut self) -> bool {
        self.src.peek() == None
    }

    fn same_symbol_group(c: &Option<char>, iter: &mut Peekable<Chars<'a>>) -> bool {
        let next = iter.peek();
        if c.as_ref() != next {
            return match *c {
                Some(' ') | Some('\t') => Some(&' ') == next || Some(&'\t') == next,
                Some(_) | None => false
            }
        }
        true
    }
}

impl <'a> Iterator for Lexer<'a> {

    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.is_empty() {
            return None;
        }
        let c = self.src.next();
        while Lexer::same_symbol_group(&c, &mut self.src) {
            self.src.next();
        }
        match c {
            Some(' ') | Some('\t')  => Some(Token::WhiteSpace),
            Some('\n') => Some(Token::NewLine),
            Some(_) | None => None
        }
    }
}
