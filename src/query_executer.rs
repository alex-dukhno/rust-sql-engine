use super::parser::ast::Node::{self, Create, Table, Insert, Values, TableColumn, NumberC, StringC};
use super::parser::ast::Type::{self, Int, Varchar};

pub struct QueryExecuter {
    tables: Vec<DatabaseTable>
}

#[derive(Debug)]
struct DatabaseTable {
    name: String,
    columns: Vec<DatabaseColumn>,
}

#[derive(Debug)]
struct DatabaseColumn {
    name: String,
    column_type: Type,
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
                            TableColumn(name, column_type, _) => DatabaseColumn { name: name, column_type: column_type },
                            _ => DatabaseColumn { name: "not a column".to_owned(), column_type: Int },
                        }
                    }
                ).collect::<Vec<DatabaseColumn>>();
                println!("columns - {:?}", columns);
                let s = name.clone();
                self.tables.push(DatabaseTable { name: name, columns: columns });
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
                            for (index, datum) in data.into_iter().enumerate() {
                                match datum {
                                    NumberC(_) => if t.columns[index].column_type != Int {
                                        return Err("column type is VARCHAR find INT".to_owned());
                                    },
                                    StringC(_) => if t.columns[index].column_type != Varchar {
                                        return Err("column type is INT find VARCHAR".to_owned());
                                    },
                                    _ => return Err("wrong node".to_owned()),
                                }
                            }
                        },
                        _ => return Err("not a values".to_owned()),
                    }
                    Ok("row was inserted".to_owned())
                } else {
                    Err(format!("[ERR 100] table '{}' does not exist", name))
                }
            },
            _ => Err("not a table".to_owned()),
        }
    }
}
