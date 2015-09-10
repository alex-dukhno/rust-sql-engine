use sql::statement::InsertStatement;

#[test]
fn test_create_insert_statement_with_complited_information() {
    InsertStatement::new("tab1".to_string(), &(vec!["col1".to_string()]), &(vec!["'val1'".to_string()]));
}
