use std::boxed::Box;
use std::option::Option;
use std::string::String;

pub struct ASTNode<'a> {
    val: &'a str,
    left: Box<Option<ASTNode<'a>>>,
    right: Box<Option<ASTNode<'a>>>,
}

impl<'a> ASTNode<'a> {

    pub fn new(val: &'a str) -> ASTNode<'a> {
        ASTNode { val: val, left: Box::new(Option::None), right: Box::new(Option::None) }
    }
}
