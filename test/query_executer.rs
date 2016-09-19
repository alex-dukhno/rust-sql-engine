#[cfg(test)]
mod data_definition_language {
    #[cfg(test)]
    mod create_table {
        use expectest::prelude::be_ok;

        use sql::lexer::tokenize;
        use sql::parser::parse;
        use sql::query_typer::type_inferring;
        use sql::query_validator::validate;
        use sql::query_executer::{execute, ExecutionResult};
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        #[test]
        fn single_column() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            expect!(
                tokenize("create table table_name (col integer);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Message("'table_name' was created".to_owned())
                )
            );
        }

        #[test]
        fn with_list_of_columns() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            expect!(
                tokenize("create table table_name (col1 integer, col2 integer, col3 integer);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Message("'table_name' was created".to_owned())
                )
            );
        }
    }
}

#[cfg(test)]
mod data_manipulation_language {
    #[cfg(test)]
    mod inserts {
        use expectest::prelude::be_ok;

        use sql::lexer::tokenize;
        use sql::parser::parse;
        use sql::query_typer::type_inferring;
        use sql::query_validator::validate;
        use sql::query_executer::{execute, ExecutionResult};
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        #[test]
        #[ignore]
        fn row_in_created_table() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                tokenize("create table table_name (col integer);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            expect!(
                tokenize("insert into table_name values(1);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Message("row was inserted".to_owned())
                )
            );
        }

        #[test]
        #[ignore]
        fn row_in_table_with_many_columns() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(tokenize("create table table_name (col1 integer, col2 integer);")
                .and_then(|tokens| parse(tokens))
                .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                .and_then(|statement| validate(catalog_manager.clone(), statement))
                .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement)));

            expect!(
                tokenize("insert into table_name values(1, 2);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Message("row was inserted".to_owned())
                )
            );
        }

        #[test]
        #[ignore]
        fn does_not_insert_into_table_that_does_not_exist() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            expect!(
                tokenize("insert into table_name values(1);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(ExecutionResult::Message("[ERR 100] table 'table_name' does not exist".to_owned())));
        }

        #[test]
        #[ignore]
        fn does_not_insert_when_column_type_does_not_match() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                tokenize("create table table_name (col integer);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            expect!(
                tokenize("insert into table_name values('string');")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(ExecutionResult::Message("column type is INT find VARCHAR".to_owned())
                )
            );
        }

        #[test]
        #[ignore]
        fn into_table_with_select() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                tokenize("create table table_name (col1 integer, col2 integer);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_name values(1, 2);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_name values(3, 4);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_name values(5, 6);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            expect!(
                tokenize("insert into table_name (col1, col2) select col1, col2 from table_name;")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Message("3 rows were inserted".to_owned())
                )
            );
        }
    }

    #[cfg(test)]
    mod selects {
        use expectest::prelude::be_ok;

        use sql::lexer::tokenize;
        use sql::parser::parse;
        use sql::query_typer::type_inferring;
        use sql::query_validator::validate;
        use sql::query_executer::{execute, ExecutionResult};
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        #[test]
        #[ignore]
        fn from_table() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                tokenize("create table table_name (col integer);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_name values(1);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            expect!(
                tokenize("select col from table_name;")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(ExecutionResult::Data(vec![vec!["1".to_owned()]])
                )
            );

            drop(
                tokenize("insert into table_name values(2);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            expect!(
                tokenize("select col from table_name;")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["2".to_owned()]])
                )
            );
        }

        #[test]
        #[ignore]
        fn limit_number_of_rows() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                tokenize("create table table_name_2 (col integer);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_name_2 values(1);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_name_2 values(2);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_name_2 values(3);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_name_2 values(4);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            expect!(
                tokenize("select col from table_name_2 where limit = 3;")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["2".to_owned()], vec!["3".to_owned()]])
                )
            );
        }

        #[test]
        #[ignore]
        fn by_column_predicate() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                tokenize("create table table_1 (col character(1));")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_1 values (\'a\');")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into table_1 values (\'b\');")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            expect!(
                tokenize("select col from table_1 where col <> \'a\';")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Data(vec![vec!["b".to_owned()]])
                )
            );
        }

        #[test]
        #[ignore]
        fn column_from_table_with_list_of_columns() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                tokenize("create table tab1 (col_1 integer, co_2 integer);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into tab1 values(1, 2);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            drop(
                tokenize("insert into tab1 values(3, 4);")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            );

            expect!(
                tokenize("select col_1 from tab1;")
                    .and_then(|tokens| parse(tokens))
                    .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
                    .and_then(|statement| validate(catalog_manager.clone(), statement))
                    .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
            ).to(
                be_ok().value(
                    ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["3".to_owned()]])
                )
            );
        }
    }
}
