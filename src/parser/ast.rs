#[derive(Debug, PartialEq)]
pub enum Node {
    Delete(Box<Node>, Box<Node>),
    From(String),
    Where(Option<Condition>),
    Id(String),
    NumberC(String),
    StringC(String),

    Insert(Box<Node>, Box<Node>),
    TableN(String, Vec<Node>),
    Values(Vec<Node>),
    Column(String),

    Select(Box<Node>, Vec<Node>),

    Create(Box<Node>),
    TableColumn(String, Type, Option<Flag>)
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Int,
    Varchar
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    PrimeryKey,
    ForeignKey(String),
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Eq(Box<Node>, Box<Node>)
}
