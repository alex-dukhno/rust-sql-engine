use super::ast::{TypedStatement, ValidatedStatement};
use super::catalog_manager::LockBasedCatalogManager;

pub fn validate(catalog_manager: LockBasedCatalogManager, statement: TypedStatement) -> Result<ValidatedStatement, String> {
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
            Ok(ValidatedStatement::Insert(query))
        },
        s => panic!("validation procedure for the {:?} statement has not been implemented yet", s)
    }
}