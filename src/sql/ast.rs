use std::boxed::Box;
use std::borrow::Borrow;
use std::vec::Vec;
use std::option::Option;

use std::string::String;

pub struct ASTNode {
    value: String,
    start_at: i32,
    end_at: i32,
}

impl ASTNode {

    pub fn new(value: String, start_at: i32, end_at: i32) -> ASTNode {
        ASTNode { value: value, start_at: start_at, end_at: end_at }
    }

    // pub fn get_string(&self) -> String {
        // self.value.clone()
    // }
}

// pub struct InsertStatementNode {
    // insert_node: SQLNode,
    // into_node: SQLNode,
    // table_name_node: SQLNode,
    // columns_nodes: Option<Vec<SQLNode>>,

// }
