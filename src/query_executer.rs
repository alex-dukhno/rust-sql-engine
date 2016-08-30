use super::parser::ast::{Node, Type};
use super::catalog_manager::{CatalogManager, LockBasedCatalogManager, Table, Column};
use super::data_manager::DataManager;

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
            Node::Create(table) => self.create_table(*table),
            Node::Insert(table, values) => self.insert_into(*table, *values),
            Node::Select(table, columns) => self.select_data(*table, columns),
            _ => Err("execute".to_owned()),
        }
    }

    fn create_table(&self, table: Node) -> Result<ExecutionResult, String> {
        match table {
            Node::Table(name, columns) => {
                self.catalog_manager.add_table(Table::new(name.as_str()));
                for column in columns.into_iter() {
                    if let Node::TableColumn(column_name, column_type, _) = column {
                        self.catalog_manager.add_column_to(name.as_str(), Column::new(column_name, column_type))
                    }
                }
                let s = name.clone();
                Ok(ExecutionResult::Message(format!("'{}' was created", s)))
            },
            _ => Err("not a table".to_owned()),
        }
    }

    fn insert_into(&self, table: Node, values: Node) -> Result<ExecutionResult, String> {
        match table {
            Node::Table(name, _) => {
                if self.catalog_manager.contains_table(name.as_str()) {
                    match values {
                        Node::Values(data) => {
                            let mut v = Vec::with_capacity(data.len());
                            for (index, datum) in data.into_iter().enumerate() {
                                match datum {
                                    Node::Numeric(n) => if self.catalog_manager.match_type(name.as_str(), index, Type::Varchar) {
                                        return Err("column type is VARCHAR find INT".to_owned());
                                    } else {
                                        v.push(n);
                                    },
                                    Node::CharSequence(_) => if self.catalog_manager.match_type(name.as_str(), index, Type::Int) {
                                        return Err("column type is INT find VARCHAR".to_owned());
                                    },
                                    _ => return Err("wrong node".to_owned()),
                                }
                            }
                            self.data_manager.save_to(name, v);
                        },
                        _ => return Err("not a values".to_owned()),
                    }
                    Ok(ExecutionResult::Message("row was inserted".to_owned()))
                } else {
                    Err(format!("[ERR 100] table '{}' does not exist", name))
                }
            },
            _ => Err("not a table".to_owned()),
        }
    }

    fn select_data(&self, table: Node, columns: Vec<Node>) -> Result<ExecutionResult, String> {
        let result = match table {
            Node::Table(table_name, _) => self.data_manager.get_range_till_end(table_name.as_str(), 0),
            _ => return Err("parsing error".to_owned()),
        };
        Ok(ExecutionResult::Data(result))
    }
}
