use sql::ast::ASTNode;

use std::option::Option;

#[test]
fn test_create_ast_node() {
    let value = "insert";
    let ast_node = ASTNode::new(value);
    // assert_eq!(ast_node.val(), &value);
    // assert_eq!(ast_node.left(), Option::None());
    // assert_eq!(ast_node.right(), Option::None());
}
