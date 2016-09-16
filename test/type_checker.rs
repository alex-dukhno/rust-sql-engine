#[cfg(test)]
mod insert_query_checker {
    use expectest::prelude::be_equal_to;

    use sql::catalog_manager::LockBasedCatalogManager;
    use sql::lexer::{Tokenizer, IntoTokenizer};
    use sql::parser::{QueryParser, IntoQueryParser};
    use sql::ast::Statement::Insert;
    use sql::ast::{InsertQuery, Value, ValueSource, Type};
    use sql::type_checker::QueryChecker;

    #[test]
    fn populates_columns_for_insert_query() {
        let catalog_manager = LockBasedCatalogManager::default();

        catalog_manager.add_table("table2");
        catalog_manager.add_column_to("table2", ("col1", Type::Integer, None));
        catalog_manager.add_column_to("table2", ("col2", Type::Integer, None));
        catalog_manager.add_column_to("table2", ("col3", Type::Integer, None));

        let statement = String::from("insert into table2 values (1, 2, 3);").into_tokenizer().tokenize().into_parser().parse();

        let query_checker = QueryChecker::new(catalog_manager.clone());

        expect!(query_checker.check(statement))
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
    fn populates_only_missed_column() {
        let catalog_manager = LockBasedCatalogManager::default();

        catalog_manager.add_table("table_1");
        catalog_manager.add_column_to("table_1", ("col1", Type::Integer, Some("1")));
        catalog_manager.add_column_to("table_1", ("col2", Type::Integer, None));

        let statement = String::from("insert into table_1 (col2) values (2);").into_tokenizer().tokenize().into_parser().parse();

        let query_checker = QueryChecker::new(catalog_manager.clone());

        expect!(query_checker.check(statement))
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
    fn populates_default_value_for_different_types() {
        let catalog_manager = LockBasedCatalogManager::default();

        catalog_manager.add_table("table_2");
        catalog_manager.add_column_to("table_2", ("col1", Type::Integer, Some("1")));
        catalog_manager.add_column_to("table_2", ("col2", Type::Integer, None));
        catalog_manager.add_column_to("table_2", ("col3", Type::VarChar(3), Some("str")));

        let statement = String::from("insert into table_2 (col2) values (2);").into_tokenizer().tokenize().into_parser().parse();

        let query_checker = QueryChecker::new(catalog_manager.clone());

        expect!(query_checker.check(statement))
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
}
