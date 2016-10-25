use std::collections::HashSet;
use std::hash::Hash;

use super::select_query::SelectQuery;
use super::Type;

#[derive(Debug, PartialEq, Clone)]
pub struct InsertQuery<T: Eq + Hash> {
    pub table_name: String,
    pub columns: HashSet<T>,
    pub values: ValueSource
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueSource {
    Row(Vec<Value>),
    SubQuery(SelectQuery)
}

impl InsertQuery<String> {
    pub fn new_raw<I: Into<String>>(table_name: I, columns: HashSet<String>, values: ValueSource) -> InsertQuery<String> {
        InsertQuery {
            table_name: table_name.into(),
            columns: columns,
            values: values
        }
    }
}

impl InsertQuery<(String, Type)> {
    pub fn new_typed<I: Into<String>>(table_name: I, columns: HashSet<(String, Type)>, values: ValueSource) -> InsertQuery<(String, Type)> {
        InsertQuery {
            table_name: table_name.into(),
            columns: columns,
            values: values
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    StrConst(String),
    NumConst(String)
}

impl Value {
    pub fn str<I: Into<String>>(v: I) -> Value {
        Value::StrConst(v.into())
    }

    pub fn num<I: Into<String>>(v: I) -> Value {
        Value::NumConst(v.into())
    }
}
