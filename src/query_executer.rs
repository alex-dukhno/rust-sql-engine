use super::parser::ast::Node::{self, Create, TableN, Insert, Values, TableColumn, NumberC, StringC, Select};
use super::parser::ast::Type::{Int, Varchar};
use super::catalog_manager::{CatalogManager, LockBasedCatalogManager, Table, Column};
use super::data_manager::DataManager;
use self::ExecutionResult::{Message, Data};

pub struct QueryExecuter {
    catalog_manager: LockBasedCatalogManager,
    data_manager: DataManager
}

impl Default for QueryExecuter {
    fn default() -> Self {
        QueryExecuter {
            catalog_manager: CatalogManager::create(),
            data_manager: DataManager::default()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Message(String),
    Data(Vec<Vec<String>>)
}

impl QueryExecuter {
    pub fn execute(&self, query: Node) -> Result<ExecutionResult, String> {
        match query {
            Create(table) => self.create_table(*table),
            Insert(table, values) => self.insert_into(*table, *values),
            Select(table, columns) => self.select_data(*table, columns),
            _ => Err("execute".to_owned()),
        }
    }

    fn create_table(&self, table: Node) -> Result<ExecutionResult, String> {
        match table {
            TableN(name, columns) => {
                self.catalog_manager.add_table(Table::new(name.as_str()));
                for column in columns.into_iter() {
                    if let TableColumn(column_name, column_type, _) = column {
                        self.catalog_manager.add_column_to(name.as_str(), Column::new(column_name, column_type))
                    }
                }
                let s = name.clone();
                Ok(Message(format!("'{}' was created", s)))
            },
            _ => Err("not a table".to_owned()),
        }
    }

    fn insert_into(&self, table: Node, values: Node) -> Result<ExecutionResult, String> {
        match table {
            TableN(name, _) => {
                if self.catalog_manager.contains_table(name.as_str()) {
                    match values {
                        Values(data) => {
                            let mut v = Vec::with_capacity(data.len());
                            for (index, datum) in data.into_iter().enumerate() {
                                match datum {
                                    NumberC(n) => if self.catalog_manager.match_type(name.as_str(), index, Varchar) {
                                        return Err("column type is VARCHAR find INT".to_owned());
                                    } else {
                                        v.push(n);
                                    },
                                    StringC(_) => if self.catalog_manager.match_type(name.as_str(), index, Int) {
                                        return Err("column type is INT find VARCHAR".to_owned());
                                    },
                                    _ => return Err("wrong node".to_owned()),
                                }
                            }
                            self.data_manager.save_to(name, v);
                        },
                        _ => return Err("not a values".to_owned()),
                    }
                    Ok(Message("row was inserted".to_owned()))
                } else {
                    Err(format!("[ERR 100] table '{}' does not exist", name))
                }
            },
            _ => Err("not a table".to_owned()),
        }
    }

    fn select_data(&self, table: Node, columns: Vec<Node>) -> Result<ExecutionResult, String> {
        let mut result = vec![];
        println!("columns = {:?}", columns);
        match table {
            TableN(table_name, _) => {
                for (index, _) in columns.into_iter().enumerate() {
                    println!("index = {:?}", index);
                    result.push(self.data_manager.get_row_from(table_name.as_str(), index));
                }
            }
            _ => return Err("parsing error".to_owned()),
        }
        Ok(Data(result))
    }
}
