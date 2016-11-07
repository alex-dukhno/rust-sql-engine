use super::ast::{Type, TypedStatement, ValidatedStatement};
use super::ast::insert_query::{Value, ValueSource};
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
            if catalog_manager.contains_table(query.table_name.as_str()) {
                match query.values {
                    ValueSource::Row(ref row) => {
                        for (index, value) in row.iter().enumerate() {
                            match value {
                                &Value::NumConst(_) => {
                                    if catalog_manager.match_type(query.table_name.as_str(), index, Type::Character(Option::from(0))) {
                                        return Err(String::from("column type is VARCHAR find INT"));
                                    } else {
                                        continue;
                                    }
                                },
                                &Value::StrConst(_) => {
                                    if catalog_manager.match_type(query.table_name.as_str(), index, Type::Integer) {
                                        return Err(String::from("column type is INT find VARCHAR"));
                                    } else {
                                        continue;
                                    }
                                }
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
