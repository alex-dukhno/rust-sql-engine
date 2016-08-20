pub use expectest::prelude::be_ok;

pub use sql::parser::Node::{self, Table, Column};
pub use sql::parser::Type;
pub use sql::query_executer::QueryExecuter;

describe! database {

    it "creates table from AST" {
        let table = Node::Create(Box::new(Table("table_name".to_owned(), Some(vec![Column("col".to_owned(), Some(Type::Int))]))));

        let executer = QueryExecuter::new();

        expect!(executer.execute(table)).to(be_ok());
    }
}
