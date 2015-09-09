use std::boxed::Box;
use std::borrow::Borrow;

use std::option::Option;

use std::string::String;

pub struct SQLAbstractTreeWalker {
    head: Option<Box<SQLNode>>,
}

impl SQLAbstractTreeWalker {

    pub fn new(value: String) ->SQLAbstractTreeWalker {
        SQLAbstractTreeWalker { head : Some(Box::new(SQLNode::new(value))) }
    }

    // pub fn next(&mut self) -> Option<Box<SQLNode>> {
        // let node = self.head.borrow();
        // self.head = match self.head.next {
            // Some(n) => Some(n.borrow()),
            // None => None
        // }
        // Some(node)
    // }
}

pub struct SQLNode {
    value: String,
    pub next: Option<Box<SQLNode>>,
}

impl SQLNode {

    pub fn new(value: String) -> SQLNode {
        SQLNode { value: value, next: Option::None }
    }

    pub fn get_string(&self) -> String {
        self.value.clone()
    }
}
