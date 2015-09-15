use std::iter::Iterator;
use std::iter::ExactSizeIterator;
use std::str::Chars;
use std::option::Option;
use std::vec::Vec;

pub struct Tokenizer<'a> {
    src: &'a str,
    white_spaces: Vec<char>,
}

impl<'a> Tokenizer<'a> {

    pub fn new(src: &'a str) -> Tokenizer {
        Tokenizer { src: src, white_spaces: vec![' ', '\t', '\n'] }
    }
}

impl<'a> Iterator for Tokenizer<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        println!("24 start next");
        if self.src.is_empty() {
            return Option::None
        }
        let is_spaces = self.white_spaces.contains(&(self.src.chars().next().unwrap()));
        println!("29 is spaces - '{}'", is_spaces);
        let mut delimeter_index = 0;
        for c in self.src.chars() {
            println!("32 current char - '{}'", c);
            if !is_spaces && self.white_spaces.contains(&c)
                    || is_spaces && !self.white_spaces.contains(&c) {
                break;
            }
            delimeter_index += 1;
        }
        println!("40 delimeter index - '{}'", delimeter_index);
        let result = &(self.src)[0..delimeter_index];
        println!("42 result - '{}'", result);
        self.src = &(self.src)[delimeter_index..self.src.len()];
        Option::Some(result)
    }
}
