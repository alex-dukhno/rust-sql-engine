use super::parser::ast::{Statement, Type, CreateTableQuery, InsertQuery, SelectQuery, Value, Condition, CondArg};
use super::parser::ast;
use super::catalog_manager::{CatalogManager, LockBasedCatalogManager, Table, Column};
use super::data_manager::{DataManager, LockBaseDataManager};

pub struct QueryExecuter {
    catalog_manager: LockBasedCatalogManager,
    data_manager: LockBaseDataManager
}

impl Default for QueryExecuter {
    fn default() -> Self {
        QueryExecuter {
            catalog_manager: CatalogManager::create(),
            data_manager: DataManager::create()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Message(String),
    Data(Vec<Vec<String>>)
}

impl QueryExecuter {
    pub fn execute(&self, query: Statement) -> ExecutionResult {
        match query {
            Statement::Create(query) => self.create_table(query),
            Statement::Insert(query) => self.insert_into(query),
            Statement::Select(query) => self.select_data(query),
            _ => unimplemented!(),
        }
    }

    fn create_table(&self, create_query: CreateTableQuery) -> ExecutionResult {
        let CreateTableQuery { table_name, columns } = create_query;
        self.catalog_manager.add_table(Table::new(table_name.as_str()));
        for column in columns.into_iter() {
            let ast::table::Column { column_name, column_type } = column;
            self.catalog_manager.add_column_to(table_name.as_str(), Column::new(column_name, column_type))
        }
        ExecutionResult::Message(format!("'{}' was created", table_name.as_str()))
    }

    fn insert_into(&self, insert: InsertQuery) -> ExecutionResult {
        let InsertQuery { table_name, columns, values } = insert;
        if self.catalog_manager.contains_table(table_name.as_str()) {
            let mut data = Vec::with_capacity(values.len());
            for (index, datum) in values.into_iter().enumerate() {
                match datum {
                    Value::NumConst(n) => if self.catalog_manager.match_type(table_name.as_str(), index, Type::Varchar) {
                        return ExecutionResult::Message("column type is VARCHAR find INT".to_owned());
                    } else {
                        data.push(n);
                    },
                    Value::StrConst(_) => if self.catalog_manager.match_type(table_name.as_str(), index, Type::Int) {
                        return ExecutionResult::Message("column type is INT find VARCHAR".to_owned());
                    },
                }
            }
            self.data_manager.save_to(table_name.as_str(), data);
            ExecutionResult::Message("row was inserted".to_owned())
        } else {
            ExecutionResult::Message(format!("[ERR 100] table '{}' does not exist", table_name.as_str()))
        }
    }

    fn select_data(&self, query: SelectQuery) -> ExecutionResult {
        let SelectQuery { table_name, columns, condition } = query;
        match condition {
            Some(Condition::Eq(CondArg::Limit, CondArg::NumConst(n))) => {
                let limit = match n.parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => panic!(e),
                };
                ExecutionResult::Data(self.data_manager.get_range(table_name.as_str(), 0, limit))
            },
            Some(_) => unimplemented!(),
            None => ExecutionResult::Data(self.data_manager.get_range_till_end(table_name.as_str(), 0))
        }
    }
}
