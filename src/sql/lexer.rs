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
}

impl Lexer {
    
    pub fn new(line: &str) -> Lexer {
        Lexer {
            src: line.chars().collect::<Vec<char>>()
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
            let s = self.src.remove(0);
            println!("38 s - {:?}", s);
            match s {
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
            println!("50 s - {:?}", s);
            match s {
                'a'...'z' |
                'A'...'Z' |
                '0'...'9' |
                '_' => i += 1,
                '\'' => {
                    if i == 0 {
                        self.src = self.src.split_off(1);
                        return Token::SingleQuote;
                    }
                    else if self.src[i] == '\''
                            && self.src[i+1] == '\'' {
                        self.src.remove(i);
                        i += 1;
                    }
                    else {
                        break;
                    }
                },
                ' ' | '\t' | '\n' => { if i == 0 { self.src.remove(0); } else { break; } },
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
                && (self.src[0] == ' ' 
                || self.src[0] == '\t'
                || self.src[0] == '\n') {
            self.src.remove(0);
        }
    }
}
