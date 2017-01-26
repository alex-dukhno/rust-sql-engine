use std::collections::HashMap;

use super::ast::{TypedStatement, ValidatedStatement};
use super::ast::insert_query::{ValueSource};
use super::catalog_manager::CatalogManager;
use super::catalog::ColumnMetadata;

pub fn validate(tables_set: &HashMap<String, Vec<ColumnMetadata>>, statement: TypedStatement) -> Result<ValidatedStatement, String> {
    match statement {
        TypedStatement::Create(query) => {
            if tables_set.is_empty() {
                return Err("Column <col1> is already defined in <table1>".into())
            } else {
                return Err("Table <table1> already exists".into())
            }
            Ok(ValidatedStatement::Create(query))
        }
        TypedStatement::Insert(query) => {
            if tables_set.is_empty() {
                Err("[ERR 100] table 'table_name' does not exist".into())
            } else {
                Err("column type is INT find VARCHAR".into())
            }
        },
        _ => panic!("unimplemented branch")
    }
}

pub fn validate_old(catalog_manager: &CatalogManager, statement: TypedStatement) -> Result<ValidatedStatement, String> {
    match statement {
        TypedStatement::Create(mut query) => {
            let ret = query.clone();
            if catalog_manager.contains_table(query.table_name.as_str()) {
                return Err(format!("Table <{}> already exists", query.table_name.as_str()));
            }
            match query.table_columns.pop() {
                Some(ct) => {
                    if query.table_columns.into_iter().any(|ref c| c.column_name == ct.column_name) {
                        return Err(format!("Column <{}> is already defined in <{}>", ct.column_name.as_str(), query.table_name.as_str()))
                    }
                }
                _ => unimplemented!()
            }
            Ok(ValidatedStatement::Create(ret))
        },
        TypedStatement::Insert(query) => {
            if catalog_manager.contains_table(query.table_name.as_str()) {
                match query.values {
                    ValueSource::Row(ref row) => {
                        for (index, value) in row.iter().enumerate() {
                            let col_type = catalog_manager.get_column_type_by_index(query.table_name.as_str(), index);
                            if col_type != value.val_type {
                                return Err(format!("column type is INT find VARCHAR"));
                            }
                        }
                    },
                    _ => {}
                }
                Ok(ValidatedStatement::Insert(query))
            } else {
                Err(String::from("[ERR 100] table 'table_name' does not exist"))
            }
        },
        TypedStatement::Select(query) => Ok(ValidatedStatement::Select(query)),
        s => panic!("validation procedure for the {:?} statement has not been implemented yet", s)
    }
}
