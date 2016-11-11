use super::ast::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnMetadata {
    pub name: String,
    pub col_type: Type,
    pub default_val: Option<String>
}

impl ColumnMetadata {

    pub fn new<I: Into<String>>(name: I, col_type: Type, default_val: Option<I>) -> ColumnMetadata {
        ColumnMetadata {
            name: name.into(),
            col_type: col_type,
            default_val: default_val.and_then(|d| Some(d.into()))
        }
    }
}
