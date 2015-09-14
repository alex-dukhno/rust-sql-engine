use sql::ast::ASTNode;
// use sql::ast::parse_query;

#[test]
fn test_create_ast_node() {
    let value = "insert";
    let mut ast_node = ASTNode::new(vec![value]);
    assert_eq!(ast_node.current_val(), value);
    assert!(ast_node.left.is_none());
    assert!(ast_node.right.is_none());
}

#[test]
fn test_current_value() {
    let value = "insert";
    let mut ast_node = ASTNode::new(vec![value]);
    assert_eq!(ast_node.current_val(), value);
}

#[test]
fn test_next_value() {
    let value = "insert";
    let mut ast_node = ASTNode::new(vec![value]);
    assert_eq!(ast_node.current_val(), value);
    assert!(ast_node.next_val().is_none());
    assert_eq!(ast_node.current_val(), value);
}

#[test]
fn test_previous_value() {
    let value = "insert";
    let mut ast_node = ASTNode::new(vec![value]);
    assert_eq!(ast_node.current_val(), value);
    assert!(ast_node.prev_val().is_none());
    assert_eq!(ast_node.current_val(), value);
}

#[test]
#[ignore]
fn test_parse_simple_insert() {
    /*let query = "insert into tab1(col1) values('1');";
    let insert_node = parse_query(query);
    assert_eq!(insert_node.val, "insert");
    let into_node = *insert_node.left;
    assert_eq!(insert_node.val, "into");
    let table_node = into_node.left;
    assert_eq!(table_node.val, "tab1");
    let values_node = insert_node.right;
    assert_eq!(values_node.val, "values");*/
}
