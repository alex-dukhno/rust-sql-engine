use super::super::catalog_manager::{LockBasedCatalogManager, Column, Table};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Create(CreateTableQuery),
    Delete(DeleteQuery),
    Insert(InsertQuery),
    Select(SelectQuery)
}

impl Statement {

    pub fn populate(mut self, catalog_manager: &LockBasedCatalogManager) -> Statement {
        match self {
            Statement::Insert(query) => {
                let columns = catalog_manager.get_table_columns(query.table_name.as_str());
                let new = InsertQuery::new(query.table_name, columns, query.values);
                Statement::Insert(new)
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CreateTableQuery {
    pub table_name: String,
    pub columns: Vec<ColumnTable>
}

impl CreateTableQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<ColumnTable>) -> CreateTableQuery {
        CreateTableQuery {
            table_name: table_name.into(),
            columns: columns
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeleteQuery {
    from: String,
    predicates: Option<Condition>
}

impl DeleteQuery {
    pub fn new<I: Into<String>>(table: I, condition: Option<Condition>) -> DeleteQuery {
        DeleteQuery {
            from: table.into(),
            predicates: condition
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InsertQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: ValueSource
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct SelectQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub condition: Option<Condition>
}

impl SelectQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<I>, condition: Option<Condition>) -> SelectQuery {
        SelectQuery {
            table_name: table_name.into(),
            columns: columns.into_iter().map(|c| c.into()).collect::<Vec<String>>(),
            condition: condition
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
    VarChar(u8),
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    PrimeryKey,
    ForeignKey(String),
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub left: CondArg,
    pub right: CondArg,
    pub cond_type: CondType
}

impl Condition {
    pub fn new(left: CondArg, right: CondArg, cond_type: CondType) -> Condition {
        Condition {
            left: left,
            right: right,
            cond_type: cond_type
        }
    }

    pub fn equals(left: CondArg, right: CondArg) -> Condition {
        Condition::new(left, right, CondType::Eq)
    }

    pub fn not_equals(left: CondArg, right: CondArg) -> Condition {
        Condition::new(left, right, CondType::NotEq)
    }
}

#[derive(Debug, PartialEq)]
pub enum CondType {
    Eq,
    NotEq
}

#[derive(Debug, PartialEq)]
pub enum CondArg {
    ColumnName(String),
    StringConstant(String),
    NumConst(String),
    Limit
}

impl CondArg {
    pub fn column<I: Into<String>>(column_name: I) -> CondArg {
        CondArg::ColumnName(column_name.into())
    }

    pub fn str<I: Into<String>>(const_str: I) -> CondArg {
        CondArg::StringConstant(const_str.into())
    }

    pub fn num<I: Into<String>>(const_num: I) -> CondArg {
        CondArg::NumConst(const_num.into())
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct ColumnTable {
    pub column_name: String,
    pub column_type: Type
}

impl ColumnTable {
    pub fn new<I: Into<String>>(name: I, column_type: Type) -> ColumnTable {
        ColumnTable {
            column_name: name.into(),
            column_type: column_type
        }
    }
}
