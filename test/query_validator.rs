use sql::lexer::tokenize;
use sql::parser::parse;
use sql::query_typer::type_inferring;
use sql::query_validator::validate;
use sql::catalog_manager::LockBasedCatalogManager;
use sql::data_manager::LockBaseDataManager;

use super::evaluate_query;

fn assert_that_query_verified_with_error_message(src_query: &str, expected_message: &str, catalog_manager: LockBasedCatalogManager) {
    let query_result = tokenize(src_query)
        .and_then(parse)
        .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
        .and_then(|statement| validate(catalog_manager.clone(), statement));

    match query_result {
        Ok(ret) => panic!("unexpected query validation result {:?}", ret),
        Err(actual_message) => assert_eq!(actual_message, expected_message)
    }
}

#[test]
fn validate_create_already_existed_table() {
    let catalog_manager = LockBasedCatalogManager::default();
    let data_manager = LockBaseDataManager::default();

    drop(evaluate_query("create table table1 (col2 integer);", data_manager, catalog_manager.clone()));

    assert_that_query_verified_with_error_message(
        "create table table1 (col1 integer);",
        "Table <table1> already exists",
        catalog_manager
    );
}

#[test]
fn validate_create_table_with_two_similar_columns() {
    let catalog_manager = LockBasedCatalogManager::default();

    assert_that_query_verified_with_error_message(
        "create table table1(col1 integer, col1 integer);",
        "Column <col1> is already defined in <table1>",
        catalog_manager.clone()
    );
}

#[test]
fn validate_insertion_into_a_table_that_does_not_exist() {
    let catalog_manager = LockBasedCatalogManager::default();

    assert_that_query_verified_with_error_message(
        "insert into table_name values(1);",
        "[ERR 100] table 'table_name' does not exist",
        catalog_manager
    );
}

#[test]
fn validate_insertion_when_column_type_does_not_match() {
    let catalog_manager = LockBasedCatalogManager::default();
    let data_manager = LockBaseDataManager::default();

    drop(evaluate_query("create table table_name (col integer);", data_manager.clone(), catalog_manager.clone()));

    assert_that_query_verified_with_error_message(
        "insert into table_name values('string');",
        "column type is INT find VARCHAR",
        catalog_manager
    );
}
