#[derive(Debug, PartialEq)]
pub enum Statement {
    Create(CreateTableQuery),
    Delete(DeleteQuery),
    Insert(InsertQuery),
    Select(SelectQuery)
}

#[derive(Debug, PartialEq)]
pub struct CreateTableQuery {
    pub table_name: String,
    pub columns: Vec<table::Column>
}

impl CreateTableQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<table::Column>) -> CreateTableQuery {
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
    pub values: Vec<ValueParameter>
}

impl InsertQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<I>, values: Vec<ValueParameter> ) -> InsertQuery {
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

#[derive(Debug, PartialEq)]
pub enum Type {
    Int,
    Varchar
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    PrimeryKey,
    ForeignKey(String),
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Eq(PredicateArgument, PredicateArgument)
}

#[derive(Debug, PartialEq)]
pub enum PredicateArgument {
    ColumnName(String),
    StringConstant(String),
    NumberConstant(String)
}

#[derive(Debug, PartialEq)]
pub enum ValueParameter {
    StringConst(String),
    NumberConst(String)
}

pub mod table {
    #[derive(Debug, PartialEq)]
    pub struct Column {
        pub column_name: String,
        pub column_type: super::Type
    }

    impl Column {
        pub fn new<I: Into<String>>(name: I, column_type: super::Type) -> Column {
            Column {
                column_name: name.into(),
                column_type: column_type
            }
        }
    }
}
