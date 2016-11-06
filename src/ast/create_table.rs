use std::fmt;

use super::Type;

#[derive(PartialEq, Clone)]
pub struct CreateTableQuery {
    pub table_name: String,
    pub table_columns: Vec<ColumnTable>
}

impl CreateTableQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<ColumnTable>) -> CreateTableQuery {
        CreateTableQuery {
            table_name: table_name.into(),
            table_columns: columns
        }
    }
}

impl fmt::Debug for CreateTableQuery {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "statement: 'create table', table name: '{}', columns: {:?}", self.table_name, self.table_columns)
    }
}

#[derive(PartialEq, Clone)]
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

impl fmt::Debug for ColumnTable {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let default = match self.default_value {
            Some(ref v) => v.as_str(),
            None => "NULL"
        };
        let primary = if self.is_primary_key { "Yes" } else { "No" };
        let nullable = if self.nullable { "Yes" } else { "No" };
        let foreign = match self.foreign_key {
            Some((ref table, ref column)) => format!("{}->{}", table.as_str(), column.as_str()),
            _ => "No".into()
        };
        write!(f, "<name: '{}', type: '{:?}', primary key: {}, foreign key: {}, nullable: {}, default value: {}>", self.column_name, self.column_type, primary, foreign, nullable, default)
    }
}
