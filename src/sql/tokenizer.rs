use std::iter::Iterator;
use std::str::Chars;
use std::option::Option;
use std::vec::Vec;

pub struct Tokenizer<'a> {
    src: &'a str,
    index: usize,
    delimeters: Vec<char>,
}

impl<'a> Tokenizer<'a> {

    pub fn new(src: &'a str) -> Tokenizer {
        Tokenizer { src: src, index: 0, delimeters: vec![' '] }
    }
}

impl<'a> Iterator for Tokenizer<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        println!("strat next");
        let mut chars = self.src.chars();
        let nth = chars.nth(self.index);
        if nth.is_some() && self.delimeters.contains(&(nth.unwrap())) {
            println!("nth - '{}'", nth.unwrap());
            let result = &(self.src)[self.index..self.index + 1];
            self.index += 1;
            return Option::Some(result);
        }
        let position = chars.position(|c| self.delimeters.contains(&c));
        println!("position - '{}'", position.unwrap());
        if position.is_some() {
            let delimeter_index = position.unwrap() + 1;
            let result = &(self.src)[self.index..delimeter_index];
            println!("result - '{}'", result);
            self.index = delimeter_index;
            return Option::Some(result)
        }
        Option::None
    }
}
