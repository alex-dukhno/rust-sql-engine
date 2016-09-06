use std::collections::HashMap;
use std::sync::Mutex;

pub trait DataManager {
    fn create() -> Self;

    fn save_to<I, D>(&self, table_name: I, data: D)
        where I: Into<String>,
              D: IntoIterator<Item = String>;

    fn get_row_from(&self, table_name: &str, row_id: usize) -> Vec<String>;

    fn get_range(&self, table_name: &str, start_from: usize, number_of_rows: usize) -> Vec<Vec<String>>;

    fn get_range_till_end(&self, table_name: &str, start_from: usize) -> Vec<Vec<String>>;

    fn get_not_equal(&self, table_name: &str, column_index: usize, value: &String) -> Vec<Vec<String>>;
}

#[derive(Debug)]
pub struct LockBaseDataManager {
    data: Mutex<HashMap<String, Vec<Vec<String>>>>
}

impl DataManager for LockBaseDataManager {
    fn create() -> LockBaseDataManager {
        LockBaseDataManager {
            data: Mutex::new(HashMap::default())
        }
    }

    fn save_to<I, D>(&self, table_name: I, data: D)
        where I: Into<String>,
              D: IntoIterator<Item = String> {
        let mut guard = self.data.lock().unwrap();
        (*guard).entry(table_name.into())
            .or_insert_with(Vec::default)
            .push(
                data.into_iter().collect::<Vec<String>>()
            );
        drop(guard);
    }

    fn get_row_from(&self, table_name: &str, row_id: usize) -> Vec<String> {
        let guard = self.data.lock().unwrap();
        let result = match (*guard).get(table_name) {
            None => vec![],
            Some(table_data) => {
                match table_data.into_iter().nth(row_id) {
                    None => vec![],
                    Some(vec) => vec.iter().cloned().collect::<Vec<String>>()
                }
            },
        };
        drop(guard);
        result
    }

    fn get_range(&self, table_name: &str, start_from: usize, number_of_rows: usize) -> Vec<Vec<String>> {
        let guard = self.data.lock().unwrap();
        let result = match (*guard).get(table_name) {
            None => vec![],
            Some(table_data) =>
                table_data.into_iter()
                    .skip(start_from)
                    .take(number_of_rows)
                    .cloned()
                    .collect::<Vec<Vec<String>>>(),
        };
        drop(guard);
        result
    }

    fn get_range_till_end(&self, table_name: &str, start_from: usize) -> Vec<Vec<String>> {
        let guard = self.data.lock().unwrap();
        let result = match (*guard).get(table_name) {
            None => unimplemented!(),
            Some(table_data) =>
                table_data.into_iter()
                    .skip(start_from)
                    .cloned()
                    .collect::<Vec<Vec<String>>>(),
        };
        drop(guard);
        result
    }

    fn get_not_equal(&self, table_name: &str, column_index: usize, value: &String) -> Vec<Vec<String>> {
        let guard = self.data.lock().unwrap();
        let result = match (*guard).get(table_name) {
            None => unimplemented!(),
            Some(table_data) => {
                table_data.into_iter().filter(|v| v.get(column_index) != Some(value)).cloned().collect::<Vec<Vec<String>>>()
            },
        };
        drop(guard);
        result
    }
}