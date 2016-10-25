use super::{Condition, Type};

#[derive(Debug, PartialEq, Clone)]
pub struct SelectQuery<T> {
    pub table_name: String,
    pub columns: Vec<T>,
    pub condition: Option<Condition>
}

impl SelectQuery<String> {
    pub fn new_raw<I: Into<String>>(table_name: I, columns: Vec<String>, condition: Option<Condition>) -> SelectQuery<String> {
        SelectQuery {
            table_name: table_name.into(),
            columns: columns,
            condition: condition
        }
    }

    pub fn new_typed<I: Into<String>>(table_name: I, columns: Vec<(String, Type)>, condition: Option<Condition>) -> SelectQuery<(String, Type)> {
        SelectQuery {
            table_name: table_name.into(),
            columns: columns,
            condition: condition
        }
    }
}
