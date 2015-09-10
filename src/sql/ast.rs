use std::boxed::Box;
use std::borrow::Borrow;
use std::vec::Vec;
use std::option::Option;
use std::marker::Sized;
use std::string::String;

use super::error::SQLError;

pub type InsertParserResult = Result<ASTInsertStatementNode, SQLError>;

pub struct ASTNode {
    value: String,
    start_at: usize,
    end_at: usize,
}

impl ASTNode {

    pub fn new(value: String, start_at: usize, end_at: usize) -> ASTNode {
        if value.len() != end_at - start_at + 1 {
            panic!("value's length is different to node length");
        }
        ASTNode { value: value, start_at: start_at, end_at: end_at }
    }

    // pub fn get_string(&self) -> String {
        // self.value.clone()
    // }
}

pub struct ASTInsertStatementNode {
    insert_node: ASTNode,
    into_node: ASTNode,
    table_name_node: ASTNode,
    columns_nodes: Option<Vec<ASTNode>>,
    values_nodes: Vec<ASTNode>,
}

impl ASTInsertStatementNode {

    pub fn new(
            insert_node: ASTNode,
            into_node: ASTNode,
            table_name_node: ASTNode,
            columns_nodes: Option<Vec<ASTNode>>,
            values_nodes: Vec<ASTNode>) -> ASTInsertStatementNode {
        ASTInsertStatementNode { insert_node: insert_node, into_node: into_node, table_name_node: table_name_node, columns_nodes: columns_nodes, values_nodes: values_nodes }
    }
}
