use std::fmt;

use super::select_query::SelectQuery;

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
