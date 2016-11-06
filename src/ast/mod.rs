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
    Insert(InsertQuery<TypedColumn>),
    Select(SelectQuery<TypedColumn>),
    Delete
}

#[derive(PartialEq)]
pub enum TypedStatement {
    Create(CreateTableQuery),
    Insert(InsertQuery<TypedColumn>),
    Select(SelectQuery<TypedColumn>),
    Delelte
}

impl fmt::Debug for TypedStatement {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypedStatement::Create(ref create_table_query) => write!(f, "{:?}", create_table_query),
            TypedStatement::Insert(ref insert_query) => write!(f, "{:?}", insert_query),
            TypedStatement::Select(ref select_query) => write!(f, "{:?}", select_query),
            _ => panic!("unimplemented debug formatting")
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct TypedColumn {
    pub name: String,
    pub col_type: Type
}

impl TypedColumn {

    pub fn new<I: Into<String>>(name: I, col_type: Type) -> TypedColumn {
        TypedColumn {
            name: name.into(),
            col_type: col_type
        }
    }
}

impl fmt::Debug for TypedColumn {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<name: '{}', type: '{:?}'>", self.name, self.col_type)
    }
}

#[derive(PartialEq, Clone)]
pub enum RawStatement {
    Create(CreateTableQuery),
    Delete(DeleteQuery),
    Insert(InsertQuery<RawColumn>),
    Select(SelectQuery<RawColumn>)
}

impl fmt::Debug for RawStatement {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RawStatement::Create(ref query) => write!(f, "{:?}", query),
            RawStatement::Delete(ref query) => write!(f, "{:?}", query),
            RawStatement::Insert(ref query) => write!(f, "{:?}", query),
            RawStatement::Select(ref query) => write!(f, "{:?}", query),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct RawColumn {
    pub name: String
}

impl RawColumn {

    pub fn new<I: Into<String>>(name: I) -> RawColumn {
        RawColumn {
            name: name.into()
        }
    }
}

impl fmt::Debug for RawColumn {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<name: '{}'>", self.name)
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

pub fn debug_predicates(predicates: &Option<Condition>) -> String {
    match *predicates {
        Some(ref cond) => cond.to_string(),
        None => "no predicate".into()
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
