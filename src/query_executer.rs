use super::parser::Node::{self, Create, Table, Insert, Values, TableColumn};

type CodeError = i32;

pub struct QueryExecuter {
    tables: Vec<DatabaseTable>
}

struct DatabaseTable {
    name: String,
    columns: Vec<String>,
}

impl QueryExecuter {

    pub fn new() -> QueryExecuter {
         QueryExecuter {
             tables: vec![]
         }
    }

    pub fn execute(&mut self, query: Node) -> Result<String, String> {
        match query {
            Create(table) => self.create_table(*table),
            Insert(table, values) => self.insert_into(*table, *values),
            _ => Err("".to_owned()),
        }
    }

    fn create_table(&mut self, table: Node) -> Result<String, String> {
        match table {
            Table(name, Some(columns)) => {
                let columns = columns.into_iter().map(
                    |tc| {
                        match tc {
                            TableColumn(name, _, _) => name,
                            _ => "".to_owned(),
                        }
                    }
                ).collect::<Vec<String>>();
                let s = name.clone();
                self.tables.push( DatabaseTable { name: name, columns: columns } );
                Ok(format!("'{}' was created", s))
            },
            Table(name, None) => { self.tables.push( DatabaseTable { name: name, columns: vec![] } ); Ok("".to_owned()) },
            _ => Err("".to_owned()),
        }
    }

    fn insert_into(&self, table: Node, values: Node) -> Result<String, String> {
        match table {
            Table(name, _) => {
                if self.tables.iter().any(|t| t.name == name) {
                    match values {
                        Values(data) => {
                            let ref t = self.tables[0];
                            if data.len() != t.columns.len() {
                                return Err("".to_owned());
                            }
                        },
                        _ => return Err("".to_owned()),
                    }
                    Ok("".to_owned())
                }
                else {
                    Err(format!("[ERR 100] table '{}' does not exist", name))
                }
            },
            _ => Err("".to_owned()),
        }
    }
}
