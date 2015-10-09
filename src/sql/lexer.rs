use std::option::Option;
use std::iter::Iterator;

pub struct Scanner<'a> {
    src: &'a str,
    vertical_white_spaces: Vec<char>,
    special_chars: Vec<char>,
}

impl<'a> Scanner<'a> {

    pub fn new(src: &'a str) -> Scanner {
        Scanner {
                src: src,
                vertical_white_spaces: vec![' ', '\t'],
                special_chars: vec!['(', ')', '\'', ';']
        }
    }

    fn first_char(&self) -> char {
        self.src.chars().nth(0).unwrap()
    }

    fn next_lexem(&mut self) -> Option<&'a str> {
        let first_char = self.first_char();
        let index = if first_char.is_alphabetic() || first_char.is_digit(10) {
            self.src.chars().take_while(|c| !self.vertical_white_spaces.contains(c) && *c != '\n' && !self.special_chars.contains(c)).count()
        }
        else if self.vertical_white_spaces.contains(&first_char) {
            self.src.chars().take_while(|c| self.vertical_white_spaces.contains(c)).count()
        }
        else if self.special_chars.contains(&first_char) {
            1
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
