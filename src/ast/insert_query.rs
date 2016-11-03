use std::fmt;

use super::select_query::SelectQuery;
use super::Type;

#[derive(PartialEq, Clone)]
pub struct InsertQuery<T> {
    pub table_name: String,
    pub columns: Vec<T>,
    pub values: ValueSource<T>
}

impl InsertQuery<String> {
    pub fn new_raw<I: Into<String>>(table_name: I, columns: Vec<String>, values: ValueSource<String>) -> InsertQuery<String> {
        InsertQuery {
            table_name: table_name.into(),
            columns: columns,
            values: values
        }
    }
}

impl fmt::Debug for InsertQuery<String> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn debug_raw_columns(columns: &Vec<String>) -> String {
            String::from("[") + columns.iter().map(|c| format!("<name: '{}'>", c)).collect::<Vec<String>>().join(", ").as_str() + "]"
        }

        write!(f, "statement: 'insert', table name: '{}', columns: {}, values: {:?}", self.table_name, debug_raw_columns(&self.columns), self.values)
    }
}

impl InsertQuery<(String, Type)> {
    pub fn new_typed<I: Into<String>>(table_name: I, columns: Vec<(String, Type)>, values: ValueSource<(String, Type)>) -> InsertQuery<(String, Type)> {
        InsertQuery {
            table_name: table_name.into(),
            columns: columns,
            values: values
        }
    }
}

impl fmt::Debug for InsertQuery<(String, Type)> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn debug_raw_columns(columns: &Vec<(String, Type)>) -> String {
            String::from("[") + columns.iter().map(|&(ref c, _)| format!("<name: '{}'>", c)).collect::<Vec<String>>().join(", ").as_str() + "]"
        }

        write!(f, "statement: 'insert', table name: '{}', columns: {}, values: {:?}", self.table_name, debug_raw_columns(&self.columns), self.values)
    }
}

#[derive(PartialEq, Clone)]
pub enum ValueSource<T> {
    Row(Vec<Value>),
    SubQuery(SelectQuery<T>)
}

impl fmt::Debug for ValueSource<String> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValueSource::Row(ref values) => write!(f, "{:?}", values),
            ValueSource::SubQuery(ref subquery) => write!(f, "<sub{:?}>", subquery)
        }
    }
}

impl fmt::Debug for ValueSource<(String, Type)> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValueSource::Row(ref values) => write!(f, "{:?}", values),
            ValueSource::SubQuery(ref subquery) => write!(f, "subquery: <{:?}>", subquery)
        }
    }
}

#[derive(PartialEq, Clone)]
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

impl fmt::Debug for Value {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::NumConst(ref number) => write!(f, "Numeric({})", number),
            Value::StrConst(ref string) => write!(f, "String({})", string)
        }
    }
}
