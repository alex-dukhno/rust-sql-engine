use super::parser::Node;

pub struct QueryExecuter;

impl QueryExecuter {

    pub fn new() -> QueryExecuter { QueryExecuter{ } }

    pub fn execute(&self, query: Node) -> Result<(), ()> {
        Ok(())
    }
}
