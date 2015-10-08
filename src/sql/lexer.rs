use std::option::Option;
use std::iter::Iterator;

pub struct Scanner<'a> {
    src: &'a str,
}

impl<'a> Scanner<'a> {

    pub fn new(src: &'a str) -> Scanner {
        Scanner {
                src: src,
        }
    }

    fn next_lexem(&mut self) -> Option<&'a str> {
        let chars = self.src.chars();
        let mut index = chars.take_while(|c| *c != ' ').count();
        if index == 0 {
            index += 1;
        }
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
