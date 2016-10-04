use super::select_query::SelectQuery;

#[derive(Debug, PartialEq, Clone)]
pub struct InsertQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: ValueSource
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueSource {
    Row(Vec<Value>),
    SubQuery(SelectQuery)
}

impl InsertQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<I>, values: ValueSource) -> InsertQuery {
        InsertQuery {
            table_name: table_name.into(),
            columns: columns.into_iter().map(|c| c.into()).collect::<Vec<String>>(),
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
