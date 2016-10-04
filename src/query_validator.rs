use super::ast::{TypedStatement, ValidatedStatement};
use super::ast::create_table::ColumnTable;
use super::catalog_manager::LockBasedCatalogManager;

pub fn validate(catalog_manager: LockBasedCatalogManager, statement: TypedStatement) -> Result<ValidatedStatement, String> {
    match statement {
        TypedStatement::Create(mut query) => {
            let ret = query.clone();
            if catalog_manager.contains_table(query.table_name.as_str()) {
                return Err(format!("Table <{}> already exists", query.table_name.as_str()));
            }
            match query.columns.pop() {
                Some(ColumnTable { column_name, column_type, is_primary_key, foreign_key, nullable, default_value }) => {
                    if query.columns.into_iter().any(|ref c| c.column_name == column_name) {
                        return Err(format!("Column <{}> is already defined in <{}>", column_name.as_str(), query.table_name.as_str()))
                    }
                }
                _ => unimplemented!()
            }
            Ok(ValidatedStatement::Create(ret))
        },
        TypedStatement::Insert(mut query) => {
            Ok(ValidatedStatement::Insert(query))
        },
        s => panic!("validation procedure for the {:?} statement has not been implemented yet", s)
    }
}