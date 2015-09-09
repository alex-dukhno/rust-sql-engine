extern crate sql;

use sql::parser::SQLQueryParser;

use sql::ast::SQLNode;

#[test]
fn test_create_sql_query_parser() {
    SQLQueryParser::new("test".to_string());
}

#[test]
fn test_parse_string_to_sql_ast() {
    let result = SQLQueryParser::new("test".to_string())
            .parse_query("select t1.c1 from table1 t1".to_string());
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_parse_simpliest_query_string() {
    let result = SQLQueryParser::new("test".to_string())
            .parse_query("select 1".to_string());
    let select_node = result.ok();
    assert!(select_node.is_some());
    // assert_eq!(select_node.next().unwrap(), "select".to_string());
}

#[test]
fn test_create_sql_node() {
    SQLNode::new("value".to_string());
}
