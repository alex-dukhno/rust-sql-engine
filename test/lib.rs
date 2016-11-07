#[macro_use(expect)]
extern crate expectest;

extern crate sql;

pub mod lexer;
pub mod parser;
pub mod query_typer;
pub mod query_validator;
pub mod query_executer;
pub mod catalog_manager;
pub mod data_manager;

use sql::lexer::tokenize;
use sql::parser::parse;
use sql::query_typer::type_inferring;
use sql::query_validator::validate;
use sql::query_executer::{execute, ExecutionResult};
use sql::data_manager::DataManager;
use sql::catalog_manager::CatalogManager;

pub fn evaluate_query(query: &str, data_manager: &DataManager, catalog_manager: &CatalogManager) -> Result<ExecutionResult, String> {
    tokenize(query)
        .and_then(parse)
        .and_then(|statement| type_inferring(catalog_manager, statement))
        .and_then(|statement| validate(catalog_manager, statement))
        .and_then(|statement| execute(catalog_manager, data_manager, statement))
}
