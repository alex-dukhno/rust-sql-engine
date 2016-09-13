use super::parser::ast::{Statement, Type, CreateTableQuery, InsertQuery, SelectQuery, Value, Condition, CondType, CondArg, ColumnTable, ValueSource};
use super::catalog_manager::{LockBasedCatalogManager, Table, Column};
use super::data_manager::LockBaseDataManager;

pub struct QueryExecuter {
    catalog_manager: LockBasedCatalogManager,
    data_manager: LockBaseDataManager
}

impl Default for QueryExecuter {
    fn default() -> Self {
        QueryExecuter {
            catalog_manager: LockBasedCatalogManager::default(),
            data_manager: LockBaseDataManager::default()
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
            let ColumnTable { column_name, column_type } = column;
            self.catalog_manager.add_column_to(table_name.as_str(), Column::new(column_name, column_type))
        }
        ExecutionResult::Message(format!("'{}' was created", table_name.as_str()))
    }

    fn insert_into(&self, insert: InsertQuery) -> ExecutionResult {
        let InsertQuery { table_name, columns, values } = insert;
        if self.catalog_manager.contains_table(table_name.as_str()) {
            match values {
                ValueSource::Row(row) => {
                    let mut data = Vec::with_capacity(row.len());
                    for (index, datum) in row.into_iter().enumerate() {
                        match datum {
                            Value::NumConst(n) => if self.catalog_manager.match_type(table_name.as_str(), index, Type::VarChar(0)) {
                                return ExecutionResult::Message("column type is VARCHAR find INT".to_owned());
                            } else {
                                data.push(n);
                            },
                            Value::StrConst(s) => if self.catalog_manager.match_type(table_name.as_str(), index, Type::Int) {
                                return ExecutionResult::Message("column type is INT find VARCHAR".to_owned());
                            } else {
                                data.push(s);
                            },
                        }
                    }
                    self.data_manager.save_to(table_name.as_str(), data);
                    ExecutionResult::Message("row was inserted".to_owned())
                },
                ValueSource::SubQuery(query) => {
                    if let ExecutionResult::Data(query_result) = self.select_data(query) {
                        let row_num = query_result.len();
                        for row in query_result {
                            self.data_manager.save_to(table_name.as_str(), row);
                        }
                        ExecutionResult::Message(format!("{} rows were inserted", row_num))
                    } else {
                        panic!("unexpected subquery result")
                    }
                },
            }
        } else {
            ExecutionResult::Message(format!("[ERR 100] table '{}' does not exist", table_name.as_str()))
        }
    }

    fn select_data(&self, query: SelectQuery) -> ExecutionResult {
        let SelectQuery { table_name, columns, condition } = query;
        match condition {
            Some(Condition { left, right, cond_type }) => {
                match (left, right, cond_type) {
                    (CondArg::Limit, CondArg::NumConst(n), CondType::Eq) => {
                        let limit = match n.parse::<usize>() {
                            Ok(v) => v,
                            Err(e) => panic!(e),
                        };
                        ExecutionResult::Data(self.data_manager.get_range(table_name.as_str(), 0, limit))
                    },
                    (CondArg::ColumnName(name), CondArg::StringConstant(value), CondType::NotEq) => {
                        if let Some(index) = self.catalog_manager.get_column_index(table_name.as_str(), &name) {
                            ExecutionResult::Data(self.data_manager.get_not_equal(table_name.as_str(), index, &value))
                        } else {
                            unimplemented!()
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            None => {
                if let Some(index) = self.catalog_manager.get_column_index(table_name.as_str(), &columns[0]) {
                    ExecutionResult::Data(self.data_manager.get_range_till_end_for_column(table_name.as_str(), index))
                } else {
                    ExecutionResult::Data(self.data_manager.get_range_till_end(table_name.as_str(), 0))
                }
            }
        }
    }
}
