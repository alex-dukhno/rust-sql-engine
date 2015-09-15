use sql::ast::SubKeyWord;
use sql::ast::KeyWord;

#[test]
fn test_inser_key_word() {
    let into = SubKeyWord::Into { table_name: "tab1".to_string(), columns_names: vec!["col1".to_string()] };
    let values = SubKeyWord::Values { columns_values: vec!["'1'".to_string()] };
    KeyWord::Insert { into: into, values: values };
}

#[test]
fn test_into_sub_key_word() {
    SubKeyWord::Into { table_name: "tab1".to_string(), columns_names: vec!["col1".to_string()] };
}

#[test]
fn test_values_sub_key_word() {
    SubKeyWord::Values { columns_values: vec!["'1'".to_string()] };
}
