use std::string::String;

pub struct SQLQueryParser {
    name: String,
}

impl SQLQueryParser {

    pub fn new(name: String) -> SQLQueryParser {
        SQLQueryParser { name: name }
    }
}

pub struct SQLNode {
    value: String,
}

impl SQLNode {

    pub fn new(value: String) -> SQLNode {
        SQLNode { value: value }
    }
}
