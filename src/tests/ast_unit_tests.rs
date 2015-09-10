use sql::ast::ASTNode;
use sql::ast::InsertStatementNode;

use std::option::Option;

#[test]
fn test_create_ast_node() {
    ASTNode::new("value".to_string(), 0, 4);
}

#[test]
#[should_panic(expected = "value's length is different to node length")]
fn test_create_ast_node_with_less_then_should_be_end_at_value() {
    ASTNode::new("value".to_string(), 0, 1);
}

#[test]
fn test_create_insert_ast_node() {
    InsertStatementNode::new(
            ASTNode::new("insert".to_string(), 0, 5),
            ASTNode::new("into".to_string(), 7, 10),
            ASTNode::new("tab1".to_string(), 11, 14),
            Option::None,
            vec![ASTNode::new("'1'".to_string(), 17, 19)]
    );
}
