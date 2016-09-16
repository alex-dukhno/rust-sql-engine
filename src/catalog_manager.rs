use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

use super::ast::Type;

type Column = (String, Type, Option<String>);

#[derive(Clone)]
pub struct LockBasedCatalogManager {
    tables: Arc<Mutex<HashMap<String, Vec<Column>>>>
}

impl Default for LockBasedCatalogManager {
    fn default() -> Self {
        LockBasedCatalogManager {
            tables: Arc::new(Mutex::new(HashMap::default()))
        }
    }
}

impl LockBasedCatalogManager {
    pub fn add_table<I: Into<String>>(&self, table_name: I) {
        let mut guard = self.tables.lock().unwrap();
        (*guard).entry(table_name.into()).or_insert(vec![]);
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
            (*table).push((column.0.into(), column.1, column.2.and_then(|i| Some(i.into()))));
        }
        drop(guard);
    }

    pub fn contains_column_in(&self, table_name: &str, column_name: &str) -> bool {
        let mut guard = self.tables.lock().unwrap();
        if let Some(table) = (*guard).get_mut(table_name) {
            (*table).iter().any(|&(ref col_name, _, _)| col_name == column_name)
        } else {
            false
        }
    }

    pub fn match_type(&self, table_name: &str, column_index: usize, column_type: Type) -> bool {
        let mut guard = self.tables.lock().unwrap();
        if let Some(table) = (*guard).get_mut(table_name) {
            match (*table).get(column_index) {
                Some(&(_, col_type, _)) => col_type == column_type,
                None => false
            }
        } else {
            false
        }
    }

    pub fn get_column_index(&self, table_name: &str, column_name: &str) -> Option<usize> {
        let guard = self.tables.lock().unwrap();
        let r = (*guard).get(table_name).and_then(|v| v.iter().position(|&(ref col_name, _, _)| col_name == column_name));
        drop(guard);
        r
    }

    pub fn get_table_columns(&self, table_name: &str) -> Vec<(String, (Option<String>, Type))> {
        let guard = self.tables.lock().unwrap();
        let r = match (*guard).get(table_name) {
            Some(table) => table.iter().map(|&(ref col_name, col_type, ref default)| (col_name.clone(), (default.clone(), col_type))).collect::<Vec<(String, (Option<String>, Type))>>(),
            None => vec![],
        };
        drop(guard);
        r
    }
}
