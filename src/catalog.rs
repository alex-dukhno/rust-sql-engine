use super::ast::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnMetadata {
    pub name: String,
    pub col_type: Type,
    pub default_val: Option<String>
}
