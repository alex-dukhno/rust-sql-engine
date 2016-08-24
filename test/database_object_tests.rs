pub use expectest::prelude::{be_ok, be_err};

pub use sql::parser::Node::{Table, TableColumn, Create, Insert, Values, Const};
pub use sql::parser::Type::{Int};
pub use sql::parser::Flag::{PrimeryKey, ForeignKey};
pub use sql::query_executer::QueryExecuter;

describe! database {

    before_each {
        let mut executer = QueryExecuter::default();
    }

    it "creates table with one column from AST" {
        let create_table = Create(Box::new(Table(table_name(), Some(vec![TableColumn(column_name(), Some(Int), None)]))));

        expect!(executer.execute(create_table)).to(be_ok().value(format!("'{}' was created", table_name())));
    }

    it "could not insert into table that does not exist" {
        let insert = Insert(Box::new(Table(table_name(), None)), Box::new(Values(vec![Const(ten_int())])));

        expect!(executer.execute(insert)).to(be_err().value(format!("[ERR 100] table '{}' does not exist", table_name())));
    }
}

pub fn table_name() -> String {
    "table_name".to_owned()
}

pub fn ten_int() -> String {
    "10".to_owned()
}

pub fn five_int() -> String {
    "5".to_owned()
}

pub fn column_name() -> String {
    "column_name".to_owned()
}
