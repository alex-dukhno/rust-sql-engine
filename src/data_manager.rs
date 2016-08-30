use std::collections::HashMap;
use std::sync::Mutex;

pub struct DataManager {
    data: Mutex<HashMap<String, Vec<Vec<String>>>>
}

impl Default for DataManager {
    fn default() -> DataManager {
        DataManager {
            data: Mutex::new(HashMap::default())
        }
    }
}

impl DataManager {
    pub fn save_to<I, D>(&self, table_name: I, data: D) -> Result<(), ()>
        where I: Into<String>,
              D: IntoIterator<Item = I> {
        let mut guard = self.data.lock().unwrap();
        (*guard).entry(table_name.into())
            .or_insert_with(Vec::default)
            .push(
                data.into_iter().map(Into::into).collect::<Vec<String>>()
            );
        drop(guard);
        Ok(())
    }

    pub fn get_row_from(&self, table_name: &str, row_id: usize) -> Vec<String> {
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
        println!("result = {:?}", result);
        result
    }

    pub fn get_range(&self, table_name: &str, start_from: usize, number_of_rows: usize) -> Vec<Vec<String>> {
        let guard = self.data.lock().unwrap();
        let result = match (*guard).get(table_name) {
            None => vec![],
            Some(table_data) =>
                table_data.into_iter()
                    .skip(start_from)
                    .take(number_of_rows)
                    .map(|v| v.iter().cloned().collect::<Vec<String>>())
                    .collect::<Vec<Vec<String>>>(),
        };
        drop(guard);
        result
    }

    pub fn get_range_till_end(&self, table_name: &str, start_from: usize) -> Vec<Vec<String>> {
        let guard = self.data.lock().unwrap();
        let result = match (*guard).get(table_name) {
            None => vec![],
            Some(table_data) =>
                table_data.into_iter()
                    .skip(start_from)
                    .map(|v| v.into_iter().cloned().collect::<Vec<String>>())
                    .collect::<Vec<Vec<String>>>(),
        };
        drop(guard);
        result
    }
}