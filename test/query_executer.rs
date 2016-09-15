//#[cfg(test)]
//mod data_definition_language {
//    #[cfg(test)]
//    mod create_table {
//        use expectest::prelude::be_equal_to;
//
//        use sql::lexer::{Tokenizer, IntoTokenizer};
//        use sql::parser::{QueryParser, IntoQueryParser};
//        use sql::query_executer::{QueryExecuter, ExecutionResult};
//
//        #[test]
//        fn single_column() {
//            let executer = QueryExecuter::default();
//            expect!(executer.execute(String::from("create table table_name (col int);").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Message("'table_name' was created".to_owned())));
//        }
//
//        #[test]
//        fn with_list_of_columns() {
//            let executer = QueryExecuter::default();
//            expect!(executer.execute(String::from("create table table_name (col1 int, col2 int, col3 int);").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Message("'table_name' was created".to_owned())));
//        }
//    }
//}
//
//#[cfg(test)]
//mod data_manipulation_language {
//    #[cfg(test)]
//    mod inserts {
//        use expectest::prelude::be_equal_to;
//
//        use sql::lexer::{Tokenizer, IntoTokenizer};
//        use sql::parser::{QueryParser, IntoQueryParser};
//        use sql::query_executer::{QueryExecuter, ExecutionResult};
//
//        #[test]
//        fn row_in_created_table() {
//            let executer = QueryExecuter::default();
//            drop(executer.execute(String::from("create table table_name (col int);").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("insert into table_name values(1);").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Message("row was inserted".to_owned())));
//        }
//
//        #[test]
//        fn row_in_table_with_many_columns() {
//            let executer = QueryExecuter::default();
//            drop(executer.execute(String::from("create table table_name (col1 int, col2 int);").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("insert into table_name values(1, 2);").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Message("row was inserted".to_owned())));
//        }
//
//        #[test]
//        fn does_not_insert_into_table_that_does_not_exist() {
//            let executer = QueryExecuter::default();
//            expect!(executer.execute(String::from("insert into table_name values(1);").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Message("[ERR 100] table 'table_name' does not exist".to_owned())));
//        }
//
//        #[test]
//        fn does_not_insert_when_column_type_does_not_match() {
//            let executer = QueryExecuter::default();
//            drop(executer.execute(String::from("create table table_name (col int);").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("insert into table_name values('string');").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Message("column type is INT find VARCHAR".to_owned())));
//        }
//
//        #[test]
//        fn into_table_with_select() {
//            let executer = QueryExecuter::default();
//            drop(executer.execute(String::from("create table table_name (col1 int, col2 int);").into_tokenizer().tokenize().into_parser().parse()));
//
//            drop(executer.execute(String::from("insert into table_name values(1, 2);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_name values(3, 4);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_name values(5, 6);").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("insert into table_name (col1, col2) select col1, col2 from table_name;").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Message("3 rows were inserted".to_owned())));
//        }
//    }
//
//    #[cfg(test)]
//    mod selects {
//        use expectest::prelude::be_equal_to;
//
//        use sql::lexer::{Tokenizer, IntoTokenizer};
//        use sql::parser::{QueryParser, IntoQueryParser};
//        use sql::query_executer::{QueryExecuter, ExecutionResult};
//
//        #[test]
//        fn from_table() {
//            let executer = QueryExecuter::default();
//
//            drop(executer.execute(String::from("create table table_name (col int);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_name values(1);").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("select col from table_name;").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()]])));
//
//            drop(executer.execute(String::from("insert into table_name values(2);").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("select col from table_name;").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["2".to_owned()]])));
//        }
//
//        #[test]
//        fn limit_number_of_rows() {
//            let executer = QueryExecuter::default();
//
//            drop(executer.execute(String::from("create table table_name_2 (col int);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_name_2 values(1);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_name_2 values(2);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_name_2 values(3);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_name_2 values(4);").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("select col from table_name_2 where limit = 3;").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["2".to_owned()], vec!["3".to_owned()]])));
//        }
//
//        #[test]
//        fn by_column_predicate() {
//            let executer = QueryExecuter::default();
//
//            drop(executer.execute(String::from("create table table_1 (col varchar(1));").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_1 values (\'a\');").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into table_1 values (\'b\');").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("select col from table_1 where col <> \'a\';").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Data(vec![vec!["b".to_owned()]])));
//        }
//
//        #[test]
//        fn column_from_table_with_list_of_columns() {
//            let executer = QueryExecuter::default();
//
//            drop(executer.execute(String::from("create table tab1 (col_1 int, co_2 int);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into tab1 values(1, 2);").into_tokenizer().tokenize().into_parser().parse()));
//            drop(executer.execute(String::from("insert into tab1 values(3, 4);").into_tokenizer().tokenize().into_parser().parse()));
//
//            expect!(executer.execute(String::from("select col_1 from tab1;").into_tokenizer().tokenize().into_parser().parse()))
//                .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["3".to_owned()]])));
//        }
//    }
//}
