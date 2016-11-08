use std::fmt;

use super::super::lexer::Token;
use super::select_query::SelectQuery;
use super::Type;

#[derive(PartialEq, Clone)]
pub struct InsertQuery<T: fmt::Debug> {
    pub table_name: String,
    pub columns: Vec<T>,
    pub values: ValueSource<T>
}

impl <T: fmt::Debug> InsertQuery<T> {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<T>, values: ValueSource<T>) -> InsertQuery<T> {
        InsertQuery {
            table_name: table_name.into(),
            columns: columns,
            values: values
        }
    }
}

impl <T: fmt::Debug> fmt::Debug for InsertQuery<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "statement: 'insert', table name: '{}', columns: {:?}, values: {:?}", self.table_name, self.columns, self.values)
    }
}

#[derive(PartialEq, Clone)]
pub enum ValueSource<T: fmt::Debug> {
    Row(Vec<Value>),
    SubQuery(SelectQuery<T>)
}

impl <T: fmt::Debug> fmt::Debug for ValueSource<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValueSource::Row(ref values) => write!(f, "{:?}", values),
            ValueSource::SubQuery(ref subquery) => write!(f, "<sub{:?}>", subquery)
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Value {
    pub val: String,
    pub val_type: Type
}

impl Value {

    pub fn new<I: Into<String>>(val: I, val_type: Type) -> Value {
        Value {
            val: val.into(),
            val_type: val_type
        }
    }
}

impl fmt::Debug for Value {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<value: {}, type: {:?}>", self.val, self.val_type)
    }
}
