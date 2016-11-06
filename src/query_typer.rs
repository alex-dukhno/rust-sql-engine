use super::catalog_manager::LockBasedCatalogManager;
use super::ast::{RawStatement, RawColumn, Type, TypedStatement, TypedColumn};
use super::ast::insert_query::{Value, ValueSource, InsertQuery};
use super::ast::create_table::{CreateTableQuery, ColumnTable};
use super::ast::select_query::SelectQuery;

pub fn type_inferring(catalog_manager: LockBasedCatalogManager, statement: RawStatement) -> Result<TypedStatement, String> {
    match statement {
        RawStatement::Create(create_table_query) => {
            let CreateTableQuery { table_name, table_columns } = create_table_query;
            let columns = infer_table_columns_type(table_columns);
            Ok(TypedStatement::Create(CreateTableQuery::new(table_name.as_str(), columns)))
        }
        RawStatement::Insert(query) => {
            let columns = resolve_columns(&query, &catalog_manager);
            let mut value_types = resolve_missed_column_value_types(&query, &catalog_manager);
            let new_values = match query.values {
                ValueSource::Row(mut query_values) => {
                    query_values.append(&mut value_types);
                    ValueSource::Row(query_values)
                }
                ValueSource::SubQuery(query) => {
                    ValueSource::SubQuery(typed_from_raw(query, &catalog_manager))
                }
            };
            Ok(TypedStatement::Insert(InsertQuery::new(query.table_name, columns, new_values)))
        }
        RawStatement::Select(query) => {
            Ok(TypedStatement::Select(typed_from_raw(query, &catalog_manager)))
        }
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

fn resolve_columns(query: &InsertQuery<RawColumn>, catalog_manager: &LockBasedCatalogManager) -> Vec<TypedColumn> {
    let mut query_columns = catalog_manager.get_table_columns(query.table_name.as_str())
        .into_iter()
        .filter(|c| query.columns.contains(&RawColumn::new(c.name.as_str())))
        .map(|c| TypedColumn::new(c.name, c.col_type))
        .collect::<Vec<TypedColumn>>();
    let mut missed_columns = catalog_manager.get_table_columns(query.table_name.as_str())
        .into_iter()
        .filter(|c| !query.columns.contains(&RawColumn::new(c.name.as_str())))
        .map(|c| TypedColumn::new(c.name, c.col_type))
        .collect::<Vec<TypedColumn>>();
    query_columns.append(&mut missed_columns);
    query_columns
}

fn resolve_missed_column_value_types(query: &InsertQuery<RawColumn>, catalog_manager: &LockBasedCatalogManager) -> Vec<Value> {
    catalog_manager.get_table_columns(query.table_name.as_str())
        .into_iter()
        .filter(|c| !query.columns.contains(&RawColumn::new(c.name.as_str())) && c.default_val.is_some())
        .map(|c| match c.col_type {
            Type::Integer => Value::NumConst(c.default_val.unwrap()),
            Type::Character(_) => Value::StrConst(c.default_val.unwrap()),
        }).collect::<Vec<Value>>()
}

fn typed_from_raw(query: SelectQuery<RawColumn>, catalog_manager: &LockBasedCatalogManager) -> SelectQuery<TypedColumn> {
    let table_name = query.table_name.as_str();
    let typed = query.columns.into_iter().map(|c| {
        let t = catalog_manager.get_column_type(table_name, &c.name);
        TypedColumn::new(c.name, t)
    }).collect::<Vec<TypedColumn>>();
    SelectQuery::new(table_name, typed, query.predicates)
}
