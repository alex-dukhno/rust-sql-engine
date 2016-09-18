use expectest::prelude::{be_ok, be_err};

use sql::lexer::{Tokenizer, IntoTokenizer};
use sql::parser::{QueryParser, IntoQueryParser};
use sql::ast::Type;
use sql::catalog_manager::LockBasedCatalogManager;
use sql::query_validator::QueryValidator;

#[test]
fn validate_create_table_qury() {
    let catalog_manger = LockBasedCatalogManager::default();

    let query_validator = QueryValidator::new(catalog_manger);

    let statement = String::from("create table table1 (col1 integer);").into_tokenizer().tokenize().into_parser().parse();

    expect!(query_validator.validate(statement.clone()))
        .to(be_ok().value(statement));
}

#[test]
fn validate_create_already_existed_table() {
    let catalog_manager = LockBasedCatalogManager::default();
    catalog_manager.add_table("table1");

    let query_validator = QueryValidator::new(catalog_manager.clone());

    let statement = String::from("create table table1 (col1 integer);").into_tokenizer().tokenize().into_parser().parse();

    expect!(query_validator.validate(statement))
        .to(be_err().value(format!("Table <table1> already exists")));
}

#[test]
#[ignore]
fn validate_create_table_with_foreign_key() {
    let catalog_manager = LockBasedCatalogManager::default();
    catalog_manager.add_table("table1");
    catalog_manager.add_column_to("table1", ("col1", Type::Integer, None));

    let query_validator = QueryValidator::new(catalog_manager.clone());

    let statement = String::from("create table table2 (col2 integer primary key, col3 integer foreign key references table1 (col1));")
        .into_tokenizer().tokenize().into_parser().parse();

    expect!(query_validator.validate(statement.clone()))
        .to(be_ok().value(statement));
}