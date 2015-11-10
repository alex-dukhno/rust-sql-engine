use std::option::Option;
use std::str::Chars;
use std::iter::Peekable;

const SIGN: char = '~';

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

    pub fn new(sql: &'a str) -> Lexer<'a> {
        Lexer {
            src: sql.chars().peekable()
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.src.peek() == None
    }

    fn has_same_group(symbol: &Option<char>, next_symbol: &Option<&char>, collector: &mut Vec<char>) -> bool {
        match (*symbol, *next_symbol) {
            (Some(' ') , Some(&' ')) | (Some('\t') , Some(&'\t')) | (Some(' '), Some(&'\t')) | (Some('\t'), Some(&' ')) => true,
            (Some('\n'), Some(&'\n')) => true,
            (Some('a'...'z'), Some(v @ &'a'...'z')) => { collector.push(*v); true },
            (Some(_), Some(_)) | (Some(_), None) | (None, Some(_)) | (None, None) => false
        }
    }
}

impl <'a> Iterator for Lexer<'a> {

    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.is_empty() {
            return None;
        }
        let c = self.src.next();
        let mut collector = Vec::new();
        collector.push(c.unwrap());
        loop {
            if !Lexer::has_same_group(&c, &(self.src.peek()), &mut collector) {
                break;
            }
            self.src.next();
        }
        match c {
            Some(' ') | Some('\t')  => Some(Token::WhiteSpace),
            Some('\n') => Some(Token::NewLine),
            Some('a'...'z') => Some(Token::KeyWord(collector.iter().map(|c| *c).collect())),
            Some(_) | None => None
        }
    }
}
