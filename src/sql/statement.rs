use std::string::String;
use std::collections::HashMap;
use std::vec::Vec;

pub struct InsertStatement {
    table_name: String,
    bindings: HashMap<String, String>,
}

impl InsertStatement {

    pub fn new(table_name: String, columns: &Vec<String>, values: &Vec<String>) -> InsertStatement {
        if columns.len() != values.len() {
            panic!("columns and values have to be the same length");
        }
        let mut bindings = HashMap::new();
        for i in 0..columns.len() {
            bindings.insert(columns[i].clone(), values[i].clone());
        }
        InsertStatement { table_name: table_name, bindings: bindings }
    }
}
