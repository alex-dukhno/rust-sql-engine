use super::parser::ast::Node::{self, Create, TableN, Insert, Values, TableColumn, NumberC, StringC};
use super::parser::ast::Type::{Int, Varchar};
use super::catalog_manager::{CatalogManager, LockBasedCatalogManager, Table, Column};

pub struct QueryExecuter {
    catalog_manager: LockBasedCatalogManager
}

impl Default for QueryExecuter {
    fn default() -> Self {
        QueryExecuter {
            catalog_manager: CatalogManager::create()
        }
    }
}

impl QueryExecuter {
    pub fn execute(&self, query: Node) -> Result<String, String> {
        match query {
            Create(table) => self.create_table(*table),
            Insert(table, values) => self.insert_into(*table, *values),
            _ => Err("execute".to_owned()),
        }
    }

    fn create_table(&self, table: Node) -> Result<String, String> {
        match table {
            TableN(name, columns) => {
                self.catalog_manager.add_table(Table::new(name.as_str()));
                for column in columns.into_iter() {
                    if let TableColumn(column_name, column_type, _) = column {
                        self.catalog_manager.add_column_to(name.as_str(), Column::new(column_name, column_type))
                    }
                }
                let s = name.clone();
                Ok(format!("'{}' was created", s))
            },
            _ => Err("not a table".to_owned()),
        }
    }

    fn insert_into(&self, table: Node, values: Node) -> Result<String, String> {
        match table {
            TableN(name, _) => {
                if self.catalog_manager.contains_table(name.as_str()) {
                    match values {
                        Values(data) => {
                            for (index, datum) in data.into_iter().enumerate() {
                                match datum {
                                    NumberC(_) => if self.catalog_manager.match_type(name.as_str(), index, Varchar) {
                                        return Err("column type is VARCHAR find INT".to_owned());
                                    },
                                    StringC(_) => if self.catalog_manager.match_type(name.as_str(), index, Int) {
                                        return Err("column type is INT find VARCHAR".to_owned());
                                    },
                                    _ => return Err("wrong node".to_owned()),
                                }
                            }
                        },
                        _ => return Err("not a values".to_owned()),
                    }
                    Ok("row was inserted".to_owned())
                } else {
                    Err(format!("[ERR 100] table '{}' does not exist", name))
                }
            },
            _ => Err("not a table".to_owned()),
        }
    }
}
