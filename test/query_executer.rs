use sql::query_executer::ExecutionResult;
use sql::catalog_manager::CatalogManager;
use sql::data_manager::DataManager;

use super::evaluate_query;

fn assert_that_query_evaluation_return_message(
        src_query: &str,
        expected_message: &str,
        data_manager: &DataManager,
        catalog_manager: &CatalogManager) {
    let execution_result = evaluate_query(src_query, data_manager, catalog_manager);

    match execution_result {
        Ok(ExecutionResult::Message(actual_message)) => assert_eq!(actual_message, expected_message),
        res => panic!("unexpected query evaluation result {:?}", res)
    }
}

fn assert_that_query_evaluation_return_data(
        src_query: &str,
        expected_data: &str,
        data_manager: &DataManager,
        catalog_manager: &CatalogManager) {
    let execution_result = evaluate_query(src_query, data_manager, catalog_manager);

    match execution_result {
        Ok(ExecutionResult::Data(data)) => assert_eq!(format!("{:?}", data), expected_data),
        res => panic!("unexpected query evaluation result {:?}", res)
    }
}

#[cfg(test)]
mod data_definition_language {
    #[cfg(test)]
    mod create_table {
        use sql::catalog_manager::CatalogManager;
        use sql::data_manager::DataManager;

        use super::super::super::evaluate_query;
        use super::super::assert_that_query_evaluation_return_message;

        #[test]
        #[ignore]
        fn single_column() {
            assert_that_query_evaluation_return_message(
                "create table table_name (col integer);",
                "'table_name' was created",
                &DataManager::default(),
                &CatalogManager::default()
            );
        }

        #[test]
        #[ignore]
        fn with_list_of_columns() {
            assert_that_query_evaluation_return_message(
                "create table table_name (col1 integer, col2 integer, col3 integer);",
                "'table_name' was created",
                &DataManager::default(),
                &CatalogManager::default()
            );
        }

        #[test]
        #[ignore]
        fn with_foreign_key() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table table1 (col1 integer primary key);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_message(
                "create table table2 (col2 integer primary key, col3 integer foreign key references table1 (col1));",
                "'table2' was created",
                &data_manager,
                &catalog_manager
            );
        }
    }
}

#[cfg(test)]
mod data_manipulation_language {
    #[cfg(test)]
    mod inserts {
        use sql::catalog_manager::CatalogManager;
        use sql::data_manager::DataManager;

        use super::super::super::evaluate_query;
        use super::super::assert_that_query_evaluation_return_message;
        use super::super::assert_that_query_evaluation_return_data;

        #[test]
        #[ignore]
        fn row_in_created_table() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table table_name (col integer);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_message(
                "insert into table_name values(1);",
                "row was inserted",
                &data_manager,
                &catalog_manager
            );
        }

        #[test]
        #[ignore]
        fn row_in_table_with_many_columns() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(
                evaluate_query(
                    "create table table_name (col1 integer, col2 integer);",
                    &data_manager,
                    &catalog_manager
                )
            );

            assert_that_query_evaluation_return_message(
                "insert into table_name values(1, 2);",
                "row was inserted",
                &data_manager,
                &catalog_manager
            );
        }

        #[test]
        #[ignore]
        fn into_table_with_select() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table table_name (col1 integer, col2 integer);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_name values(1, 2);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_name values(3, 4);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_name values(5, 6);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_message(
                "insert into table_name (col1, col2) select col1, col2 from table_name;",
                "3 rows were inserted",
                 &data_manager,
                 &catalog_manager
            );
        }

        #[test]
        #[ignore]
        fn column_with_default_value() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table table1 (col1 integer, col2 integer default 1);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_message(
                "insert into table1 values (1);",
                "row was inserted",
                &data_manager,
                &catalog_manager
            );

            assert_that_query_evaluation_return_data(
                "select col1, col2 from table1;",
                "[[\"1\", \"1\"]]",
                &data_manager,
                &catalog_manager
            );
        }
    }

    #[cfg(test)]
    mod selects {
        use sql::catalog_manager::CatalogManager;
        use sql::data_manager::DataManager;

        use super::super::super::evaluate_query;
        use super::super::assert_that_query_evaluation_return_data;

        #[test]
        #[ignore]
        fn from_table() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table table_name (col integer);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_name values(1);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_data(
                "select col from table_name;",
                "[[\"1\"]]",
                &data_manager,
                &catalog_manager
            );

            drop(evaluate_query("insert into table_name values(2);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_data(
                "select col from table_name;",
                "[[\"1\"], [\"2\"]]",
                &data_manager,
                &catalog_manager
            );
        }

        #[test]
        #[ignore]
        fn limit_number_of_rows() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table table_name_2 (col integer);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_name_2 values(1);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_name_2 values(2);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_name_2 values(3);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_name_2 values(4);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_data(
                "select col from table_name_2 where limit = 3;",
                "[[\"1\"], [\"2\"], [\"3\"]]",
                &data_manager,
                &catalog_manager
            );
        }

        #[test]
        #[ignore]
        fn by_column_predicate() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table table_1 (col character(1));", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_1 values (\'a\');", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into table_1 values (\'b\');", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_data(
                "select col from table_1 where col <> \'a\';",
                "[[\"b\"]]",
                &data_manager,
                &catalog_manager
            );
        }

        #[test]
        #[ignore]
        fn column_from_table_with_list_of_columns() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table tab1 (col_1 integer, col_2 integer);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into tab1 values(1, 2);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into tab1 values(3, 4);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_data(
                "select col_1 from tab1;",
                "[[\"1\"], [\"3\"]]",
                &data_manager,
                &catalog_manager
            );
        }

        #[test]
        #[ignore]
        fn list_of_columns_from_table_with_many_columns() {
            let catalog_manager = CatalogManager::default();
            let data_manager = DataManager::default();

            drop(evaluate_query("create table tab1 (col_1 integer, col_2 integer);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into tab1 values(1, 2);", &data_manager, &catalog_manager));
            drop(evaluate_query("insert into tab1 values(3, 4);", &data_manager, &catalog_manager));

            assert_that_query_evaluation_return_data(
                "select col_1, col_2 from tab1;",
                "[[\"1\", \"2\"], [\"3\", \"4\"]]",
                &data_manager,
                &catalog_manager
            );
        }
    }
}
