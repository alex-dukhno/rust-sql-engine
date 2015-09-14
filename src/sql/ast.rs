use std::boxed::Box;
use std::borrow::BorrowMut;
use std::borrow::Borrow;
use std::option::Option;
use std::string::String;

pub struct ASTNode<'a> {
    pub val: &'a str,
    pub left: Box<Option<ASTNode<'a>>>,
    pub right: Box<Option<ASTNode<'a>>>,
}

impl<'a> ASTNode<'a> {

    pub fn new(val: &'a str) -> ASTNode<'a> {
        ASTNode { val: val, left: Box::new(Option::None), right: Box::new(Option::None) }
    }
}
