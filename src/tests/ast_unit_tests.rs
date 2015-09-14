use sql::ast::ASTNode;

#[test]
fn test_create_ast_node() {
    let value = "insert";
    let ast_node = ASTNode::new(value);
    assert_eq!(ast_node.val, value);
    assert!(ast_node.left.is_none());
    assert!(ast_node.right.is_none());
}
