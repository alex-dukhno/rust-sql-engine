use expectest::prelude::be_equal_to;

use sql::catalog_manager::LockBasedCatalogManager;
use sql::lexer::{Tokenizer, IntoTokenizer};
use sql::parser::{QueryParser, IntoQueryParser};
use sql::parser::ast::InsertQuery;
use sql::parser::ast::Statement::Insert;
use sql::parser::ast::Value;
use sql::parser::ast::ValueSource;
use sql::parser::ast::Type;

#[test]
fn populates_columns_for_insert_query() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table2");
    catalog_manager.add_column_to("table2", ("col1", Type::Int, None));
    catalog_manager.add_column_to("table2", ("col2", Type::Int, None));
    catalog_manager.add_column_to("table2", ("col3", Type::Int, None));

    expect!(String::from("insert into table2 values (1, 2, 3);").into_tokenizer().tokenize().into_parser().parse().populate(&catalog_manager))
        .to(
            be_equal_to(
                Insert(
                    InsertQuery::new(
                        "table2",
                        vec!["col1", "col2", "col3"],
                        ValueSource::Row(vec![Value::num("1"), Value::num("2"), Value::num("3")])
                    )
                )
            )
        );
}

#[test]
fn populate_only_missed_column() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table_1");
    catalog_manager.add_column_to("table_1", ("col1", Type::Int, Some("1")));
    catalog_manager.add_column_to("table_1", ("col2", Type::Int, None));

    expect!(String::from("insert into table_1 (col2) values (2);").into_tokenizer().tokenize().into_parser().parse().populate(&catalog_manager))
        .to(
            be_equal_to(
                Insert(
                    InsertQuery::new(
                        "table_1",
                        vec!["col2", "col1"],
                        ValueSource::Row(vec![Value::num("2"), Value::num("1")])
                    )
                )
            )
        );
}

#[test]
fn populate_default_value_for_different_types() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table_2");
    catalog_manager.add_column_to("table_2", ("col1", Type::Int, Some("1")));
    catalog_manager.add_column_to("table_2", ("col2", Type::Int, None));
    catalog_manager.add_column_to("table_2", ("col3", Type::VarChar(3), Some("str")));

    expect!(String::from("insert into table_2 (col2) values (2);").into_tokenizer().tokenize().into_parser().parse().populate(&catalog_manager))
        .to(
            be_equal_to(
                Insert(
                    InsertQuery::new(
                        "table_2",
                        vec!["col2", "col1", "col3"],
                        ValueSource::Row(vec![Value::num("2"), Value::num("1"), Value::str("str")])
                    )
                )
            )
        );
}