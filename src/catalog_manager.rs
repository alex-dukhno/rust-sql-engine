use std::sync::{Mutex, Arc};
use std::collections::HashMap;

use super::ast::Type;
use super::catalog::ColumnMetadata;

#[derive(Clone)]
pub struct CatalogManager {
    tables: Arc<Mutex<HashMap<String, Vec<ColumnMetadata>>>>
}

impl Default for CatalogManager {
    fn default() -> Self {
        CatalogManager {
            tables: Arc::new(Mutex::new(HashMap::default()))
        }
    }
}

impl CatalogManager {
    pub fn add_table<I: Into<String>>(&self, table_name: I) {
        let mut guard = self.tables.lock().unwrap();
        (*guard).entry(table_name.into()).or_insert_with(Vec::default);
        drop(guard);
    }

    pub fn contains_table(&self, table_name: &str) -> bool {
        let guard = self.tables.lock().unwrap();
        let r = (*guard).keys().any(|name| name == table_name);
        drop(guard);
        r
    }

    pub fn add_column_to<I: Into<String>>(&self, table_name: &str, column: (I, Type, Option<I>)) {
        let mut guard = self.tables.lock().unwrap();
        if let Some(table) = (*guard).get_mut(table_name) {
            (*table).push(ColumnMetadata { name: column.0.into(), col_type: column.1, default_val: column.2.and_then(|i| Some(i.into())) });
        }
        drop(guard);
    }

    pub fn contains_column_in(&self, table_name: &str, column_name: &str) -> bool {
        let mut guard = self.tables.lock().unwrap();
        if let Some(table) = (*guard).get_mut(table_name) {
            (*table).iter().any(|ref c| c.name == column_name)
        } else {
            false
        }
    }

    pub fn match_type(&self, table_name: &str, column_index: usize, column_type: Type) -> bool {
        let mut guard = self.tables.lock().unwrap();
        if let Some(table) = (*guard).get_mut(table_name) {
            match (*table).get(column_index) {
                Some(ref c) => c.col_type == column_type,
                None => false
            }
        } else {
            false
        }
    }

    pub fn get_column_index(&self, table_name: &str, column_name: &str) -> Option<usize> {
        let guard = self.tables.lock().unwrap();
        let r = (*guard).get(table_name).and_then(|v| v.iter().position(|ref c| c.name == column_name));
        drop(guard);
        r
    }

    pub fn get_table_columns(&self, table_name: &str) -> Vec<ColumnMetadata> {
        let guard = self.tables.lock().unwrap();
        let r = match (*guard).get(table_name) {
            Some(table) => table.iter().cloned().collect::<Vec<ColumnMetadata>>(),
            None => vec![] //panic!("table <{}> not found", table_name),
        };
        drop(guard);
        r
    }

    pub fn get_column_type(&self, table_name: &str, column_name: &str) -> Type {
        let guard = self.tables.lock().unwrap();
        match (*guard).get(table_name) {
            Some(table) => {
                for c in table {
                    if c.name == column_name {
                        return c.col_type
                    }
                }
                panic!("unimplemented if column with <{}> name does not exist in <{}> table", column_name, table_name);
            }
            None => panic!("unimplemented if table <{}> does not exist", table_name)
        }
        drop(guard);
    }

    pub fn get_column_type_by_index(&self, table_name: &str, index: usize) -> Type {
        let guard = self.tables.lock().unwrap();
        match (*guard).get(table_name) {
            Some(table) => {
                match table.into_iter().nth(index) {
                    Some(cm) => cm.col_type,
                    _ => panic!("unimplemented if column with <{}> index does not exist in <{}> table", index, table_name)
                }
            }
            _ => panic!("unimplemented if table <{}> does not exist", table_name)
        }
    }
}
