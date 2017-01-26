use std::collections::HashMap;

use sql::lexer::tokenize;
use sql::parser::parse;
use sql::query_typer::type_inferring;
use sql::query_validator::validate;

use sql::ast::Type;
use sql::catalog::ColumnMetadata;

fn assert_that_query_verified_with_error_message(src_query: &str, expected_message: &str, tables_set: &HashMap<String, Vec<ColumnMetadata>>) {
    match tokenize(src_query)
            .and_then(parse)
            .and_then(|statement| type_inferring(tables_set, statement))
            .and_then(|statement| validate(tables_set, statement)) {
        Ok(ret) => panic!("unexpected query validation result {:?}", ret),
        Err(actual_message) => assert_eq!(actual_message, expected_message)
    }
}

#[test]
fn validate_create_already_existed_table() {
    let mut table = HashMap::new();
    let columns = vec![
        ColumnMetadata::new("col2", Type::Integer, None)
    ];
    table.insert("table1".into(), columns);

    assert_that_query_verified_with_error_message(
        "create table table1 (col1 integer);",
        "Table <table1> already exists",
        &table
    );
}

#[test]
fn validate_create_table_with_two_similar_columns() {
    assert_that_query_verified_with_error_message(
        "create table table1(col1 integer, col1 integer);",
        "Column <col1> is already defined in <table1>",
        &HashMap::new()
    );
}

#[test]
fn validate_insertion_into_a_table_that_does_not_exist() {
    assert_that_query_verified_with_error_message(
        "insert into table_name values(1);",
        "[ERR 100] table 'table_name' does not exist",
        &HashMap::new()
    );
}

#[test]
fn validate_insertion_when_column_type_does_not_match() {
    let mut table = HashMap::new();
    let columns = vec![
        ColumnMetadata::new("col", Type::Integer, None)
    ];
    table.insert("table_name".into(), columns);

    assert_that_query_verified_with_error_message(
        "insert into table_name values('string');",
        "column type is INT find VARCHAR",
        &table
    );
}
