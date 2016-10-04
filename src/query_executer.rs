use super::ast::{ValidatedStatement, Type, Condition, CondType, CondArg};
use super::ast::create_table::{CreateTableQuery, ColumnTable};
use super::ast::insert_query::{InsertQuery, ValueSource, Value};
use super::ast::select_query::SelectQuery;
use super::catalog_manager::LockBasedCatalogManager;
use super::data_manager::LockBaseDataManager;

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Message(String),
    Data(Vec<Vec<String>>)
}

pub fn execute(catalog_manager: LockBasedCatalogManager, data_manager: LockBaseDataManager, query: ValidatedStatement) -> Result<ExecutionResult, String> {
    match query {
        ValidatedStatement::Create(query) => create_table(catalog_manager, data_manager, query),
        ValidatedStatement::Insert(query) => insert_into(catalog_manager, data_manager, query),
        ValidatedStatement::Select(query) => select_data(catalog_manager, data_manager, query),
        _ => unimplemented!(),
    }
}

fn create_table(catalog_manager: LockBasedCatalogManager, data_manager: LockBaseDataManager, create_query: CreateTableQuery) -> Result<ExecutionResult, String> {
    let CreateTableQuery { table_name, columns } = create_query;
    catalog_manager.add_table(table_name.as_str());
    for column in columns.into_iter() {
        let ColumnTable { column_name, column_type, is_primary_key, foreign_key, nullable, default_value  } = column;
        catalog_manager.add_column_to(table_name.as_str(), (column_name, column_type, None))
    }
    Ok(ExecutionResult::Message(format!("'{}' was created", table_name.as_str())))
}

fn insert_into(catalog_manager: LockBasedCatalogManager, data_manager: LockBaseDataManager, insert: InsertQuery) -> Result<ExecutionResult, String> {
    let InsertQuery { table_name, columns, values } = insert;
    if catalog_manager.contains_table(table_name.as_str()) {
        match values {
            ValueSource::Row(row) => {
                let mut data = Vec::with_capacity(row.len());
                for (index, datum) in row.into_iter().enumerate() {
                    match datum {
                        Value::NumConst(n) => if catalog_manager.match_type(table_name.as_str(), index, Type::Character(Option::from(0))) {
                            return Ok(ExecutionResult::Message("column type is VARCHAR find INT".to_owned()));
                        } else {
                            data.push(n);
                        },
                        Value::StrConst(s) => if catalog_manager.match_type(table_name.as_str(), index, Type::Integer) {
                            return Ok(ExecutionResult::Message("column type is INT find VARCHAR".to_owned()));
                        } else {
                            data.push(s);
                        },
                    }
                }
                data_manager.save_to(table_name.as_str(), data);
                Ok(ExecutionResult::Message("row was inserted".to_owned()))
            },
            ValueSource::SubQuery(query) => {
                if let Ok(ExecutionResult::Data(query_result)) = select_data(catalog_manager, data_manager.clone(), query) {
                    let row_num = query_result.len();
                    for row in query_result {
                        data_manager.save_to(table_name.as_str(), row);
                    }
                    Ok(ExecutionResult::Message(format!("{} rows were inserted", row_num)))
                } else {
                    panic!("unexpected subquery result")
                }
            },
        }
    } else {
        Ok(ExecutionResult::Message(format!("[ERR 100] table '{}' does not exist", table_name.as_str())))
    }
}

fn select_data(catalog_manager: LockBasedCatalogManager, data_manager: LockBaseDataManager, query: SelectQuery) -> Result<ExecutionResult, String> {
    let SelectQuery { table_name, columns, condition } = query;
    match condition {
        Some(Condition { left, right, cond_type }) => {
            match (left, right, cond_type) {
                (CondArg::Limit, CondArg::NumConst(n), CondType::Eq) => {
                    let limit = match n.parse::<usize>() {
                        Ok(v) => v,
                        Err(e) => panic!(e),
                    };
                    Ok(ExecutionResult::Data(data_manager.get_range(table_name.as_str(), 0, limit)))
                },
                (CondArg::ColumnName(name), CondArg::StringConstant(value), CondType::NotEq) => {
                    if let Some(index) = catalog_manager.get_column_index(table_name.as_str(), &name) {
                        Ok(ExecutionResult::Data(data_manager.get_not_equal(table_name.as_str(), index, &value)))
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        None => {
            if let Some(index) = catalog_manager.get_column_index(table_name.as_str(), &columns[0]) {
                Ok(ExecutionResult::Data(data_manager.get_range_till_end_for_column(table_name.as_str(), index)))
            } else {
                Ok(ExecutionResult::Data(data_manager.get_range_till_end(table_name.as_str(), 0)))
            }
        }
    }
}
