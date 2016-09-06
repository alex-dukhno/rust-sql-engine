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

impl Token {
    pub fn ident<I: Into<String>>(indet: I) -> Token {
        Token::Ident(indet.into())
    }

    pub fn number<I: Into<String>>(num: I) -> Token {
        Token::NumericConstant(num.into())
    }

    pub fn string<I: Into<String>>(string: I) -> Token {
        Token::CharactersConstant(string.into())
    }
}

impl From<char> for Token {
    fn from(c: char) -> Token {
        match c {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            ',' => Token::Comma,
            '\'' => Token::SingleQuote,
            ';' => Token::Semicolon,
            '=' => Token::EqualSign,
            '*' => Token::Asterisk,
            _ => unimplemented!(),
        }
    }
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

pub struct Tokenizer {
    source: String
}

impl<'s> From<&'s str> for Tokenizer {
    fn from(source: &'s str) -> Tokenizer {
        Tokenizer { source: source.to_owned() }
    }
}

impl<'t> IntoIterator for &'t Tokenizer {
    type Item = char;
    type IntoIter = Chars<'t>;

    fn into_iter(self) -> Self::IntoIter {
        self.source.chars()
    }
}

impl Tokenizer {
    pub fn tokenize(self) -> Vec<Token> {
        let mut chars = self.into_iter().peekable();
        let mut tokens = vec![];
        loop {
            match chars.peek().cloned() {
                Some(' ') | Some('\n') | Some('\t') => { chars.next(); },
                Some('\'') => {
                    chars.next();
                    tokens.push(self.string_token(&mut chars.by_ref()));
                },
                Some('a'...'z') | Some('A'...'Z') => { tokens.push(self.ident_token(&mut chars.by_ref())); },
                Some('0'...'9') => { tokens.push(self.numeric_token(&mut chars.by_ref())); },
                Some(c) => {
                    chars.next();
                    tokens.push(Token::from(c));
                },
                None => break,
            }
        }
        tokens
    }

    fn ident_token<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> Token {
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
        Token::ident(token.to_lowercase())
    }

    fn numeric_token<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> Token {
        let mut number = String::default();
        while let Some(d @ '0'...'9') = chars.peek().cloned() {
            chars.next();
            number.push(d);
        }
        Token::number(number)
    }

    fn string_token<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> Token {
        Token::string(self.take_while_quote(chars.by_ref()) + &self.match_break(chars.by_ref()))
    }

    fn take_while_quote<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> String {
        chars.take_while(|c| *c != '\'').collect::<String>()
    }

    fn match_break<I: Iterator<Item = char>>(&self, chars: &mut Peekable<I>) -> String {
        let string = String::default();
        if let Some('\'') = chars.peek().cloned() {
            chars.next();
            string + "\'" + self.take_while_quote(chars.by_ref()).as_str()
        } else {
            string
        }
    }
}
