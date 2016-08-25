pub use expectest::prelude::{be_ok, be_err};

pub use sql::lexer::Tokenizer;
pub use sql::parser::Parser;
pub use sql::query_executer::QueryExecuter;

describe! create_table_queries {

    describe! queries_execution {

        before_each {
            let mut executer = QueryExecuter::default();
        }

        it "creates single column table" {
            expect!(executer.execute("create table table_name (col int);".to_owned().tokenize().unwrap().parse().unwrap()))
                .to(be_ok().value("'table_name' was created".to_owned()));
        }

        it "creates a table with list of columns" {
            expect!(executer.execute("create table table_name (col1 int, col2 int, col3 int);".to_owned().tokenize().unwrap().parse().unwrap()))
                .to(be_ok().value("'table_name' was created".to_owned()));
        }

        it "inserts row in created table" {
            executer.execute("create table table_name (col int);".to_owned().tokenize().unwrap().parse().unwrap());

            expect!(executer.execute("insert into table_name values(1);".to_owned().tokenize().unwrap().parse().unwrap()))
                .to(be_ok().value("row was inserted".to_owned()));
        }

        it "inserts row in table with many columns" {
            executer.execute("create table table_name (col1 int, col2 int);".to_owned().tokenize().unwrap().parse().unwrap());

            expect!(executer.execute("insert into table_name values(1, 2);".to_owned().tokenize().unwrap().parse().unwrap()))
                .to(be_ok().value("row was inserted".to_owned()));
        }
    }
}
