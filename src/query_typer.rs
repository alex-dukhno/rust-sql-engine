use super::catalog_manager::LockBasedCatalogManager;
use super::ast::{Statement, Type, Value, ValueSource, InsertQuery, Constraint};

pub struct QueryTyper {
    catalog_manager: LockBasedCatalogManager
}

impl QueryTyper {

    pub fn new(catalog_manager: LockBasedCatalogManager) -> QueryTyper {
        QueryTyper {
            catalog_manager: catalog_manager
        }
    }

    pub fn type_inferring(&self, statement: Statement) -> Statement {
        match statement {
            Statement::Insert(mut query) => {
                let (mut columns, value_types): (Vec<String>, Vec<(Option<String>, Type)>) = self.catalog_manager.get_table_columns(query.table_name.as_str()).into_iter().filter(|c| !query.columns.contains(&(c.0))).unzip();
                query.columns.append(&mut columns);
                let new_values = match query.values {
                    ValueSource::Row(mut query_values) => {
                        let mut default_values = value_types.into_iter()
                            .filter(|&(ref d, _)| d.is_some())
                            .map(|(s, t)| match t {
                                Type::Integer => Value::NumConst(s.unwrap()),
                                Type::VarChar(_) => Value::StrConst(s.unwrap()),
                            }).collect::<Vec<Value>>();
                        query_values.append(&mut default_values);
                        query_values
                    },
                    _ => unimplemented!(),
                };
                let new = InsertQuery::new(query.table_name, query.columns, ValueSource::Row(new_values));
                Statement::Insert(new)
            },
            s => s,
        }
    }
}

