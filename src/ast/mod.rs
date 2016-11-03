pub mod create_table;
pub mod delete_query;
pub mod insert_query;
pub mod select_query;

use std::fmt;

use self::create_table::CreateTableQuery;
use self::delete_query::DeleteQuery;
use self::insert_query::InsertQuery;
use self::select_query::SelectQuery;

#[derive(Debug, PartialEq)]
pub enum ValidatedStatement {
    Create(CreateTableQuery),
    Insert(InsertQuery<(String, Type)>),
    Select(SelectQuery<(String, Type)>),
    Delete
}

#[derive(Debug, PartialEq)]
pub enum TypedStatement {
    Create(CreateTableQuery),
    Insert(InsertQuery<(String, Type)>),
    Select(SelectQuery<(String, Type)>),
    Delelte
}

#[derive(PartialEq, Clone)]
pub enum RawStatement {
    Create(CreateTableQuery),
    Delete(DeleteQuery),
    Insert(InsertQuery<String>),
    Select(SelectQuery<String>)
}

impl fmt::Debug for RawStatement {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn debug_predicates(predicates: &Option<Condition>) -> String {
            match predicates {
                &Some(ref cond) => cond.to_string(),
                &None => "no predicate".into()
            }
        }

        match *self {
            RawStatement::Create(ref query) => write!(f, "statement: 'create table', table name: '{}', columns: {:?}", query.table_name, query.table_columns),
            RawStatement::Delete(ref query) => write!(f, "statement: 'delete', table name: '{}', where: {}", query.from, debug_predicates(&query.predicates)),
            RawStatement::Insert(ref query) => write!(f, "{:?}", query),
            RawStatement::Select(ref query) => write!(f, "{:?}", query),
            // _ => panic!("unimlemented debug formatting")
        }
    }
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub enum Type {
    Integer,
    Character(Option<u8>),
}

impl fmt::Debug for Type {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Integer => write!(f, "integer"),
            Type::Character(Some(v)) => write!(f, "character size of {}", v),
            Type::Character(None) => write!(f, "character")
        }
    }
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

impl ToString for Condition {

    fn to_string(&self) -> String {
        match (&self.left, &self.cond_type, &self.right) {
            (&CondArg::ColumnName(ref name), &CondType::Eq, &CondArg::NumConst(ref c)) => format!("predicate <{} equals to {}>", name, c),
            (&CondArg::StringConstant(ref c), &CondType::Eq, &CondArg::ColumnName(ref name)) => format!("predicate <'{}' equals to {}>", c, name),
            (&CondArg::Limit, &CondType::Eq, &CondArg::NumConst(ref c)) => format!("predicate <limit equals to {}>", c),
            (&CondArg::ColumnName(ref name), &CondType::NotEq, &CondArg::StringConstant(ref c)) => format!("predicate <{} not equals to '{}'>", name, c),
            _ => "unimlemented condition formatting".into()
        }
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
