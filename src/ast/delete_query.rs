use std::fmt;

use super::{Condition, debug_predicates};

#[derive(PartialEq, Clone)]
pub struct DeleteQuery {
    pub from: String,
    pub predicates: Option<Condition>
}

impl DeleteQuery {
    pub fn new<I: Into<String>>(table: I, condition: Option<Condition>) -> DeleteQuery {
        DeleteQuery {
            from: table.into(),
            predicates: condition
        }
    }
}

impl fmt::Debug for DeleteQuery {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "statement: 'delete', table name: '{}', where: {}", self.from,  debug_predicates(&self.predicates))
    }
}
