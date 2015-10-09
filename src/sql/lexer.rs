use std::option::Option;
use std::iter::Iterator;
use std::collections::HashMap;

pub struct Scanner<'a> {
    src: &'a str,
    vertical_white_spaces: Vec<char>,
    single_special_chars: Vec<char>,
    double_special_chars: HashMap<char, char>,
    multi_special_chars: Vec<char>,
}

impl<'a> Scanner<'a> {

    pub fn new(src: &'a str) -> Scanner {
        let mut double_special_chars = HashMap::with_capacity(6);
        double_special_chars.insert('=', '=');
        double_special_chars.insert('/', '*');
        double_special_chars.insert('*', '/');
        double_special_chars.insert('-', '-');
        double_special_chars.insert('>', '=');
        double_special_chars.insert('!', '=');
        Scanner {
                src: src,
                vertical_white_spaces: vec![' ', '\t'],
                single_special_chars: vec!['(', ')', '\'', ';', '.', '+'],
                double_special_chars: double_special_chars,
                multi_special_chars: vec!['*', '-', '/'],
        }
    }

    fn first_char(&self) -> char {
        self.nth_char(0).unwrap()
    }

    fn nth_char(&self, index: usize) -> Option<char> {
        self.src.chars().nth(index)
    }

    fn check_double_spec_chars(&self, first_char: &char, next_char: &char) -> bool {
        match self.double_special_chars.get(first_char) {
            Some(c) => *c == *next_char,
            None => false
        }
    }

    fn next_lexem(&mut self) -> Option<&'a str> {
        let first_char = self.first_char();
        let option_nth_char = self.nth_char(1);
        let index = if first_char.is_alphabetic() || first_char.is_digit(10) {
            self.src.chars().take_while(
                |c| !self.vertical_white_spaces.contains(c)
                    && *c != '\n'
                    && !self.single_special_chars.contains(c)
                    && !self.multi_special_chars.contains(c)
                ).count()
        }
        else if self.vertical_white_spaces.contains(&first_char) {
            self.src.chars().take_while(|c| self.vertical_white_spaces.contains(c)).count()
        }
        else if self.single_special_chars.contains(&first_char) {
            1
        }
        else if self.double_special_chars.contains_key(&first_char) && option_nth_char.is_some() {
            if self.check_double_spec_chars(&first_char, &(option_nth_char.unwrap())) {
                2
            }
            else {
                1
            }
        }
        else if first_char == '<' {
            if  option_nth_char.unwrap() == '>' || option_nth_char.unwrap() == '=' {
                2
            }
            else {
                1
            }
        }
        else {
            self.src.chars().take_while(|c| *c == '\n').count()
        };
        let result = &(self.src)[0..index];
        let src_len = self.src.len();
        self.src = &(self.src)[index..src_len];
        Option::Some(result)
    }
}

impl<'a> Iterator for Scanner<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.src.is_empty() {
            return Option::None
        }
        self.next_lexem()
    }
}
