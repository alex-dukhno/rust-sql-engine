use sql::query_executer::ExecutionResult;
use sql::catalog_manager::LockBasedCatalogManager;
use sql::data_manager::LockBaseDataManager;

use super::evaluate_query;

fn assert_that_query_evaluation_return_message(
        src_query: &str,
        expected_message: &str,
        data_manager: LockBaseDataManager,
        catalog_manager: LockBasedCatalogManager) {
    let execution_result = evaluate_query(src_query, data_manager.clone(), catalog_manager.clone());

    match execution_result {
        Ok(ExecutionResult::Message(actual_message)) => assert_eq!(actual_message, expected_message),
        res => panic!("unexpected query evaluation result {:?}", res)
    }
}

fn assert_that_query_evaluation_return_data(src_query: &str, expected_data: &str, data_manager: LockBaseDataManager, catalog_manager: LockBasedCatalogManager) {
    let execution_result = evaluate_query(src_query, data_manager.clone(), catalog_manager.clone());

    match execution_result {
        Ok(ExecutionResult::Data(data)) => assert_eq!(format!("{:?}", data), expected_data),
        res => panic!("unexpected query evaluation result {:?}", res)
    }
}

#[cfg(test)]
mod data_definition_language {
    #[cfg(test)]
    mod create_table {
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        use super::super::super::evaluate_query;
        use super::super::assert_that_query_evaluation_return_message;

        #[test]
        fn single_column() {
            assert_that_query_evaluation_return_message(
                "create table table_name (col integer);",
                "'table_name' was created",
                LockBaseDataManager::default(),
                LockBasedCatalogManager::default()
            );
        }

        #[test]
        fn with_list_of_columns() {
            assert_that_query_evaluation_return_message(
                "create table table_name (col1 integer, col2 integer, col3 integer);",
                "'table_name' was created",
                LockBaseDataManager::default(),
                LockBasedCatalogManager::default()
            );
        }

        #[test]
        fn with_foreign_key() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table table1 (col1 integer primary key);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_message(
                "create table table2 (col2 integer primary key, col3 integer foreign key references table1 (col1));",
                "'table2' was created",
                LockBaseDataManager::default(),
                catalog_manager
            );
        }
    }
}

#[cfg(test)]
mod data_manipulation_language {
    #[cfg(test)]
    mod inserts {
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        use super::super::super::evaluate_query;
        use super::super::assert_that_query_evaluation_return_message;
        use super::super::assert_that_query_evaluation_return_data;

        #[test]
        fn row_in_created_table() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table table_name (col integer);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_message(
                "insert into table_name values(1);",
                "row was inserted",
                data_manager,
                catalog_manager
            );
        }

        #[test]
        fn row_in_table_with_many_columns() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(
                evaluate_query(
                    "create table table_name (col1 integer, col2 integer);",
                    data_manager.clone(),
                    catalog_manager.clone()
                )
            );

            assert_that_query_evaluation_return_message(
                "insert into table_name values(1, 2);",
                "row was inserted",
                data_manager,
                catalog_manager
            );
        }

        #[test]
        fn into_table_with_select() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table table_name (col1 integer, col2 integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_name values(1, 2);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_name values(3, 4);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_name values(5, 6);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_message(
                "insert into table_name (col1, col2) select col1, col2 from table_name;",
                "3 rows were inserted",
                 data_manager,
                 catalog_manager
            );
        }

        #[test]
        fn column_with_default_value() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table table1 (col1 integer, col2 integer default 1);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_message(
                "insert into table1 values (1);",
                "row was inserted",
                data_manager.clone(), catalog_manager.clone()
            );

            assert_that_query_evaluation_return_data(
                "select col1, col2 from table1;",
                "[[\"1\", \"1\"]]",
                data_manager,
                catalog_manager
            );
        }
    }

    #[cfg(test)]
    mod selects {
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::data_manager::LockBaseDataManager;

        use super::super::super::evaluate_query;
        use super::super::assert_that_query_evaluation_return_data;

        #[test]
        fn from_table() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table table_name (col integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_name values(1);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_data("select col from table_name;", "[[\"1\"]]", data_manager.clone(), catalog_manager.clone());

            drop(evaluate_query("insert into table_name values(2);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_data("select col from table_name;", "[[\"1\"], [\"2\"]]", data_manager, catalog_manager);
        }

        #[test]
        fn limit_number_of_rows() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table table_name_2 (col integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_name_2 values(1);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_name_2 values(2);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_name_2 values(3);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_name_2 values(4);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_data(
                "select col from table_name_2 where limit = 3;",
                "[[\"1\"], [\"2\"], [\"3\"]]",
                data_manager,
                catalog_manager
            );
        }

        #[test]
        fn by_column_predicate() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table table_1 (col character(1));", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_1 values (\'a\');", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into table_1 values (\'b\');", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_data("select col from table_1 where col <> \'a\';", "[[\"b\"]]", data_manager, catalog_manager);
        }

        #[test]
        fn column_from_table_with_list_of_columns() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table tab1 (col_1 integer, col_2 integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into tab1 values(1, 2);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into tab1 values(3, 4);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_data("select col_1 from tab1;", "[[\"1\"], [\"3\"]]", data_manager, catalog_manager);
        }

        #[test]
        fn list_of_columns_from_table_with_many_columns() {
            let catalog_manager = LockBasedCatalogManager::default();
            let data_manager = LockBaseDataManager::default();

            drop(evaluate_query("create table tab1 (col_1 integer, col_2 integer);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into tab1 values(1, 2);", data_manager.clone(), catalog_manager.clone()));
            drop(evaluate_query("insert into tab1 values(3, 4);", data_manager.clone(), catalog_manager.clone()));

            assert_that_query_evaluation_return_data(
                "select col_1, col_2 from tab1;",
                "[[\"1\", \"2\"], [\"3\", \"4\"]]",
                data_manager,
                catalog_manager
            );
        }
    }
}
