use std::option::Option;
use std::vec::Vec;

pub struct Lexer<'a> {
    src: &'a str,
    white_spaces: Vec<char>,
    special_chars: Vec<char>,
}

impl<'a> Lexer<'a> {

    pub fn new(src: &'a str) -> Lexer {
        Lexer {
                src: src,
                white_spaces: vec![' ', '\t', '\n'],
                special_chars: vec!['!', '?', '%', '(', ')', '\'', '"', '>', '<', '=', '+', '-', '*', '/', '\\']
        }
    }

    fn is_char_types_changed(&self, current_char: &char) -> bool {
        let first_char = &(self.take_first_char());
        // println!("first char is - '{}', current char is - '{}'", first_char, current_char);
        self.is_ascii_symbol(first_char) && (self.is_white_space(current_char) || self.is_special_char(current_char))
            || self.is_white_space(first_char) && (self.is_ascii_symbol(current_char) || self.is_special_char(current_char) || first_char != current_char)
            || self.is_special_char(first_char) && (self.is_ascii_symbol(current_char) || self.is_white_space(current_char) || first_char != current_char)
    }

    fn is_ascii_symbol(&self, c: &char) -> bool {
        !self.special_chars.contains(&c) && !self.white_spaces.contains(&c)
    }

    fn is_white_space(&self, c: &char) -> bool {
        self.white_spaces.contains(c)
    }

    fn is_special_char(&self, c: &char) -> bool {
        self.special_chars.contains(c)
    }

    fn find_delimeter_index(&self) -> usize {
        self.src.chars().take_while(|c| !self.is_char_types_changed(&c)).count()
    }

    fn take_first_char(&self) -> char {
        self.src.chars().next().unwrap()
    }

    fn next_lexem(&mut self) -> Option<&'a str> {
        let delimeter_index = self.find_delimeter_index();
        let result = self.take_lexem(&delimeter_index);
        self.resize_src(&delimeter_index);
        result
    }

    fn take_lexem(&self, delimeter_index: &usize) -> Option<&'a str> {
        Option::Some(&(self.src)[0..*delimeter_index])
    }

    fn resize_src(&mut self, delimeter_index: &usize) {
        self.src = &(self.src)[*delimeter_index..self.src.len()];
    }
}

impl<'a> Iterator for Lexer<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.src.is_empty() {
            return Option::None
        }
        self.next_lexem()
    }
}
