use super::parser::Node::{self, Create, Table, Insert, Values, TableColumn};

type CodeError = i32;

pub struct QueryExecuter {
    tables: Vec<DatabaseTable>
}

#[derive(Debug)]
struct DatabaseTable {
    name: String,
    columns: Vec<String>,
}

impl Default for QueryExecuter {

    fn default() -> Self {
        QueryExecuter {
            tables: vec![]
        }
    }
}

impl QueryExecuter {

    pub fn execute(&mut self, query: Node) -> Result<String, String> {
        println!("query - {:?}", query);
        match query {
            Create(table) => self.create_table(*table),
            Insert(table, values) => self.insert_into(*table, *values),
            _ => Err("execute".to_owned()),
        }
    }

    fn create_table(&mut self, table: Node) -> Result<String, String> {
        match table {
            Table(name, columns) => {
                let columns = columns.into_iter().map(
                    |tc| {
                        match tc {
                            TableColumn(name, _, _) => name,
                            _ => "not a table column".to_owned(),
                        }
                    }
                ).collect::<Vec<String>>();
                println!("columns - {:?}", columns);
                let s = name.clone();
                self.tables.push( DatabaseTable { name: name, columns: columns } );
                Ok(format!("'{}' was created", s))
            },
            //Table(name, _) => { self.tables.push( DatabaseTable { name: name, columns: vec![] } ); Ok("".to_owned()) },
            _ => Err("not a table".to_owned()),
        }
    }

    fn insert_into(&self, table: Node, values: Node) -> Result<String, String> {
        match table {
            Table(name, _) => {
                if self.tables.iter().any(|t| t.name == name) {
                    match values {
                        Values(data) => {
                            println!("data - {:?}", data);
                            let t = &self.tables[0];
                            println!("table - {:?}", t);
                            if data.len() != t.columns.len() {
                                return Err("more column than expected".to_owned());
                            }
                        },
                        _ => return Err("not a values".to_owned()),
                    }
                    Ok("row was inserted".to_owned())
                }
                else {
                    Err(format!("[ERR 100] table '{}' does not exist", name))
                }
            },
            _ => Err("not a table".to_owned()),
        }
    }
}
