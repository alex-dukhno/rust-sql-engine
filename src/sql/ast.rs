use std::vec::Vec;
use std::option::Option;

pub enum KeyWord {
    Insert { into: SubKeyWord, values: SubKeyWord },
}

pub enum SubKeyWord {
    Into { table_name: String, columns_names: Option<Vec<String>> },
    Values { columns_values: Vec<String> },
}
