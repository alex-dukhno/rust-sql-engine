use sql::ast::SQLNode;

// use sql::parser::parse_query;

#[test]
#[ignore]
fn test_parse_string_to_sql_ast() {
    // let result = parse_query("select t1.c1 from table1 t1".to_string());
    // assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_parse_simpliest_query_string() {
    // let result = parse_query("select 1".to_string());
    // let select_node = result.ok();
    // assert!(select_node.is_some());
    // assert_eq!(select_node.next().unwrap(), "select".to_string());
}

#[test]
#[ignore]
fn test_parse_simpliest_complete_insert_query_string() {
    // let result = parse_insert_query("insert into tab1 (col1) values ('1')".to_string());
    // let statement = result.ok();
}

#[test]
#[ignore]
fn test_create_sql_node() {
    // SQLNode::new("value".to_string());
}
