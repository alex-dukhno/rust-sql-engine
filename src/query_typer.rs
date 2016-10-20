use super::catalog_manager::LockBasedCatalogManager;
use super::ast::{RawStatement, Type, TypedStatement};
use super::ast::insert_query::{Value, ValueSource, InsertQuery};
use super::ast::create_table::{CreateTableQuery, ColumnTable};
use super::ast::select_query::TypedSelectQuery;

pub fn type_inferring(catalog_manager: LockBasedCatalogManager, statement: RawStatement) -> Result<TypedStatement, String> {
    match statement {
        RawStatement::Create(create_table_query) => {
            let CreateTableQuery { table_name, table_columns } = create_table_query;
            let columns = infer_table_columns_type(table_columns);
            Ok(TypedStatement::Create(CreateTableQuery::new(table_name.as_str(), columns)))
        },
        RawStatement::Insert(mut query) => {
            let mut column_names = resolve_missed_column_names(&query, &catalog_manager);
            let mut value_types = resolve_missed_column_value_types(&query, &catalog_manager);
            query.columns.append(&mut column_names);
            let new_values = match query.values {
                ValueSource::Row(mut query_values) => {
                    query_values.append(&mut value_types);
                    query_values
                }
                vs => panic!("unimplemented raw -> typed transformation for insert query with {:?} value source", vs)
            };
            let new = InsertQuery::new(query.table_name, query.columns, ValueSource::Row(new_values));
            Ok(TypedStatement::Insert(new))
        }
        RawStatement::Select(query) => {
            let typed = query.columns.into_iter().map(|c| (c, Type::Integer)).collect::<Vec<(String, Type)>>();
            Ok(TypedStatement::Select(TypedSelectQuery::new(query.table_name, typed, query.condition)))
        },
        s => panic!("unimplemented type inferring for {:?}", s)
    }
}

fn infer_table_columns_type(table_columns: Vec<ColumnTable>) -> Vec<ColumnTable> {
    table_columns.into_iter().map(
        |mut column| {
            column.column_type = match column.column_type {
                Type::Character(None) => Type::Character(Option::from(255)),
                col_type => col_type
            };
            column
        }
    ).collect::<Vec<ColumnTable>>()
}

fn resolve_missed_column_names(query: &InsertQuery, catalog_manager: &LockBasedCatalogManager) -> Vec<String> {
    catalog_manager.get_table_columns(query.table_name.as_str())
        .into_iter()
        .filter(|c| !query.columns.contains(&c.name))
        .map(|c| c.name).collect::<Vec<String>>()
}

fn resolve_missed_column_value_types(query: &InsertQuery, catalog_manager: &LockBasedCatalogManager) -> Vec<Value> {
    catalog_manager.get_table_columns(query.table_name.as_str())
        .into_iter()
        .filter(|c| !query.columns.contains(&c.name) && c.default_val.is_some())
        .map(|c| match c.col_type {
            Type::Integer => Value::NumConst(c.default_val.unwrap()),
            Type::Character(_) => Value::StrConst(c.default_val.unwrap()),
        }).collect::<Vec<Value>>()
}
