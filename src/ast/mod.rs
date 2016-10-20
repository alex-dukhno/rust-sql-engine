pub mod create_table;
pub mod delete_query;
pub mod insert_query;
pub mod select_query;

use self::create_table::CreateTableQuery;
use self::delete_query::DeleteQuery;
use self::insert_query::InsertQuery;
use self::select_query::{SelectQuery, TypedSelectQuery};

#[derive(Debug, PartialEq)]
pub enum ValidatedStatement {
    Create(CreateTableQuery),
    Insert(InsertQuery),
    Select(SelectQuery),
    Delete
}

#[derive(Debug, PartialEq)]
pub enum TypedStatement {
    Create(CreateTableQuery),
    Insert(InsertQuery),
    Select(TypedSelectQuery),
    Delelte
}

#[derive(Debug, PartialEq, Clone)]
pub enum RawStatement {
    Create(CreateTableQuery),
    Delete(DeleteQuery),
    Insert(InsertQuery),
    Select(SelectQuery)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Integer,
    Character(Option<u8>),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum CondType {
    Eq,
    NotEq
}

#[derive(Debug, PartialEq, Clone)]
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
