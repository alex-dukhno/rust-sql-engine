use sql::lexer::Scanner;
use sql::ast::KeyWord;

#[test]
#[ignore]
fn test_parse_insert_query() {
    let query = "inser into tab1(col1)values('1');";
    let lexer = Scanner::new(query);
/*    let insert = parse_query(&lexer);
    assert_eq!(insert.into.table_name, "tab1".to_string());
    assert!(insert.into.columns_names.is_some());
    assert_eq!(insert.into.columns_names.unwrap(), vec!["col1".to_string()]);
    assert_eq!(insert.values.columns_values, vec!["'1'".to_string()]);*/
}
