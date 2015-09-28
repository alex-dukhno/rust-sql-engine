use std::option::Option;
use std::vec::Vec;
use std::iter::Iterator;
use std::collections::HashMap;

pub struct Scanner<'a> {
    src: &'a str,
    white_spaces: Vec<char>,
    special_chars: Vec<char>,
    peek_next: HashMap<char, char>,
}

impl<'a> Scanner<'a> {

    pub fn new(src: &'a str) -> Scanner {
        let mut peek_next = HashMap::new();
        peek_next.insert('-', '-');
        Scanner {
                src: src,
                white_spaces: vec![' ', '\t', '\n'],
                special_chars: vec!['!', '?', '%', '(', ')', '\'', '"', '>', '<', '=', '+', '-', '*', '/', '\\'],
                peek_next: peek_next,
        }
    }

    fn is_char_types_changed(&self, current_char: &char) -> bool {
        self.is_char_type_change_from_ascii_symbol(current_char)
            || self.is_char_type_change_from_white_space(current_char)
            || self.is_char_type_change_from_special_char(current_char)
    }

    fn is_char_type_change_from_ascii_symbol(&self, current_char: &char) -> bool {
        let first_char = &(self.take_first_char());
        println!("first char is - '{}', current char is - '{}'", first_char, current_char);
        self.is_ascii_symbol(first_char)
            && (self.is_white_space(current_char) || self.is_special_char(current_char))
    }

    fn is_char_type_change_from_white_space(&self, current_char: &char) -> bool {
        let first_char = &(self.take_first_char());
        println!("first char is - '{}', current char is - '{}'", first_char, current_char);
        self.is_white_space(first_char)
            && (self.is_ascii_symbol(current_char) || self.is_special_char(current_char) || current_char != first_char)
    }

    fn is_char_type_change_from_special_char(&self, current_char: &char) -> bool {
        let first_char = &(self.take_first_char());
        println!("first char is - '{}', current char is - '{}'", first_char, current_char);
        if self.is_special_char(first_char) {
            if self.peek_next.contains_key(first_char) {
                let v = self.peek_next.get(first_char).unwrap();
                let mut chars = self.src.chars();
                match chars.skip(1).peekable().peek() {
                    Some(n) => {
                        println!("peekable char is '{}'", n);
                        return n != v;
                    },
                    None => return self.is_ascii_symbol(current_char) || self.is_white_space(current_char) || current_char != first_char,
                }
            }
            return self.is_ascii_symbol(current_char) || self.is_white_space(current_char) || current_char != first_char
        }
        false
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

impl<'a> Iterator for Scanner<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.src.is_empty() {
            return Option::None
        }
        self.next_lexem()
    }
}

// "--" -> "\n"
// "/*" -> "*/"
// " "|"\n"|"\t" -> ""

pub struct Evaluator<'a> {
    scanner: Scanner<'a>,
    white_spaces: Vec<&'a str>,
}

impl<'a> Evaluator<'a> {

    pub fn new(src: &'a str) -> Evaluator {
        Evaluator { scanner: Scanner::new(src), white_spaces: vec![" "] }
    }

    fn skip_spaces(&mut self) {
        // let lexem = self.scanner.peekable().peek();
        // match lexem {
            // Some(val) => match *val {
                // " " => self.scanner.skip_while(|s| self.skip_while_not_a_space(s)),
                // _ => self.scanner.skip_while(self.nothing_to_do("")),
            // },
            // None => self.scanner.skip_while(self.nothing_to_do("")),
        // }
    }

/*    fn nothing_to_do(&self, s: &'a str) -> bool {
        false
    }

    fn skip_while_not_a_space(&self, s: &'a str) -> bool {
        self.white_spaces.contains(s)
    }*/
}

impl<'a> Iterator for Evaluator<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.skip_spaces();
        let lexem = self.scanner.next();
        match lexem {
            Some(val) => if val != " " { lexem } else { self.next() } ,
            None => lexem
        }
    }
}
