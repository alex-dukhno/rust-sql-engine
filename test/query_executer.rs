use sql::lexer::tokenize;
use sql::parser::parse;
use sql::query_typer::type_inferring;
use sql::query_validator::validate;
use sql::query_executer::{execute, ExecutionResult};
use sql::catalog_manager::LockBasedCatalogManager;
use sql::data_manager::LockBaseDataManager;

pub fn evaluate(
    query: &str,
    data_manager: LockBaseDataManager,
    catalog_manager: LockBasedCatalogManager)
    -> Result<ExecutionResult, String> {
    tokenize(query)
        .and_then(parse)
        .and_then(|statement| type_inferring(catalog_manager.clone(), statement))
        .and_then(|statement| validate(catalog_manager.clone(), statement))
        .and_then(|statement| execute(catalog_manager.clone(), data_manager.clone(), statement))
}

#[cfg(test)]
mod data_definition_language {
    #[cfg(test)]
    mod create_table {
        use expectest::prelude::be_ok;

        use sql::query_executer::ExecutionResult;
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        use super::super::evaluate;

        #[test]
        fn single_column() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            expect!(evaluate("create table table_name (col integer);", data_manager, catalog_manager))
                .to(
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
                evaluate("create table table_name (col1 integer, col2 integer, col3 integer);", data_manager, catalog_manager)
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

        use sql::query_executer::ExecutionResult;
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        use super::super::evaluate;

        #[test]
        fn row_in_created_table() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate("create table table_name (col integer);", data_manager.clone(), catalog_manager.clone()));

            expect!(evaluate("insert into table_name values(1);", data_manager, catalog_manager))
                .to(
                    be_ok().value(
                        ExecutionResult::Message("row was inserted".to_owned())
                    )
                );
        }

        #[test]
        fn row_in_table_with_many_columns() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                evaluate(
                    "create table table_name (col1 integer, col2 integer);",
                    data_manager.clone(),
                    catalog_manager.clone()
                )
            );

            expect!(evaluate("insert into table_name values(1, 2);", data_manager, catalog_manager))
                .to(
                    be_ok().value(
                        ExecutionResult::Message("row was inserted".to_owned())
                    )
                );
        }

        #[test]
        fn does_not_insert_into_table_that_does_not_exist() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            expect!(evaluate("insert into table_name values(1);", data_manager, catalog_manager))
                .to(
                    be_ok().value(
                        ExecutionResult::Message("[ERR 100] table 'table_name' does not exist".to_owned())
                    )
                );
        }

        #[test]
        fn does_not_insert_when_column_type_does_not_match() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate("create table table_name (col integer);", data_manager.clone(), catalog_manager.clone()));

            expect!(evaluate("insert into table_name values('string');", data_manager, catalog_manager))
                .to(
                    be_ok().value(
                        ExecutionResult::Message("column type is INT find VARCHAR".to_owned())
                    )
                );
        }

        #[test]
        #[ignore]
        fn into_table_with_select() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate("create table table_name (col1 integer, col2 integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_name values(1, 2);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_name values(3, 4);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_name values(5, 6);", data_manager.clone(), catalog_manager.clone()));

            expect!(
                evaluate(
                    "insert into table_name (col1, col2) select col1, col2 from table_name;",
                     data_manager,
                     catalog_manager
                )
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

        use sql::query_executer::ExecutionResult;
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        use super::super::evaluate;

        #[test]
        #[ignore]
        fn from_table() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate("create table table_name (col integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_name values(1);", data_manager.clone(), catalog_manager.clone()));

            expect!(evaluate("select col from table_name;", data_manager.clone(), catalog_manager.clone()))
                .to(
                    be_ok().value(
                        ExecutionResult::Data(vec![vec!["1".to_owned()]])
                    )
                );

            drop(evaluate("insert into table_name values(2);", data_manager.clone(), catalog_manager.clone()));

            expect!(evaluate("select col from table_name;", data_manager, catalog_manager))
                .to(
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

            drop(evaluate("create table table_name_2 (col integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_name_2 values(1);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_name_2 values(2);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_name_2 values(3);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_name_2 values(4);", data_manager.clone(), catalog_manager.clone()));

            expect!(evaluate("select col from table_name_2 where limit = 3;", data_manager, catalog_manager))
                .to(
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

            drop(evaluate("create table table_1 (col character(1));", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_1 values (\'a\');", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into table_1 values (\'b\');", data_manager.clone(), catalog_manager.clone()));

            expect!(evaluate("select col from table_1 where col <> \'a\';", data_manager, catalog_manager))
                .to(
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

            drop(evaluate("create table tab1 (col_1 integer, co_2 integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into tab1 values(1, 2);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate("insert into tab1 values(3, 4);", data_manager.clone(), catalog_manager.clone()));

            expect!(evaluate("select col_1 from tab1;", data_manager, catalog_manager))
                .to(
                    be_ok().value(
                        ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["3".to_owned()]])
                    )
                );
        }
    }
}
