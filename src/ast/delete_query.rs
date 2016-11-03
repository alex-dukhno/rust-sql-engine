use super::Condition;

#[derive(Debug, PartialEq, Clone)]
pub struct DeleteQuery {
    pub from: String,
    pub predicates: Option<Condition>
}

impl DeleteQuery {
    pub fn new<I: Into<String>>(table: I, condition: Option<Condition>) -> DeleteQuery {
        DeleteQuery {
            from: table.into(),
            predicates: condition
        }
    }
}
