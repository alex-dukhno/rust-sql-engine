use super::ast::{ValidatedStatement, TypedColumn, Condition, CondType, CondArg};
use super::ast::create_table::CreateTableQuery;
use super::ast::insert_query::{InsertQuery, ValueSource, Value};
use super::ast::select_query::SelectQuery;
use super::catalog_manager::CatalogManager;
use super::data_manager::DataManager;

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Message(String),
    Data(Vec<Vec<String>>)
}

pub fn execute(catalog_manager: &CatalogManager, data_manager: &DataManager, query: ValidatedStatement) -> Result<ExecutionResult, String> {
    match query {
        ValidatedStatement::Create(query) => create_table(catalog_manager, query),
        ValidatedStatement::Insert(query) => insert_into(catalog_manager, data_manager, query),
        ValidatedStatement::Select(query) => select_data(catalog_manager, data_manager, query),
        _ => unimplemented!(),
    }
}

fn create_table(catalog_manager: &CatalogManager, create_query: CreateTableQuery) -> Result<ExecutionResult, String> {
    let CreateTableQuery { table_name, table_columns } = create_query;
    catalog_manager.add_table(table_name.as_str());
    for column in table_columns.into_iter() {
        catalog_manager.add_column_to(table_name.as_str(), (column.column_name, column.column_type, column.default_value))
    }
    Ok(ExecutionResult::Message(format!("'{}' was created", table_name.as_str())))
}

fn insert_into(catalog_manager: &CatalogManager, data_manager: &DataManager, insert: InsertQuery<TypedColumn>) -> Result<ExecutionResult, String> {
    match insert.values {
        ValueSource::Row(row) => {
            let data = row.into_iter().map(|v| v.val).collect::<Vec<String>>();
            data_manager.save_to(insert.table_name.as_str(), data);
            println!("data manager - {:?}", data_manager.get_row_from(insert.table_name.as_str(), 0));
            Ok(ExecutionResult::Message("row was inserted".to_owned()))
        },
        ValueSource::SubQuery(query) => {
            if let Ok(ExecutionResult::Data(query_result)) = select_data(catalog_manager, data_manager, query) {
                let row_num = query_result.len();
                for row in query_result {
                    data_manager.save_to(insert.table_name.as_str(), row);
                }
                Ok(ExecutionResult::Message(format!("{} rows were inserted", row_num)))
            } else {
                panic!("unexpected sub query result")
            }
        },
    }
}

fn select_data(catalog_manager: &CatalogManager, data_manager: &DataManager, query: SelectQuery<TypedColumn>) -> Result<ExecutionResult, String> {
    let SelectQuery { table_name, columns, predicates } = query;
    match predicates {
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
            if let Some(index) = catalog_manager.get_column_index(table_name.as_str(), &columns[0].name) {
                Ok(ExecutionResult::Data(data_manager.get_range_till_end_for_column(table_name.as_str(), index, columns.len())))
            } else {
                Ok(ExecutionResult::Data(data_manager.get_range_till_end(table_name.as_str(), 0)))
            }
        }
    }
}
