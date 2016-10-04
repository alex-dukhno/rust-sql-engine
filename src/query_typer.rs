use super::query_executer::execute;
use super::query_executer::ExecutionResult;
use super::query_validator::validate;
use super::data_manager::LockBaseDataManager;
use super::catalog_manager::LockBasedCatalogManager;
use super::ast::{RawStatement, Type, TypedStatement};
use super::ast::insert_query::{Value, ValueSource, InsertQuery};
use super::ast::create_table::{CreateTableQuery, ColumnTable};

pub fn type_inferring(catalog_manager: LockBasedCatalogManager, statement: RawStatement) -> Result<TypedStatement, String> {
    match statement {
        RawStatement::Create(create_table_query) => {
            let CreateTableQuery { table_name, columns } = create_table_query;
            let columns = columns.into_iter().map(
                |mut column| {
                    column.column_type = match column.column_type {
                        Type::Character(None) => Type::Character(Option::from(255)),
                        col_type => col_type
                    };
                    column
                }
            ).collect::<Vec<ColumnTable>>();
            Ok(TypedStatement::Create(CreateTableQuery::new(table_name.as_str(), columns)))
        },
        RawStatement::Insert(mut query) => {
            let (mut columns, value_types): (Vec<String>, Vec<(Option<String>, Type)>) = catalog_manager.get_table_columns(query.table_name.as_str()).into_iter().filter(|c| !query.columns.contains(&(c.0))).unzip();
            query.columns.append(&mut columns);
            let new_values = match query.values {
                ValueSource::Row(mut query_values) => {
                    let mut default_values = value_types.into_iter()
                        .filter(|&(ref d, _)| d.is_some())
                        .map(|(s, t)| match t {
                            Type::Integer => Value::NumConst(s.unwrap()),
                            Type::Character(_) => Value::StrConst(s.unwrap()),
                        }).collect::<Vec<Value>>();
                    query_values.append(&mut default_values);
                    query_values
                },
                _ => unimplemented!(),
            };
            let new = InsertQuery::new(query.table_name, query.columns, ValueSource::Row(new_values));
            Ok(TypedStatement::Insert(new))
        },
        RawStatement::Select(query) => Ok(TypedStatement::Select(query)),
        s => panic!("unimplemented type inferring for {:?}", s)
    }
}

