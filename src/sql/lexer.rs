#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Word(String),

    //characters
    LeftParenthesis,
    RightParenthesis,
    SemiColon,
    SingleQuote,
    Colon
}

pub struct Lexer {
    src: Vec<char>,
    string_expr: bool
}

impl Lexer {
    
    pub fn new(line: &str) -> Lexer {
        Lexer {
            src: line.chars().collect::<Vec<char>>(),
            string_expr: false
        }
    }

    pub fn next_lexem(&mut self) -> Option<Token> {
        self.remove_front_spaces();
        if self.src.is_empty() {
            None
        }
        else {
            Some(self.parse_lexem())
        }
    }

    fn parse_lexem(&mut self) -> Token {
        if self.has_lexem_in_begining() {
            match self.src.remove(0) {
                '(' => return Token::LeftParenthesis,
                ',' => return Token::Colon,
                ')' => return Token::RightParenthesis,
                ';' => return Token::SemiColon,
                _ => {}
            }
        }
        let mut i = 0;
        while i < self.src.len() {
            let s = self.src[i];
            match s {
                'a'...'z' |
                'A'...'Z' |
                '0'...'9' |
                '_' => i += 1,
                '\'' => {
                    if i == 0 && !self.string_expr {
                        self.src = self.src.split_off(1);
                        self.string_expr = !self.string_expr;
                        return Token::SingleQuote;
                    }
                    else if self.src.len() - i > 1
                            && self.src[i] == '\''
                            && self.src[i+1] == '\''
                            && self.string_expr {
                        self.src.remove(i);
                        i += 1;
                    }
                    else {
                        self.string_expr = !self.string_expr;
                        break;
                    }
                },
                ' ' | '\t' | '\n' => {
                    if i == 0 && !self.string_expr {
                        self.src.remove(0);
                    }
                    else if self.string_expr {
                        i += 1;
                    }
                    else {
                        break;
                    }
                },
                '-' => {
                    if i == 0 && self.src[1] == '-' {
                        let mut j = 2;
                        let mut c = self.src[j];
                        while c != '\n' {
                            j += 1;
                            c = self.src[j];
                        }
                        self.src = self.src.split_off(j);
                    }
                }
                _ => break,
            }
        }
        let mut v = self.src.clone();
        self.src = v.split_off(i);
        Token::Word(v.iter().map(|c| *c).collect::<String>().to_lowercase())
    }

    fn has_lexem_in_begining(&self) -> bool {
        match self.src[0] {
            '(' | ')' | ',' | ';' => true,
            _ => false,
        }
    }

    fn remove_front_spaces(&mut self) {
        while !self.src.is_empty()
                && !self.string_expr
                && (self.src[0] == ' ' 
                || self.src[0] == '\t'
                || self.src[0] == '\n') {
            self.src.remove(0);
        }
    }
}
