use std::sync::Mutex;
use std::collections::HashMap;

use super::parser::ast::Type;

pub trait CatalogManager {
    fn create() -> Self;

    fn add_table(&self, table: Table);

    fn contains_table(&self, table_name: &str) -> bool;

    fn add_column_to(&self, table_name: &str, column: Column);

    fn contains_column_in(&self, table_name: &str, column_name: &str) -> bool;

    fn match_type(&self, table_name: &str, column_index: usize, column_type: Type) -> bool;

    fn get_column_index(&self, table_name: &str, column_name: &str) -> Option<usize>;
}

pub struct LockBasedCatalogManager {
    tables: Mutex<HashMap<String, Table>>
}

impl CatalogManager for LockBasedCatalogManager {
    fn create() -> LockBasedCatalogManager {
        LockBasedCatalogManager {
            tables: Mutex::new(HashMap::default())
        }
    }

    fn add_table(&self, table: Table) {
        let mut guard = self.tables.lock().unwrap();
        (*guard).insert(table.name.clone(), table);
        drop(guard);
    }

    fn contains_table(&self, table_name: &str) -> bool {
        let guard = self.tables.lock().unwrap();
        let r = (*guard).keys().any(|name| name == table_name);
        drop(guard);
        r
    }

    fn add_column_to(&self, table_name: &str, column: Column) {
        let mut guard = self.tables.lock().unwrap();
        if let Some(table) = (*guard).get_mut(table_name) {
            (*table).columns.push(column);
        }
    }

    fn contains_column_in(&self, table_name: &str, column_name: &str) -> bool {
        let mut guard = self.tables.lock().unwrap();
        if let Some(table) = (*guard).get_mut(table_name) {
            (*table).columns.iter().any(|col| col.name == column_name)
        } else {
            false
        }
    }

    fn match_type(&self, table_name: &str, column_index: usize, column_type: Type) -> bool {
        let mut guard = self.tables.lock().unwrap();
        if let Some(table) = (*guard).get_mut(table_name) {
            match (*table).columns.get(column_index) {
                Some(col) => {
                    col.column_type == column_type
                },
                None => false
            }
        } else {
            false
        }
    }

    fn get_column_index(&self, table_name: &str, column_name: &str) -> Option<usize> {
        let guard = self.tables.lock().unwrap();
        let r = (*guard).get(table_name).and_then(|t| t.columns.iter().position(|c| c.name == column_name));
        drop(guard);
        r
    }
}

pub struct Table {
    name: String,
    columns: Vec<Column>
}

impl Table {
    pub fn new<I: Into<String>>(name: I) -> Table {
        Table { name: name.into(), columns: Vec::default() }
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    column_type: Type
}

impl Column {
    pub fn new<I: Into<String>>(name: I, columnt_type: Type) -> Column {
        Column { name: name.into(), column_type: columnt_type }
    }
}
