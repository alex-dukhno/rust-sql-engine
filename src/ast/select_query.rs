use std::fmt;

use super::{Condition, Type};

#[derive(PartialEq, Clone)]
pub struct SelectQuery<T> {
    pub table_name: String,
    pub columns: Vec<T>,
    pub predicates: Option<Condition>
}

impl SelectQuery<String> {
    pub fn new_raw<I: Into<String>>(table_name: I, columns: Vec<String>, predicates: Option<Condition>) -> SelectQuery<String> {
        SelectQuery {
            table_name: table_name.into(),
            columns: columns,
            predicates: predicates
        }
    }
}

impl SelectQuery<(String, Type)> {
    pub fn new_typed<I: Into<String>>(table_name: I, columns: Vec<(String, Type)>, predicates: Option<Condition>) -> SelectQuery<(String, Type)> {
        SelectQuery {
            table_name: table_name.into(),
            columns: columns,
            predicates: predicates
        }
    }
}

impl fmt::Debug for SelectQuery<String> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn debug_predicates(predicates: &Option<Condition>) -> String {
            match predicates {
                &Some(ref cond) => cond.to_string(),
                &None => "no predicate".into()
            }
        }

        fn debug_raw_columns(columns: &Vec<String>) -> String {
            String::from("[") + columns.iter().map(|c| format!("<name: '{}'>", c)).collect::<Vec<String>>().join(", ").as_str() + "]"
        }

        write!(f, "statement: 'select', tables: [<name: '{}'>], columns: {}, where: {}", self.table_name, debug_raw_columns(&self.columns), debug_predicates(&self.predicates))
    }
}

impl fmt::Debug for SelectQuery<(String, Type)> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn debug_predicates(predicates: &Option<Condition>) -> String {
            match predicates {
                &Some(ref cond) => cond.to_string(),
                &None => "no predicate".into()
            }
        }

        fn debug_raw_columns(columns: &Vec<(String, Type)>) -> String {
            String::from("[") + columns.iter().map(|&(ref c, _)| format!("<name: '{}'>", c)).collect::<Vec<String>>().join(", ").as_str() + "]"
        }

        write!(f, "statement: 'select', tables: [<name: '{}'>], columns: {}, where: {}", self.table_name, debug_raw_columns(&self.columns), debug_predicates(&self.predicates))
    }

}
