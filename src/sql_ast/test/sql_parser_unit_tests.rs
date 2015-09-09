extern crate sql_ast;

use sql_ast::SQLQueryParser;
use sql_ast::SQLNode;

#[test]
fn test_create_sql_query_parser() {
    SQLQueryParser::new("test".to_string());
}

#[test]
fn test_create_sql_node() {
    SQLNode::new("value".to_string());
}
