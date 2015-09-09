extern crate sql_ast;

use sql_ast::SQLQueryParser;

#[test]
fn test_create_sql_query_parser() {
    SQLQueryParser::new("test".to_string());
}
