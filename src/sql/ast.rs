use std::vec::Vec;

pub enum KeyWord {
    Insert { into: SubKeyWord, values: SubKeyWord },
}

pub enum SubKeyWord {
    Into { table_name: String, columns_names: Vec<String> },
    Values { columns_values: Vec<String> },
}
