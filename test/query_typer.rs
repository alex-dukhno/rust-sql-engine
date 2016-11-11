use std::collections::HashMap;

use sql::lexer::tokenize;
use sql::parser::parse;
use sql::query_typer::type_inferring;
use sql::catalog::ColumnMetadata;

fn assert_that_types_will_be_inferred(sql: &str, expected_dsl: &str, tables_set: &HashMap<String, Vec<ColumnMetadata>>) {
    match tokenize(sql)
            .and_then(parse)
            .and_then(|parsed| type_inferring(tables_set, parsed)) {
        Ok(good) => assert_eq!(format!("{:?}", good), expected_dsl),
        Err(e) => panic!("An unexpected type inferring failure. {:?}", e)
    }
}

#[cfg(test)]
mod create_table_query {
    use super::assert_that_types_will_be_inferred;

    use std::collections::HashMap;

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
    use super::assert_that_types_will_be_inferred;

    use std::collections::HashMap;

    use sql::catalog::ColumnMetadata;
    use sql::ast::Type;


    #[test]
    fn populates_columns_for_insert_query() {
        let mut table = HashMap::new();
        let columns = vec![
            ColumnMetadata::new("col1", Type::Integer, None),
            ColumnMetadata::new("col2", Type::Integer, None),
            ColumnMetadata::new("col3", Type::Integer, None)
        ];
        table.insert("table2".into(), columns);

        assert_that_types_will_be_inferred(
            "insert into table2 values (1, 2, 3);",
            "statement: 'insert', table name: 'table2', columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>, <name: 'col3', type: 'integer'>], values: [<value: 1, type: integer>, <value: 2, type: integer>, <value: 3, type: integer>]",
            &table
        );
    }

    #[test]
    fn populates_only_missed_column() {
        let mut table = HashMap::new();
        let columns = vec![
            ColumnMetadata::new("col1", Type::Integer, Some("1")),
            ColumnMetadata::new("col2", Type::Integer, None)
        ];
        table.insert("table_1".into(), columns);

        assert_that_types_will_be_inferred(
            "insert into table_1 (col2) values (2);",
            "statement: 'insert', table name: 'table_1', columns: [<name: 'col2', type: 'integer'>, <name: 'col1', type: 'integer'>], values: [<value: 2, type: integer>, <value: 1, type: integer>]",
            &table
        );
    }

    #[test]
    fn populates_default_value_for_different_types() {
        let mut table = HashMap::new();
        let columns = vec![
            ColumnMetadata::new("col1", Type::Integer, Some("1")),
            ColumnMetadata::new("col2", Type::Integer, None),
            ColumnMetadata::new("col3", Type::Character(Some(3)), Some("str"))
        ];
        table.insert("table_2".into(), columns);

        assert_that_types_will_be_inferred(
            "insert into table_2 (col2) values (2);",
            "statement: 'insert', table name: 'table_2', columns: [<name: 'col2', type: 'integer'>, <name: 'col1', type: 'integer'>, <name: 'col3', type: 'character[3]'>], values: [<value: 2, type: integer>, <value: 1, type: integer>, <value: str, type: character[3]>]",
            &table
        );
    }

    #[test]
    fn populates_types_of_columns_in_select_sub_query() {
        let mut table = HashMap::new();
        let columns = vec![
            ColumnMetadata::new("col1", Type::Integer, Some("1")),
            ColumnMetadata::new("col2", Type::Integer, Some("2"))
        ];
        table.insert("table_1".into(), columns);

        assert_that_types_will_be_inferred(
            "insert into table_1 (col1, col2) select col1, col2 from table_1;",
            "statement: 'insert', table name: 'table_1', columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>], values: <substatement: 'select', tables: [<name: 'table_1'>], columns: [<name: 'col1', type: 'integer'>, <name: 'col2', type: 'integer'>], where: no predicate>",
            &table
        );
    }
}

#[cfg(test)]
mod select_query {
    use super::assert_that_types_will_be_inferred;

    use std::collections::HashMap;

    use sql::ast::Type;
    use sql::catalog::ColumnMetadata;

    #[test]
    fn single_column_query() {
        let mut table = HashMap::new();
        let columns = vec![
            ColumnMetadata::new("col1", Type::Integer, None)
        ];
        table.insert("table_1".into(), columns);

        assert_that_types_will_be_inferred(
            "select col1 from table_1;",
            "statement: 'select', tables: [<name: 'table_1'>], columns: [<name: 'col1', type: 'integer'>], where: no predicate",
            &table
        );
    }

    #[test]
    fn multiple_columns_query() {
        let mut table = HashMap::new();
        let columns = vec![
            ColumnMetadata::new("col2", Type::Integer, None),
            ColumnMetadata::new("col3", Type::Character(Some(10)), None),
            ColumnMetadata::new("col5", Type::Integer, None)
        ];
        table.insert("table_3".into(), columns);

        assert_that_types_will_be_inferred(
            "select col2, col3, col5 from table_3;",
            "statement: 'select', tables: [<name: 'table_3'>], columns: [<name: 'col2', type: 'integer'>, <name: 'col3', type: 'character[10]'>, <name: 'col5', type: 'integer'>], where: no predicate",
            &table
        );
    }

    #[test]
    fn not_all_columns() {
        let mut table = HashMap::new();
        let columns = vec![
            ColumnMetadata::new("col1", Type::Integer, None),
            ColumnMetadata::new("col2", Type::Integer, None),
            ColumnMetadata::new("col3", Type::Integer, None)
        ];
        table.insert("table_2".into(), columns);

        assert_that_types_will_be_inferred(
            "select col1, col3 from table_2;",
            "statement: 'select', tables: [<name: 'table_2'>], columns: [<name: 'col1', type: 'integer'>, <name: 'col3', type: 'integer'>], where: no predicate",
            &table
        );
    }
}
