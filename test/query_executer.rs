#[cfg(test)]
mod data_definition_language {
    #[cfg(test)]
    mod create_table {
        use expectest::prelude::be_equal_to;

        use sql::lexer::{Tokenizer, IntoTokenizer};
        use sql::parser::{QueryParser, IntoQueryParser};
        use sql::query_executer::{QueryExecuter, ExecutionResult};
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::query_typer::QueryTyper;

        #[test]
        fn single_column() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let statement = String::from("create table table_name (col integer);").into_tokenizer().tokenize().into_parser().parse();
            expect!(executer.execute(checker.type_inferring(statement)))
                .to(be_equal_to(ExecutionResult::Message("'table_name' was created".to_owned())));
        }

        #[test]
        fn with_list_of_columns() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let statement = String::from("create table table_name (col1 integer, col2 integer, col3 integer);").into_tokenizer().tokenize().into_parser().parse();
            expect!(executer.execute(checker.type_inferring(statement)))
                .to(be_equal_to(ExecutionResult::Message("'table_name' was created".to_owned())));
        }
    }
}

#[cfg(test)]
mod data_manipulation_language {
    #[cfg(test)]
    mod inserts {
        use expectest::prelude::be_equal_to;

        use sql::lexer::{Tokenizer, IntoTokenizer};
        use sql::parser::{QueryParser, IntoQueryParser};
        use sql::catalog_manager::LockBasedCatalogManager;
        use sql::query_executer::{QueryExecuter, ExecutionResult};
        use sql::query_typer::QueryTyper;

        #[test]
        fn row_in_created_table() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let create_statement = String::from("create table table_name (col integer);").into_tokenizer().tokenize().into_parser().parse();

            drop(executer.execute(checker.type_inferring(create_statement)));

            let insert_statement = String::from("insert into table_name values(1);").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(insert_statement)))
                .to(be_equal_to(ExecutionResult::Message("row was inserted".to_owned())));
        }

        #[test]
        fn row_in_table_with_many_columns() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let create_statement = String::from("create table table_name (col1 integer, col2 integer);").into_tokenizer().tokenize().into_parser().parse();

            drop(executer.execute(checker.type_inferring(create_statement)));

            let insert_statement = String::from("insert into table_name values(1, 2);").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(insert_statement)))
                .to(be_equal_to(ExecutionResult::Message("row was inserted".to_owned())));
        }

        #[test]
        #[ignore]
        fn does_not_insert_into_table_that_does_not_exist() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let statement = String::from("insert into table_name values(1);").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(statement)))
                .to(be_equal_to(ExecutionResult::Message("[ERR 100] table 'table_name' does not exist".to_owned())));
        }

        #[test]
        #[ignore]
        fn does_not_insert_when_column_type_does_not_match() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let create_statement = String::from("create table table_name (col integer);").into_tokenizer().tokenize().into_parser().parse();

            drop(executer.execute(checker.type_inferring(create_statement)));

            let insert_statement = String::from("insert into table_name values('string');").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(insert_statement)))
                .to(be_equal_to(ExecutionResult::Message("column type is INT find VARCHAR".to_owned())));
        }

        #[test]
        #[ignore]
        fn into_table_with_select() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let create_statement = String::from("create table table_name (col1 integer, col2 integer);").into_tokenizer().tokenize().into_parser().parse();

            drop(executer.execute(checker.type_inferring(create_statement)));

            let insert_1 = String::from("insert into table_name values(1, 2);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_1)));
            let insert_2 = String::from("insert into table_name values(3, 4);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_2)));
            let insert_3 = String::from("insert into table_name values(5, 6);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_3)));

            let select_statement = String::from("insert into table_name (col1, col2) select col1, col2 from table_name;").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(select_statement)))
                .to(be_equal_to(ExecutionResult::Message("3 rows were inserted".to_owned())));
        }
    }

    #[cfg(test)]
    mod selects {
        use expectest::prelude::be_equal_to;

        use sql::lexer::{Tokenizer, IntoTokenizer};
        use sql::parser::{QueryParser, IntoQueryParser};
        use sql::query_executer::{QueryExecuter, ExecutionResult};
        use sql::query_typer::QueryTyper;
        use sql::catalog_manager::LockBasedCatalogManager;

        #[test]
        fn from_table() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let create_statement = String::from("create table table_name (col integer);").into_tokenizer().tokenize().into_parser().parse();

            drop(executer.execute(checker.type_inferring(create_statement)));

            let insert_1 = String::from("insert into table_name values(1);").into_tokenizer().tokenize().into_parser().parse();

            drop(executer.execute(checker.type_inferring(insert_1)));

            let select_1 = String::from("select col from table_name;").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(select_1)))
                .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()]])));

            let insert_2 = String::from("insert into table_name values(2);").into_tokenizer().tokenize().into_parser().parse();

            drop(executer.execute(checker.type_inferring(insert_2)));

            let select_2 = String::from("select col from table_name;").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(select_2)))
                .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["2".to_owned()]])));
        }

        #[test]
        fn limit_number_of_rows() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let create_statement = String::from("create table table_name_2 (col integer);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(create_statement)));
            let insert_1 = String::from("insert into table_name_2 values(1);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_1)));
            let insert_2 = String::from("insert into table_name_2 values(2);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_2)));
            let insert_3 = String::from("insert into table_name_2 values(3);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_3)));
            let insert_4 = String::from("insert into table_name_2 values(4);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_4)));

            let select_statement = String::from("select col from table_name_2 where limit = 3;").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(select_statement)))
                .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["2".to_owned()], vec!["3".to_owned()]])));
        }

        #[test]
        fn by_column_predicate() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let create_statement = String::from("create table table_1 (col character(1));").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(create_statement)));

            let insert_1 = String::from("insert into table_1 values (\'a\');").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_1)));

            let insert_2 = String::from("insert into table_1 values (\'b\');").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_2)));

            let select_statement = String::from("select col from table_1 where col <> \'a\';").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(select_statement)))
                .to(be_equal_to(ExecutionResult::Data(vec![vec!["b".to_owned()]])));
        }

        #[test]
        fn column_from_table_with_list_of_columns() {
            let catalog_manager = LockBasedCatalogManager::default();

            let executer = QueryExecuter::new(catalog_manager.clone());
            let checker = QueryTyper::new(catalog_manager.clone());

            let create_statement = String::from("create table tab1 (col_1 integer, co_2 integer);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(create_statement)));

            let insert_1 = String::from("insert into tab1 values(1, 2);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_1)));

            let insert_2 = String::from("insert into tab1 values(3, 4);").into_tokenizer().tokenize().into_parser().parse();
            drop(executer.execute(checker.type_inferring(insert_2)));

            let select_statement = String::from("select col_1 from tab1;").into_tokenizer().tokenize().into_parser().parse();

            expect!(executer.execute(checker.type_inferring(select_statement)))
                .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["3".to_owned()]])));
        }
    }
}
