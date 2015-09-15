use std::iter::Iterator;
use std::str::Chars;
use std::option::Option;
use std::vec::Vec;

pub struct Tokenizer<'a> {
    src: &'a str,
    delimeters: Vec<char>,
}

impl<'a> Tokenizer<'a> {

    pub fn new(src: &'a str) -> Tokenizer {
        Tokenizer { src: src, delimeters: vec![' ', '\t', '\n'] }
    }
}

impl<'a> Iterator for Tokenizer<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let mut delimeter_index = 0;
        for c in self.src.chars() {
            if delimeter_index == 0
                    && self.delimeters.contains(&c) {
                let result = &(self.src)[0..1];
                self.src = &(self.src)[1..self.src.len()];
                return Option::Some(result);
            }
            if self.delimeters.contains(&c) {
                let result = &(self.src)[0..delimeter_index];
                self.src = &(self.src)[delimeter_index..self.src.len()];
                return Option::Some(result)
            }
            delimeter_index += 1;
        }
        let result = &(self.src)[0..delimeter_index];
        Option::Some(result)
    }
}
