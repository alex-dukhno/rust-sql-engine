use sql::ast::ASTNode;

#[test]
fn test_create_ast_node() {
    ASTNode::new("value".to_string(), 0, 4);
}
