use std::iter::Peekable;
use std::fmt;

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
    Plus,
    Minus,
    Asterisk,
    Slash,

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
    Primary,
    Key,
    Default,
    Not,
    Null,
    Foreign,
    References,

    Int,
    Character
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
            "+" => Token::Plus,
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
            "primary" => Token::Primary,
            "foreign" => Token::Foreign,
            "key" => Token::Key,
            "references" => Token::References,
            "default" => Token::Default,
            "not" => Token::Not,
            "null" => Token::Null,
            "integer" => Token::Int,
            "char" | "character" => Token::Character,
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
            '<' => Token::Less,
            '>' => Token::Greater,
            '*' => Token::Asterisk,
            '+' => Token::Plus,
            '/' => Token::Slash,
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

fn skip_comments(src: &str) -> String {
    let mut with_out_comments = src.chars().collect::<String>();
    for (start, end) in comment_sections(src) {
        with_out_comments.drain(start..end);
    }
    with_out_comments
}

fn comment_sections(src: &str) -> Vec<(usize, usize)> {
    let mut sections = vec![];

    let mut chars = src.chars();
    let mut counter = 0;
    let mut previous = ' ';
    while let Some(current) = chars.next() {
        match (previous, current) {
            ('-', '-') => {
                let (start, end) = end_of_comments(counter, chars.by_ref(), end_of_linear_comment);
                sections.push((start, end));
                counter = end;
                previous = '\n';
            }
            ('/', '*') => {
                let (start, end) = end_of_comments(counter, chars.by_ref(), end_multi_line_comment);
                sections.push((start, end));
                counter = end;
                previous = '/';
            }
            _ => {
                previous = current;
                counter += 1;
            }
        }
    }
    sections.reverse();
    sections
}

fn end_of_comments<F: Fn(&mut I) -> usize, I: Iterator<Item = char>>(index: usize, char_seq: &mut I, skip_function: F) -> (usize, usize) {
    (index - 1, skip_function(char_seq.by_ref()) + index + 1)
}

fn end_of_linear_comment<I: Iterator<Item = char>>(char_seq: &mut I) -> usize {
    let mut end = 0;
    while let Some(current) = char_seq.next() {
        end += 1;
        if current == '\n' {
            break;
        }
    }
    end
}

fn end_multi_line_comment<I: Iterator<Item = char>>(char_seq: &mut I) -> usize {
    let mut end = 0;
    let mut previous = ' ';
    while let Some(current) = char_seq.next() {
        end += 1;
        if previous == '*' && current == '/' {
            break;
        }
        previous = current;
    }
    end
}

pub type Tokens = Vec<Token>;

pub fn tokenize(src: &str) -> Result<Tokens, String> {
    let without_comments = skip_comments(src);
    let mut chars = without_comments.chars().peekable();
    let mut tokens = vec![];
    loop {
        match look_ahead(chars.by_ref()) {
            Some(' ') | Some('\n') | Some('\t') => { consume(chars.by_ref()); },
            Some('\'') => {
                consume(chars.by_ref());
                tokens.push(string_token(chars.by_ref()));
            },
            Some('a' ... 'z') | Some('A' ... 'Z') => { tokens.push(ident_token(chars.by_ref())); },
            Some('/') => {
                consume(chars.by_ref());
                match look_ahead(chars.by_ref()) {
                    Some('*') => {
                        consume(chars.by_ref());
                        let mut previous = ' ';
                        while let Some(c) = look_ahead(chars.by_ref()) {
                            consume(chars.by_ref());
                            if previous == '*' && c == '/' {
                                break;
                            } else {
                                previous = c;
                            }
                        }
                    },
                    Some(_) => tokens.push(Token::Slash),
                    None => unimplemented!()
                }
            }
            Some('-') => {
                consume(chars.by_ref());
                match look_ahead(chars.by_ref()) {
                    Some('0' ... '9') => { tokens.push(numeric_token(chars.by_ref(), Some('-'))); },
                    Some('-') => {
                        while let Some(c) = look_ahead(chars.by_ref()) {
                            match c {
                                '\n' => break,
                                _ => consume(chars.by_ref())
                            }
                        }
                    },
                    Some(_) => tokens.push(Token::Minus),
                    None => unimplemented!()
                }
            }
            Some('0' ... '9') => { tokens.push(numeric_token(chars.by_ref(), None)); },
            Some('<') => {
                consume(chars.by_ref());
                match look_ahead(chars.by_ref()) {
                    Some('>') => {
                        consume(chars.by_ref());
                        tokens.push(Token::NotEqualSign);
                    },
                    Some('=') => {
                        consume(chars.by_ref());
                        tokens.push(Token::LessEqual);
                    },
                    _ => tokens.push(Token::Less),
                }
            },
            Some('>') => {
                consume(chars.by_ref());
                match look_ahead(chars.by_ref()) {
                    Some('=') => {
                        consume(chars.by_ref());
                        tokens.push(Token::GreaterEqual);
                    },
                    _ => tokens.push(Token::Greater),
                }
            }
            Some('!') => {
                consume(chars.by_ref());
                match look_ahead(chars.by_ref()) {
                    Some('=') => {
                        consume(chars.by_ref());
                        tokens.push(Token::NotEqualSign);
                    },
                    _ => unimplemented!(),
                }
            },
            Some(c) => {
                consume(chars.by_ref());
                tokens.push(Token::from(c));
            },
            None => break,
        }
    }
    Ok(tokens)
}

fn ident_token<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Token {
    let mut token = String::default();
    loop {
        match look_ahead(chars.by_ref()) {
            Some(c @ 'A' ... 'Z') |
            Some(c @ 'a' ... 'z') |
            Some(c @ '_') |
            Some(c @ '0' ... '9') => {
                consume(chars.by_ref());
                token.push(c);
            },
            _ => break,
        }
    }
    Token::from(token.to_lowercase().as_str())
}

fn numeric_token<I: Iterator<Item = char>>(chars: &mut Peekable<I>, sign: Option<char>) -> Token {
    let mut number = String::default();
    if let Some(c) = sign {
        number.push(c);
    }
    while let Some(d) = look_ahead(chars.by_ref()) {
        match d {
            '0' ... '9' => {
                consume(chars.by_ref());
                number.push(d);
            },
            _ => break
        }
    }
    Token::number(number)
}

fn string_token<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Token {
    let mut string = String::default();
    loop {
        match look_ahead(chars.by_ref()) {
            Some('\'') => {
                consume(chars.by_ref());
                match look_ahead(chars.by_ref()) {
                    Some('\'') => {
                        consume(chars.by_ref());
                        string.push('\'');
                    },
                    _ => break,
                }
            },
            Some(c) => {
                consume(chars.by_ref());
                string.push(c);
            },
            None => break,
        }
    }
    Token::string(string)
}

fn look_ahead<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Option<char> {
    chars.peek().cloned()
}

fn consume<I: Iterator<Item = char>>(chars: &mut Peekable<I>) {
    chars.next();
}
