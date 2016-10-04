use super::Type;

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnTable {
    pub column_name: String,
    pub column_type: Type,
    pub is_primary_key: bool,
    pub foreign_key: Option<(String, String)>,
    pub nullable: bool,
    pub default_value: Option<String>
}

impl ColumnTable {
    pub fn new<I>(name: I, column_type: Type, is_primary_key: bool, foreign_key: Option<(String, String)>, nullable: bool, default_value: Option<I>) -> ColumnTable
        where I: Into<String> {
        ColumnTable {
            column_name: name.into(),
            column_type: column_type,
            is_primary_key: is_primary_key,
            foreign_key: foreign_key,
            nullable: nullable,
            default_value: default_value.map(Into::into)
        }
    }
}
