use std::boxed::Box;
use std::borrow::Borrow;
use std::vec::Vec;
use std::option::Option;

use std::string::String;

pub struct SQLNode {
    value: String,
    start_at: i32,
    end_at: i32,
}

impl SQLNode {

    // pub fn new(value: String) -> SQLNode {
        // SQLNode { value: value, next: Option::None }
    // }

    // pub fn get_string(&self) -> String {
        // self.value.clone()
    // }
}

pub struct InsertStatementNode {
    insert_node: SQLNode,
    into_node: SQLNode,
    table_name_node: SQLNode,
    columns_nodes: Vec<SQLNode>,

}
