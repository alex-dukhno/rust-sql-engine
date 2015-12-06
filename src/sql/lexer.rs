#[derive(PartialEq, Debug)]
pub enum Token {
    Word(String),

    //characters
    LeftParenthesis,
    RightParenthesis,
    SemiColon,
    SingleQuote,
    Colon
}

#[derive(PartialEq, Debug)]
enum QueryParseStatus {
    EmptyLine,
    StringExpression,
    QueryBody
}

pub struct Lexer {
    src: Vec<char>,
    status: QueryParseStatus
}

impl Lexer {
    
    pub fn new(line: &str) -> Lexer {
        let src = line.chars().collect::<Vec<char>>();
        Lexer {
            src: src,
            status: if line.is_empty() { QueryParseStatus::EmptyLine } else { QueryParseStatus::QueryBody }
        }
    }

    pub fn next_lexem(&mut self) -> Option<Token> {
        match self.status {
            QueryParseStatus::EmptyLine => None,
            QueryParseStatus::StringExpression => self.parse_word_token(),
            QueryParseStatus::QueryBody => self.parse_non_word_token(),
        }
    }

    fn parse_word_token(&mut self) -> Option<Token> {
        let mut i = 0;
        while i < self.src.len() 
                && !self.is_string_expression_delimeter(i) {
            if self.is_single_quote_escape(i) {
                self.src.remove(i);
            }
            i += 1;
        }
        Some(Token::Word(self.parse_word_lexem(i)))
    }

    fn parse_word_lexem(&mut self, index: usize) -> String {
        let mut v = self.src.clone();
        self.src = v.split_off(index);
        self.change_query_status(None);
        v.iter().map(|c| *c).collect::<String>().to_lowercase()
    }

    fn change_query_status(&mut self, symbol: Option<char>) {
        if self.src.is_empty() {
            self.status = QueryParseStatus::EmptyLine;
        }
        else if symbol == Some('\'') {
            self.status = QueryParseStatus::StringExpression;
        }
        else {
            self.status = QueryParseStatus::QueryBody;
        }
    }

    fn is_single_quote_escape(&self, index: usize) -> bool {
        self.src.len() - index > 1
            && self.src[index] == '\''
            && self.src[index + 1] == '\''
    }

    fn is_string_expression_delimeter(&self, index: usize) -> bool {
        self.src.len() - index <= 1
            && self.src[index] == '\''
    }

    fn parse_non_word_token(&mut self) -> Option<Token> {
        self.remove_front_spaces();
        self.change_query_status(None);
        if self.status == QueryParseStatus::EmptyLine {
            None
        }
        else {
            self.parse_lexem()
        }
    }

    fn parse_lexem(&mut self) -> Option<Token> {
        if self.has_lexem_in_begining() {
            let s = self.src.remove(0);
            self.change_query_status(Some(s));
            match s {
                '(' => return Some(Token::LeftParenthesis),
                ',' => return Some(Token::Colon),
                ')' => return Some(Token::RightParenthesis),
                ';' => return Some(Token::SemiColon),
                '\'' => return Some(Token::SingleQuote),
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
                ' ' | '\t' | '\n' => {
                    if i == 0 {
                        println!("119 going to remove spaces");
                        self.remove_front_spaces();
                    }
                    else {
                        break;
                    }
                },
                '-' => {
                    if i == 0 && self.src[1] == '-' {
                        let comment_length = self.src.iter().take_while(|c| **c != '\n').count();
                        self.src = self.src.split_off(comment_length);
                    }
                },
                '/' => {
                    if i == 0 && self.src[1] == '*' {
                        let mut comment_length = 2;
                        let mut c = self.src[comment_length];
                        let mut c_next = self.src[comment_length+1];
                        while c != '*'
                                && c_next != '/' {
                            comment_length += 1;
                            c = self.src[comment_length];
                            c_next = self.src[comment_length+1];
                        }
                        self.src = self.src.split_off(comment_length+2);
                    }
                },
                _ => break,
            }
        }
        Some(Token::Word(self.parse_word_lexem(i)))
    }

    fn has_lexem_in_begining(&self) -> bool {
        match self.src[0] {
            '(' | ')' | ',' | ';' | '\'' => true,
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
