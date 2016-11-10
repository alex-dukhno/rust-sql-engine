use std::collections::HashMap;

use sql::lexer::tokenize;
use sql::parser::parse;
use sql::query_typer::type_inferring_old;
use sql::query_typer::type_inferring;
use sql::catalog_manager::CatalogManager;
use sql::catalog::ColumnMetadata;

fn assert_that_types_will_be_inferred(sql: &str, expected_dsl: &str, tables_set: &HashMap<String, Vec<ColumnMetadata>>) {
    match tokenize(sql)
            .and_then(parse)
            .and_then(|parsed| type_inferring(tables_set, parsed)) {
        Ok(good) => assert_eq!(format!("{:?}", good), expected_dsl),
        Err(e) => panic!("An unexpected type inferring failure. {:?}", e)
    }
}

fn assert_that_types_will_be_inferred_old(src_sql: &str, expected_dsl: &str, catalog_manager: &CatalogManager) {
    let typed_statement = tokenize(src_sql)
        .and_then(parse)
        .and_then(|parsed_statement| type_inferring_old(catalog_manager, parsed_statement));
    match typed_statement {
        Ok(good) => assert_eq!(format!("{:?}", good), expected_dsl),
        Err(e) => panic!("An unexpected type inferring failer.\n{:?}", e)
    }
}

#[cfg(test)]
mod create_table_query {
    use super::assert_that_types_will_be_inferred_old;
    use super::assert_that_types_will_be_inferred;

    use std::collections::HashMap;

    use sql::catalog::ColumnMetadata;

    use sql::catalog_manager::CatalogManager;

    #[test]
    fn inferring_int_type_of_column() {
        assert_that_types_will_be_inferred(
            "create table int_table (col1 int);",
            "statement: 'create table', table name: 'int_table', columns: [<name: 'col1', type: 'integer', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]",
            &HashMap::new()
        );
    }

    #[test]
    fn inferring_character_type_of_column() {
        assert_that_types_will_be_inferred(
            "create table char_table (col1 char(10));",
            "statement: 'create table', table name: 'char_table', columns: [<name: 'col1', type: 'character[10]', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]",
            &HashMap::new()
        );
    }

    #[test]
    fn default_size_for_char_should_be_255() {
        assert_that_types_will_be_inferred(
            "create table tab1 (col1 char);",
            "statement: 'create table', table name: 'tab1', columns: [<name: 'col1', type: 'character[255]', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]",
            &HashMap::new()
        );
    }

    #[test]
    fn list_of_columns_with_default_char_size() {
        assert_that_types_will_be_inferred(
            "create table tab1 (col1 char, col2 char, col3 char);",
            "statement: 'create table', table name: 'tab1', columns: [<name: 'col1', type: 'character[255]', primary key: No, foreign key: No, nullable: Yes, default value: NULL>, <name: 'col2', type: 'character[255]', primary key: No, foreign key: No, nullable: Yes, default value: NULL>, <name: 'col3', type: 'character[255]', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]",
            &HashMap::new()
        )
    }
}

#[cfg(test)]
mod insert_query {
    use super::assert_that_types_will_be_inferred_old;
    use super::super::evaluate_query;

    use sql::catalog_manager::CatalogManager;
    use sql::data_manager::DataManager;

    #[test]
    fn populates_columns_for_insert_query() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table2 (col1 integer, col2 integer, col3 integer);",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred_old(
            "insert into table2 values (1, 2, 3);",
            "statement: 'insert', table name: 'table2', columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>, <name: 'col3', type: 'integer'>], values: [<value: 1, type: integer>, <value: 2, type: integer>, <value: 3, type: integer>]",
            &catalog_manager
        );
    }

    #[test]
    fn populates_only_missed_column() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table_1 (col1 integer default 1, col2 integer);",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred_old(
            "insert into table_1 (col2) values (2);",
            "statement: 'insert', table name: 'table_1', columns: [<name: 'col2', type: 'integer'>, <name: 'col1', type: 'integer'>], values: [<value: 2, type: integer>, <value: 1, type: integer>]",
            &catalog_manager
        );
    }

    #[test]
    fn populates_default_value_for_different_types() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table_2 (col1 integer default 1, col2 integer, col3 char(3) default 'str');",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred_old(
            "insert into table_2 (col2) values (2);",
            "statement: 'insert', table name: 'table_2', columns: [<name: 'col2', type: 'integer'>, <name: 'col1', type: 'integer'>, <name: 'col3', type: 'character[3]'>], values: [<value: 2, type: integer>, <value: 1, type: integer>, <value: str, type: character[3]>]",
            &catalog_manager
        );
    }

    #[test]
    fn populates_types_of_columns_in_select_sub_query() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table_1 (col1 integer default 1, col2 integer default 2);",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred_old(
            "insert into table_1 (col1, col2) select col1, col2 from table_1;",
            "statement: 'insert', table name: 'table_1', columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>], values: <substatement: 'select', tables: [<name: 'table_1'>], columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>], where: no predicate>",
            &catalog_manager
        );
    }
}

#[cfg(test)]
mod select_query {
    use super::assert_that_types_will_be_inferred_old;
    use super::super::evaluate_query;

    use sql::catalog_manager::CatalogManager;
    use sql::data_manager::DataManager;

    #[test]
    fn single_column_query() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table_1 (col1 integer);",
                &DataManager::default(),
                &catalog_manager.clone()
            )
        );

        assert_that_types_will_be_inferred_old(
            "select col1 from table_1;",
            "statement: 'select', tables: [<name: 'table_1'>], columns: [<name: 'col1', type: 'integer'>], where: no predicate",
            &catalog_manager
        );
    }

    #[test]
    fn multiple_columns_query() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table_3 (col2 integer, col3 char(10), col5 integer);",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred_old(
            "select col2, col3, col5 from table_3;",
            "statement: 'select', tables: [<name: 'table_3'>], columns: [<name: 'col2', type: 'integer'>, <name: 'col3', type: 'character[10]'>, <name: 'col5', type: 'integer'>], where: no predicate",
            &catalog_manager
        );
    }

    #[test]
    fn not_all_columns() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table_2 (col1 integer, col2 integer, col3 integer);",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred_old(
            "select col1, col3 from table_2;",
            "statement: 'select', tables: [<name: 'table_2'>], columns: [<name: 'col1', type: 'integer'>, <name: 'col3', type: 'integer'>], where: no predicate",
            &catalog_manager
        );
    }
}
