use std::boxed::Box;
use std::borrow::BorrowMut;
use std::borrow::Borrow;
use std::option::Option;
use std::string::String;
use std::vec::Vec;

pub struct ASTNode<'a> {
    vals: Vec<&'a str>,
    index: usize,
    pub left: Box<Option<ASTNode<'a>>>,
    pub right: Box<Option<ASTNode<'a>>>,
}

impl<'a> ASTNode<'a> {

    pub fn new(vals: Vec<&'a str>) -> ASTNode<'a> {
        ASTNode { vals: vals, index: 0, left: Box::new(Option::None), right: Box::new(Option::None) }
    }

    pub fn current_val(&self) -> &'a str {
        self.vals[self.index]
    }

    pub fn next_val(&mut self) -> Option<&'a str> {
        if self.index == usize::max_value() {
            return Option::None;
        }
        let v = self.vals.get(self.index + 1);
        if v.is_some() {
            self.index += 1;
            return Option::Some(*(v.unwrap()));
        }
        Option::None
    }

    pub fn prev_val(&mut self) -> Option<&'a str> {
        if self.index == 0 {
            return Option::None;
        }
        let v = self.vals.get(self.index - 1);
        if v.is_some() {
            self.index -= 1;
            return Option::Some(*(v.unwrap()));
        }
        Option::None
    }
}

pub fn parse_query<'a>(query_string: &'a str) -> ASTNode<'a> {
    let mut c = query_string.chars().peekable();
    let mut last_index = 0;
    // let mut head = Option::None;
    // while c.peek().is_some() {
        let index = c.position(|c| c == ' ').unwrap();
        let v = &query_string[last_index..index];
        let node = ASTNode::new(vec![v]);
        last_index = index + 2;
        // if head.is_none() {
            let head = Option::Some(node);
        // }
        // else {
            // head.unwrap().left = Box::new(Option::Some(node));
        // }
    // }
    head.unwrap()
}
