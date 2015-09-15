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
        println!("start next");
        if self.src.is_empty() {
            return Option::None;
        }
        let last = self.src.chars().rev().next().unwrap();
        println!("last is - '{}'", last);
        let mut chars = self.src.chars();
        let delimeter = if self.white_spaces.contains(&last) {
            chars.position(
                |c| {
                    println!("current char - '{}'", c);
                    !(self.white_spaces.contains(&c))
                }
            )
        }
        else {
            chars.position(
                |c| {
                    println!("current char - '{}'", c);
                    self.white_spaces.contains(&c) || c != last
                }
            )
        };
        if delimeter.is_none() {
            return Option::None
        }
        let delimeter_index = delimeter.unwrap();
        println!("delimeter index is - '{}'", delimeter_index);
        let result = &(self.src)[0..delimeter_index];
        println!("result - '{}'", result);
        self.src = &(self.src)[delimeter_index..self.src.len()];
        Option::Some(result)
    }
}
