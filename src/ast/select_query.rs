use std::fmt;

use super::{Condition, debug_predicates};

#[derive(PartialEq, Clone)]
pub struct SelectQuery<T: fmt::Debug> {
    pub table_name: String,
    pub columns: Vec<T>,
    pub predicates: Option<Condition>
}

impl <T: fmt::Debug> SelectQuery<T> {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<T>, predicates: Option<Condition>) -> SelectQuery<T> {
        SelectQuery {
            table_name: table_name.into(),
            columns: columns,
            predicates: predicates
        }
    }
}

impl <T: fmt::Debug> fmt::Debug for SelectQuery<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "statement: 'select', tables: [<name: '{}'>], columns: {:?}, where: {}", self.table_name, self.columns, debug_predicates(&self.predicates))
    }
}
