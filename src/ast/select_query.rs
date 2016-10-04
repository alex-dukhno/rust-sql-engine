use super::Condition;

#[derive(Debug, PartialEq, Clone)]
pub struct SelectQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub condition: Option<Condition>
}

impl SelectQuery {
    pub fn new<I: Into<String>>(table_name: I, columns: Vec<I>, condition: Option<Condition>) -> SelectQuery {
        SelectQuery {
            table_name: table_name.into(),
            columns: columns.into_iter().map(|c| c.into()).collect::<Vec<String>>(),
            condition: condition
        }
    }
}
