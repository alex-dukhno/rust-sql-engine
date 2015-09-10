use sql::ast::ASTNode;

#[test]
fn test_create_ast_node() {
    ASTNode::new("value".to_string(), 0, 4);
}

#[test]
#[should_panic(expected = "value's length is different to node length")]
fn test_create_ast_node_with_less_then_should_be_end_at_value() {
    ASTNode::new("value".to_string(), 0, 1);
}
