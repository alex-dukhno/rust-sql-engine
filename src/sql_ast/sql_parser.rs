use std::string::String;

pub struct SQLQueryParser {
    name: String,
}

impl SQLQueryParser {

    pub fn new(name: String) -> SQLQueryParser {
        SQLQueryParser { name: name }
    }
}
