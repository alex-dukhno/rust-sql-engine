use std::iter::Peekable;
use std::fmt;

#[derive(PartialEq)]
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
    Null,
    Foreign,
    References,

    And,
    Not,

    Int,
    Character
}

impl fmt::Debug for Token {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Ident(ref val) => write!(f, "Ident('{}')", val),
            Token::NumConst(ref val) => write!(f, "NumericConstant({})", val),
            Token::CharsConst(ref val) => write!(f, "StringConstant({})", val),

            Token::EqualSign => write!(f, "EqualTo"),
            Token::NotEqualSign => write!(f, "NotEqualTo"),
            Token::Less => write!(f, "LessThan"),
            Token::LessEqual => write!(f, "LessThanOrEqualTo"),
            Token::Greater => write!(f, "GreaterThan"),
            Token::GreaterEqual => write!(f, "GreaterThanOrEqualTo"),

            Token::Minus => write!(f, "Symbol(-)"),
            Token::Plus => write!(f, "Symbol(+)"),
            Token::Asterisk => write!(f, "Symbol(*)"),
            Token::Slash => write!(f, "Symbol(/)"),
            Token::LParent => write!(f, "Symbol('(')"),
            Token::RParent => write!(f, "Symbol(')')"),
            Token::Semicolon => write!(f, "Symbol(';')"),
            Token::Comma => write!(f, "Symbol(',')"),

            Token::Character => write!(f, "KeyWord('CHARACTER')"),
            Token::Int => write!(f, "KeyWord('INTEGER')"),

            Token::Insert => write!(f, "KeyWord('INSERT')"),
            Token::Into => write!(f, "KeyWord('INTO')"),
            Token::Values => write!(f, "KeyWord('VALUES')"),
            Token::Select => write!(f, "KeyWord('SELECT')"),
            Token::From => write!(f, "KeyWord('FROM')"),
            Token::Where => write!(f, "KeyWord('WHERE')"),
            Token::Create => write!(f, "KeyWord('CREATE')"),
            Token::Table => write!(f, "KeyWord('TABLE')"),
            Token::Primary => write!(f, "KeyWord('PRIMARY')"),
            Token::Foreign => write!(f, "KeyWord('FOREIGN')"),
            Token::Key => write!(f, "KeyWord('KEY')"),
            Token::References => write!(f, "KeyWord('REFERENCES')"),
            Token::Null => write!(f, "KeyWord('NULL')"),

            Token::Not => write!(f, "KeyWord('NOT')"),
            Token::And => write!(f, "KeyWord('AND')"),

            _ => write!(f, "unimplemented debug representation")
        }
    }
}

impl<'s> From<&'s str> for Token {
    fn from(token: &'s str) -> Token {
        match token {
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
            "integer" | "int" => Token::Int,
            "char" | "character" => Token::Character,
            "limit" => Token::Limit,
            "and" => Token::And,
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

pub type Tokens = Vec<Token>;

pub fn tokenize(src: &str) -> Result<Tokens, String> {
    let mut chars = src.chars().peekable();
    let mut tokens = vec![];
    while let Some(c) = chars.peek().cloned() {
        match c {
            ' ' | '\n' | '\t' => { chars.next(); },
            'a' ... 'z' |
            'A' ... 'Z' => tokens.push(ident_token(chars.by_ref())),
            '0' ... '9' => tokens.push(numeric_token(chars.by_ref())),
            '\'' => tokens.push(string_token(chars.by_ref())),
            '/' => {
                chars.next();
                match chars.peek().cloned() {
                    Some('*') => skip_multi_line_comment(chars.by_ref()),
                    _ => tokens.push(Token::Slash)
                }
            }
            '-' => {
                chars.next();
                match chars.peek().cloned() {
                    Some('-') => skip_single_line_comment(chars.by_ref()),
                    _ => tokens.push(Token::Minus)
                }
            }
            '<' => {
                chars.next();
                match chars.peek().cloned() {
                    Some('>') => {
                        chars.next();
                        tokens.push(Token::NotEqualSign);
                    }
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::LessEqual);
                    }
                    _ => tokens.push(Token::Less),
                }
            }
            '>' => {
                chars.next();
                match chars.peek().cloned() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::GreaterEqual);
                    }
                    _ => tokens.push(Token::Greater),
                }
            }
            '!' => {
                chars.next();
                match chars.peek().cloned() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::NotEqualSign);
                    }
                    _ => unimplemented!(),
                }
            }
            _ => {
                tokens.push(Token::from(c));
                chars.next();
            },
        }
    }
    Ok(tokens)
}

fn ident_token<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Token {
    let mut token = String::default();
    while let Some(c) = chars.peek().cloned() {
        match c {
            'A' ... 'Z' |
            'a' ... 'z' |
            '0' ... '9' |
            '_' => { token.push(c); chars.next(); },
            _ => break,
        }
    }
    Token::from(token.to_lowercase().as_str())
}

fn numeric_token<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Token {
    let mut number = String::default();
    while let Some(d) = chars.peek().cloned() {
        if d.is_digit(10) {
            number.push(d);
            chars.next();
        } else {
            break;
        }
    }
    Token::NumConst(number)
}

fn string_token<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Token {
    chars.next();
    let mut string = String::default();
    while let Some(c) = chars.peek().cloned() {
        match c {
            '\'' => {
                chars.next();
                match chars.peek().cloned() {
                    Some('\'') => {
                        string.push('\'');
                        chars.next();
                    },
                    _ => break,
                }
            }
            _ => {
                string.push(c);
                chars.next();
            }
        }
    }
    Token::CharsConst(string)
}

fn skip_multi_line_comment<I: Iterator<Item = char>>(chars: &mut Peekable<I>) {
    let mut previous = chars.next().unwrap();
    while let Some(current) = chars.peek().cloned() {
        chars.next();
        if (previous, current) == ('*', '/') {
            break;
        } else {
            previous = current;
        }
    }
}

fn skip_single_line_comment<I: Iterator<Item = char>>(chars: &mut Peekable<I>) {
    while let Some(c) = chars.peek().cloned() {
        match c {
            '\n' => break,
            _ => { chars.next(); },
        }
    }
}
