#[cfg(test)]
mod create_table_query_typer {
    use expectest::prelude::be_ok;

    use sql::lexer::tokenize;
    use sql::parser::parse;
    use sql::query_typer::type_inferring;
    use sql::ast::{TypedStatement, Type};
    use sql::ast::create_table::{CreateTableQuery, ColumnTable};
    use sql::catalog_manager::LockBasedCatalogManager;

    #[test]
    fn default_size_for_char_should_be_255() {
        let catalog_manager = LockBasedCatalogManager::default();

        expect!(
            tokenize("create table tab1 (col1 char);")
                .and_then(parse)
                .and_then(|statement| type_inferring(catalog_manager, statement))
        ).to(
            be_ok().value(
                TypedStatement::Create(
                    CreateTableQuery::new(
                        "tab1",
                        vec![ColumnTable::new("col1", Type::Character(Option::from(255)), false, None, true, None)]
                    )
                )
            )
        );
    }

    #[test]
    fn list_of_columns_with_default_char_size() {
        let catalog_manager = LockBasedCatalogManager::default();

        expect!(
            tokenize("create table tab1 (col1 char, col2 char, col3 char);")
                .and_then(parse)
                .and_then(|statement| type_inferring(catalog_manager, statement))
        ).to(
            be_ok().value(
                TypedStatement::Create(
                    CreateTableQuery::new(
                        "tab1",
                        vec![
                            ColumnTable::new("col1", Type::Character(Option::from(255)), false, None, true, None),
                            ColumnTable::new("col2", Type::Character(Option::from(255)), false, None, true, None),
                            ColumnTable::new("col3", Type::Character(Option::from(255)), false, None, true, None)
                        ]
                    )
                )
            )
        );
    }
}

#[cfg(test)]
mod insert_query_typer {
    use expectest::prelude::be_ok;

    use sql::lexer::tokenize;
    use sql::parser::parse;
    use sql::query_typer::type_inferring;
    use sql::ast::{Type, TypedStatement};
    use sql::ast::insert_query::{InsertQuery, Value, ValueSource};
    use sql::catalog_manager::LockBasedCatalogManager;

    #[test]
    fn populates_columns_for_insert_query() {
        let catalog_manager = LockBasedCatalogManager::default();

        catalog_manager.add_table("table2");
        catalog_manager.add_column_to("table2", ("col1", Type::Integer, None));
        catalog_manager.add_column_to("table2", ("col2", Type::Integer, None));
        catalog_manager.add_column_to("table2", ("col3", Type::Integer, None));

        expect!(
            tokenize("insert into table2 values (1, 2, 3);")
                .and_then(parse)
                .and_then(|statement| type_inferring(catalog_manager, statement))
        ).to(
            be_ok().value(
                TypedStatement::Insert(
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

        expect!(
            tokenize("insert into table_1 (col2) values (2);")
                .and_then(parse)
                .and_then(|statement| type_inferring(catalog_manager, statement))
        ).to(
            be_ok().value(
                TypedStatement::Insert(
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
        catalog_manager.add_column_to("table_2", ("col3", Type::Character(Option::from(3)), Some("str")));

        expect!(
            tokenize("insert into table_2 (col2) values (2);")
                .and_then(parse)
                .and_then(|statement| type_inferring(catalog_manager, statement))
        ).to(
            be_ok().value(
                TypedStatement::Insert(
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
