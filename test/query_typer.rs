use sql::lexer::tokenize;
use sql::parser::parse;
use sql::query_typer::type_inferring;
use sql::catalog_manager::CatalogManager;

fn assert_that_types_will_be_inferred(src_sql: &str, expected_dsl: &str, catalog_manager: &CatalogManager) {
    let typed_statement = tokenize(src_sql)
        .and_then(parse)
        .and_then(|parsed_statement| type_inferring(catalog_manager, parsed_statement));
    match typed_statement {
        Ok(good) => assert_eq!(format!("{:?}", good), expected_dsl),
        Err(e) => panic!("An unexpected type inferring failer.\n{:?}", e)
    }
}

#[cfg(test)]
mod create_table_query {
    use super::assert_that_types_will_be_inferred;

    use sql::catalog_manager::CatalogManager;

    #[test]
    fn default_size_for_char_should_be_255() {
        assert_that_types_will_be_inferred(
            "create table tab1 (col1 char);",
            "statement: 'create table', table name: 'tab1', columns: [<name: 'col1', type: 'character size of 255', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]",
            &CatalogManager::default()
        );
    }

    #[test]
    fn list_of_columns_with_default_char_size() {
        assert_that_types_will_be_inferred(
            "create table tab1 (col1 char, col2 char, col3 char);",
            "statement: 'create table', table name: 'tab1', columns: [<name: 'col1', type: 'character size of 255', primary key: No, foreign key: No, nullable: Yes, default value: NULL>, <name: 'col2', type: 'character size of 255', primary key: No, foreign key: No, nullable: Yes, default value: NULL>, <name: 'col3', type: 'character size of 255', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]",
            &CatalogManager::default()
        )
    }
}

#[cfg(test)]
mod insert_query {
    use super::assert_that_types_will_be_inferred;
    use super::super::evaluate_query;

    use sql::catalog_manager::CatalogManager;
    use sql::data_manager::DataManager;

    #[test]
    fn populates_columns_for_insert_query() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table2 (col1 integer, col2 integer, col3 integer)",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred(
            "insert into table2 values (1, 2, 3);",
            "statement: 'insert', table name: 'table2', columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>, <name: 'col3', type: 'integer'>], values: [Numeric(1), Numeric(2), Numeric(3)]",
            &catalog_manager
        );
    }

    #[test]
    fn populates_only_missed_column() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table_1 (col1 integer default 1, col2 integer)",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred(
            "insert into table_1 (col2) values (2);",
            "statement: 'insert', table name: 'table_1', columns: [<name: 'col2', type: 'integer'>, <name: 'col1', type: 'integer'>], values: [Numeric(2), Numeric(1)]",
            &catalog_manager
        );
    }

    #[test]
    fn populates_default_value_for_different_types() {
        let catalog_manager = CatalogManager::default();

        drop(
            evaluate_query(
                "create table table_2 (col1 integer default 1, col2 integer, col3 char(3) default 'str')",
                &DataManager::default(),
                &catalog_manager
            )
        );

        assert_that_types_will_be_inferred(
            "insert into table_2 (col2) values (2);",
            "statement: 'insert', table name: 'table_2', columns: [<name: 'col2', type: 'integer'>, <name: 'col1', type: 'integer'>, <name: 'col3', type: 'character size of 3'>], values: [Numeric(2), Numeric(1), String(str)]",
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

        assert_that_types_will_be_inferred(
            "insert into table_1 (col1, col2) select col1, col2 from table_1;",
            "statement: 'insert', table name: 'table_1', columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>], values: <substatement: 'select', tables: [<name: 'table_1'>], columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>], where: no predicate>",
            &catalog_manager
        );
    }
}

#[cfg(test)]
mod select_query {
    use super::assert_that_types_will_be_inferred;
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

        assert_that_types_will_be_inferred(
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

        assert_that_types_will_be_inferred(
            "select col2, col3, col5 from table_3;",
            "statement: 'select', tables: [<name: 'table_3'>], columns: [<name: 'col2', type: 'integer'>, <name: 'col3', type: 'character size of 10'>, <name: 'col5', type: 'integer'>], where: no predicate",
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

        assert_that_types_will_be_inferred(
            "select col1, col3 from table_2;",
            "statement: 'select', tables: [<name: 'table_2'>], columns: [<name: 'col1', type: 'integer'>, <name: 'col3', type: 'integer'>], where: no predicate",
            &catalog_manager
        );
    }
}
