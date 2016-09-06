pub use expectest::prelude::{be_ok, be_err, be_equal_to};

use sql::lexer::Tokenizer;
use sql::parser::Parser;
use sql::query_executer::{QueryExecuter, ExecutionResult};

#[test]
fn creates_single_column_table() {
    let executer = QueryExecuter::default();
    expect!(executer.execute(Tokenizer::from("create table table_name (col int);").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Message("'table_name' was created".to_owned())));
}

#[test]
fn creates_a_table_with_list_of_columns() {
    let executer = QueryExecuter::default();
    expect!(executer.execute(Tokenizer::from("create table table_name (col1 int, col2 int, col3 int);").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Message("'table_name' was created".to_owned())));
}

#[test]
fn inserts_row_in_created_table() {
    let executer = QueryExecuter::default();
    drop(executer.execute(Tokenizer::from("create table table_name (col int);").tokenize().parse()));

    expect!(executer.execute(Tokenizer::from("insert into table_name values(1);").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Message("row was inserted".to_owned())));
}

#[test]
fn inserts_row_in_table_with_many_columns() {
    let executer = QueryExecuter::default();
    drop(executer.execute(Tokenizer::from("create table table_name (col1 int, col2 int);").tokenize().parse()));

    expect!(executer.execute(Tokenizer::from("insert into table_name values(1, 2);").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Message("row was inserted".to_owned())));
}

#[test]
fn does_not_insert_into_table_that_does_not_exist() {
    let executer = QueryExecuter::default();
    expect!(executer.execute(Tokenizer::from("insert into table_name values(1);").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Message("[ERR 100] table 'table_name' does not exist".to_owned())));
}

#[test]
fn does_not_insert_when_column_type_does_not_match() {
    let executer = QueryExecuter::default();
    drop(executer.execute(Tokenizer::from("create table table_name (col int);").tokenize().parse()));

    expect!(executer.execute(Tokenizer::from("insert into table_name values('string');").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Message("column type is INT find VARCHAR".to_owned())));
}

#[test]
fn selects_inserted_data_from_table() {
    let executer = QueryExecuter::default();

    drop(executer.execute(Tokenizer::from("create table table_name (col int);").tokenize().parse()));
    drop(executer.execute(Tokenizer::from("insert into table_name values(1);").tokenize().parse()));

    expect!(executer.execute(Tokenizer::from("select col from table_name;").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()]])));

    drop(executer.execute(Tokenizer::from("insert into table_name values(2);").tokenize().parse()));

    expect!(executer.execute(Tokenizer::from("select col from table_name;").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["2".to_owned()]])));
}

#[test]
fn select_limit_number_of_rows() {
    let executer = QueryExecuter::default();

    drop(executer.execute(Tokenizer::from("create table table_name_2 (col int);").tokenize().parse()));
    drop(executer.execute(Tokenizer::from("insert into table_name_2 values(1);").tokenize().parse()));
    drop(executer.execute(Tokenizer::from("insert into table_name_2 values(2);").tokenize().parse()));
    drop(executer.execute(Tokenizer::from("insert into table_name_2 values(3);").tokenize().parse()));
    drop(executer.execute(Tokenizer::from("insert into table_name_2 values(4);").tokenize().parse()));

    expect!(executer.execute(Tokenizer::from("select col from table_name_2 where limit = 3;").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Data(vec![vec!["1".to_owned()], vec!["2".to_owned()], vec!["3".to_owned()]])));
}

#[test]
fn select_by_column_predicate() {
    let executer = QueryExecuter::default();

    drop(executer.execute(Tokenizer::from("create table table_1 (col varchar(1));").tokenize().parse()));
    drop(executer.execute(Tokenizer::from("insert into table_1 values (\'a\');").tokenize().parse()));
    drop(executer.execute(Tokenizer::from("insert into table_1 values (\'b\');").tokenize().parse()));

    expect!(executer.execute(Tokenizer::from("select col from table_1 where col <> \'a\';").tokenize().parse()))
        .to(be_equal_to(ExecutionResult::Data(vec![vec!["b".to_owned()]])));
}
