use sql::ast::ASTNode;
use sql::ast::parse_query;

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
/*
#[test]
#[ignore]
fn test_parse_insert_node() {
    let query = "insert into tab1(col1) values('1');";
    let mut insert_node = parse_query(query);
    assert_eq!(insert_node.current_val(), "insert");
    assert!(insert_node.next_val().is_none());
    assert!(insert_node.prev_val().is_none());
}

#[test]
#[ignore]
fn test_parse_into_node() {
    let query = "insert into tab1(col1) values('1');";
    let insert_node = parse_query(query);
    let mut into_node = (*insert_node.left).unwrap();
    assert_eq!(into_node.current_val(), "into");
    assert!(into_node.next_val().is_none());
    assert!(into_node.prev_val().is_none());
}

#[test]
#[ignore]
fn test_parse_table_node() {
    let query = "insert into tab1(col1) values('1');";
    let insert_node = parse_query(query);
    let into_node = (*insert_node.left).unwrap();
    let mut table_node = (*into_node.left).unwrap();
    assert_eq!(table_node.current_val(), "tab1");
    assert!(table_node.next_val().is_none());
    assert!(table_node.prev_val().is_none());
}

#[test]
#[ignore]
fn test_parse_simple_columns_node() {
    let query = "insert into tab1(col1) values('1');";
    let insert_node = parse_query(query);
    let into_node = (*insert_node.left).unwrap();
    let mut columns_node = (*into_node.right).unwrap();
    assert_eq!(columns_node.current_val(), "col1");
    assert!(columns_node.next_val().is_none());
    assert!(columns_node.prev_val().is_none());
}

#[test]
#[ignore]
fn test_parse_values_node() {
    let query = "insert into tab1(col1) values('1');";
    let insert_node = parse_query(query);
    let mut values_node = (*insert_node.right).unwrap();
    assert_eq!(values_node.current_val(), "values");
    assert!(values_node.next_val().is_none());
    assert!(values_node.prev_val().is_none());
}

#[test]
#[ignore]
fn test_parse_simple_data_node() {
    let query = "insert into tab1(col1) values('1');";
    let insert_node = parse_query(query);
    let values_node = (*insert_node.right).unwrap();
    let mut data_node = (*values_node.left).unwrap();
    assert_eq!(data_node.current_val(), "'1'");
    assert!(data_node.next_val().is_none());
    assert!(data_node.prev_val().is_none());
}
*/
