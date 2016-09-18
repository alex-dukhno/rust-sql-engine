use super::ast::{Statement, ColumnTable};
use super::catalog_manager::LockBasedCatalogManager;

pub struct QueryValidator {
    catalog_manager: LockBasedCatalogManager
}

impl QueryValidator {
    pub fn new(catalog_manager: LockBasedCatalogManager) -> QueryValidator {
        QueryValidator {
            catalog_manager: catalog_manager
        }
    }

    pub fn validate(&self, statement: Statement) -> Result<Statement, String> {
        let copy = statement.clone();
        match statement {
            Statement::Create(mut query) => {
                if self.catalog_manager.contains_table(query.table_name.as_str()) {
                    return Err(format!("Table <{}> already exists", query.table_name.as_str()));
                }
                match query.columns.pop() {
                    Some(ColumnTable {column_name, column_type, constraints}) => {
                        if query.columns.into_iter().any(|ref c| c.column_name == column_name) {
                            return Err(format!("Column <{}> is already defined in <{}>", column_name.as_str(), query.table_name.as_str()))
                        }
                    }
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
        Ok(copy)
    }
}