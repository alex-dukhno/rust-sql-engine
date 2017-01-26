use std::collections::HashMap;

use super::catalog_manager::CatalogManager;
use super::catalog::ColumnMetadata;
use super::ast::{RawStatement, RawColumn, Type, TypedStatement, TypedColumn};
use super::ast::insert_query::{Value, ValueSource, InsertQuery};
use super::ast::create_table::{CreateTableQuery, ColumnTable};
use super::ast::select_query::SelectQuery;

pub fn type_inferring(tables_set: &HashMap<String, Vec<ColumnMetadata>>, statement: RawStatement) -> Result<TypedStatement, String> {
    match statement {
        RawStatement::Create(mut create_table_query) => {
            infer_table_columns_type(&mut create_table_query.table_columns);
            Ok(TypedStatement::Create(create_table_query))
        }
        RawStatement::Insert(query) => {
            let columns = resolve_columns(&query, tables_set);
            let mut value_types = resolve_missed_column_value_types(&query, tables_set);
            let new_values = match query.values {
                ValueSource::Row(mut query_values) => {
                    query_values.append(&mut value_types);
                    ValueSource::Row(query_values)
                }
                ValueSource::SubQuery(query) => {
                    ValueSource::SubQuery(typed_from_raw(query, tables_set))
                }
            };
            Ok(TypedStatement::Insert(InsertQuery::new(query.table_name, columns, new_values)))
        }
        RawStatement::Select(query) => {
            Ok(TypedStatement::Select(typed_from_raw(query, tables_set)))
        }
        s => Err(format!("unimplemented type inferring for {:?}", s))
    }
}

fn infer_table_columns_type(table_columns: &mut Vec<ColumnTable>) {
    for col in table_columns.iter_mut() {
        if col.column_type == Type::Character(None) {
            col.column_type = Type::Character(Option::from(255));
        }
    }
}

fn resolve_columns(query: &InsertQuery<RawColumn>, table_set: &HashMap<String, Vec<ColumnMetadata>>) -> Vec<TypedColumn> {
    let mut query_columns = match table_set.get(query.table_name.as_str()) {
        Some(v) => v.iter()
            .filter(|c| query.columns.contains(&RawColumn::new(c.name.as_str())))
            .map(|c| TypedColumn::new(c.name.as_str(), c.col_type))
            .collect::<Vec<TypedColumn>>(),
        _ => vec![]
    };
    let mut missed_columns = match table_set.get(query.table_name.as_str()) {
        Some(v) => v.iter()
            .filter(|c| !query.columns.contains(&RawColumn::new(c.name.as_str())))
            .map(|c| TypedColumn::new(c.name.as_str(), c.col_type))
            .collect::<Vec<TypedColumn>>(),
        _ => vec![]
    };
    query_columns.append(&mut missed_columns);
    query_columns
}

fn resolve_missed_column_value_types(query: &InsertQuery<RawColumn>, table_set: &HashMap<String, Vec<ColumnMetadata>>) -> Vec<Value> {
    match table_set.get(query.table_name.as_str()) {
        Some(v) => v.into_iter()
            .filter(|c| !query.columns.contains(&RawColumn::new(c.name.as_str())) && c.default_val.is_some())
            .map(
                |ref c| match c.col_type {
                    Type::Integer => Value::new(c.default_val.as_ref().unwrap().as_str(), Type::Integer),
                    Type::Character(_) => {
                        let val = c.default_val.as_ref().unwrap().as_str();
                        let size = val.len() as u8;
                        Value::new(val, Type::Character(Option::from(size)))
                    }
                }
            ).collect::<Vec<Value>>(),
        _ => vec![]
    }
}

fn typed_from_raw(query: SelectQuery<RawColumn>, table_set: &HashMap<String, Vec<ColumnMetadata>>) -> SelectQuery<TypedColumn> {
    let table_name = query.table_name.as_str();
    let typed = query.columns.into_iter().map(|c| {
        let t = match table_set.get(table_name).and_then(|v| v.iter().find(|&rc| rc.name == c.name)) {
            Some(ref col) => col.col_type,
            _ => panic!("unimplemented if column with <{}> name does not exist in <{}> table", c.name, table_name)
        };
        TypedColumn::new(c.name, t)
    }).collect::<Vec<TypedColumn>>();
    SelectQuery::new(table_name, typed, query.predicates)
}




pub fn type_inferring_old(catalog_manager: &CatalogManager, statement: RawStatement) -> Result<TypedStatement, String> {
    match statement {
        RawStatement::Create(create_table_query) => {
            let CreateTableQuery { table_name, table_columns } = create_table_query;
            let columns = infer_table_columns_type_old(table_columns);
            Ok(TypedStatement::Create(CreateTableQuery::new(table_name.as_str(), columns)))
        }
        RawStatement::Insert(query) => {
            let columns = resolve_columns_old(&query, catalog_manager);
            let mut value_types = resolve_missed_column_value_types_old(&query, catalog_manager);
            println!("value types - {:?}", value_types);
            let new_values = match query.values {
                ValueSource::Row(mut query_values) => {
                    query_values.append(&mut value_types);
                    ValueSource::Row(query_values)
                }
                ValueSource::SubQuery(query) => {
                    ValueSource::SubQuery(typed_from_raw_old(query, catalog_manager))
                }
            };
            Ok(TypedStatement::Insert(InsertQuery::new(query.table_name, columns, new_values)))
        }
        RawStatement::Select(query) => {
            Ok(TypedStatement::Select(typed_from_raw_old(query, catalog_manager)))
        }
        s => panic!("unimplemented type inferring for {:?}", s)
    }
}

fn infer_table_columns_type_old(table_columns: Vec<ColumnTable>) -> Vec<ColumnTable> {
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

fn resolve_columns_old(query: &InsertQuery<RawColumn>, catalog_manager: &CatalogManager) -> Vec<TypedColumn> {
    let mut query_columns = catalog_manager.get_table_columns(query.table_name.as_str())
        .into_iter()
        .filter(|c| query.columns.contains(&RawColumn::new(c.name.as_str())))
        .map(|c| TypedColumn::new(c.name.as_str(), c.col_type))
        .collect::<Vec<TypedColumn>>();
    let mut missed_columns = catalog_manager.get_table_columns(query.table_name.as_str())
        .into_iter()
        .filter(|c| !query.columns.contains(&RawColumn::new(c.name.as_str())))
        .map(|c| TypedColumn::new(c.name.as_str(), c.col_type))
        .collect::<Vec<TypedColumn>>();
    query_columns.append(&mut missed_columns);
    query_columns
}

fn resolve_missed_column_value_types_old(query: &InsertQuery<RawColumn>, catalog_manager: &CatalogManager) -> Vec<Value> {
    catalog_manager.get_table_columns(query.table_name.as_str())
        .into_iter()
        .filter(|c| !query.columns.contains(&RawColumn::new(c.name.as_str())) && c.default_val.is_some())
        .map(
            |ref c| match c.col_type {
                    Type::Integer => Value::new(c.default_val.as_ref().unwrap().as_str(), Type::Integer),
                    Type::Character(_) => {
                        let val = c.default_val.as_ref().unwrap().as_str();
                        let size = val.len() as u8;
                        Value::new(val, Type::Character(Option::from(size)))
                    }
                }
        ).collect::<Vec<Value>>()
}

fn typed_from_raw_old(query: SelectQuery<RawColumn>, catalog_manager: &CatalogManager) -> SelectQuery<TypedColumn> {
    let table_name = query.table_name.as_str();
    let typed = query.columns.into_iter().map(|c| {
        let t = catalog_manager.get_column_type(table_name, &c.name);
        TypedColumn::new(c.name, t)
    }).collect::<Vec<TypedColumn>>();
    SelectQuery::new(table_name, typed, query.predicates)
}
