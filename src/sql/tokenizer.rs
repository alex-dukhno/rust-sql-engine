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

    fn is_char_types_changed(&self, current_char: &char) -> bool{
        !self.is_spaces_token() && self.white_spaces.contains(current_char)
                || self.is_spaces_token() && !self.white_spaces.contains(current_char)
    }

    fn find_delimeter_index(&self) -> usize {
        self.src.chars().take_while(|c| !self.is_char_types_changed(&c)).count()
    }

    fn is_spaces_token(&self) -> bool {
        self.white_spaces.contains(&(self.src.chars().next().unwrap()))
    }
}

impl<'a> Iterator for Tokenizer<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        println!("43 start next");
        if self.src.is_empty() {
            return Option::None
        }
        let delimeter_index = self.find_delimeter_index();
        println!("48 delimeter index - '{}'", delimeter_index);
        let result = &(self.src)[0..delimeter_index];
        println!("50 result - '{}'", result);
        self.src = &(self.src)[delimeter_index..self.src.len()];
        Option::Some(result)
    }
}
