use super::ast::Statement;
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
            Statement::Create(query) => {
                if self.catalog_manager.contains_table(query.table_name.as_str()) {
                    return Err(format!("Table <{}> already exists", query.table_name.as_str()));
                }
            },
            _ => unimplemented!()
        }
        Ok(copy)
    }
}