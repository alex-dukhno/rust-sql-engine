use sql::lexer::tokenize;
use sql::parser::parse;

fn assert_that_statement_parsed_into(src: &str, expected: &str) {
    match tokenize(src).and_then(parse) {
        Ok(good) => assert_eq!(format!("{:?}", good), expected),
        Err(exception) => assert_eq!(exception, expected)
    }
}

#[cfg(test)]
mod should_parse {
    #[cfg(test)]
    mod create_table_statement {
        use super::super::assert_that_statement_parsed_into;

        #[test]
        fn with_one_column() {
            assert_that_statement_parsed_into(
                "create table table_name_1 (col integer);",
                "statement: 'create table', table name: 'table_name_1', columns: [<name: 'col', type: 'integer', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]"
            );
        }

        #[test]
        fn with_list_of_columns() {
            assert_that_statement_parsed_into(
                "create table table_name_2 (col1 integer, col2 integer, col3 integer);",
                "statement: 'create table', table name: 'table_name_2', columns: [<name: 'col1', type: 'integer', primary key: No, foreign key: No, nullable: Yes, default value: NULL>, <name: 'col2', type: 'integer', primary key: No, foreign key: No, nullable: Yes, default value: NULL>, <name: 'col3', type: 'integer', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]")
        }

        #[test]
        fn with_varchar_column_type() {
            assert_that_statement_parsed_into(
                "create table table_1 (col_2 character(10));",
                "statement: 'create table', table name: 'table_1', columns: [<name: 'col_2', type: 'character size of 10', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]"
            );
        }

        #[test]
        fn with_default_value_constraint() {
            assert_that_statement_parsed_into(
                "create table table1 (col integer default 1);",
                "statement: 'create table', table name: 'table1', columns: [<name: 'col', type: 'integer', primary key: No, foreign key: No, nullable: Yes, default value: 1>]"
            );
        }

        #[test]
        fn infer_type_for_primary_key_column() {
            assert_that_statement_parsed_into(
                "create table table_1 (col integer primary key);",
                "statement: 'create table', table name: 'table_1', columns: [<name: 'col', type: 'integer', primary key: Yes, foreign key: No, nullable: No, default value: NULL>]"
            )
        }

        #[test]
        fn with_primary_key_discard_default_value() {
            assert_that_statement_parsed_into(
                "create table table_1 (col integer primary key default 1);",
                "statement: 'create table', table name: 'table_1', columns: [<name: 'col', type: 'integer', primary key: Yes, foreign key: No, nullable: No, default value: 1>]"
            );
        }

        #[test]
        fn not_null_constraint() {
            assert_that_statement_parsed_into(
                "create table table_2 (col integer not null);",
                "statement: 'create table', table name: 'table_2', columns: [<name: 'col', type: 'integer', primary key: No, foreign key: No, nullable: No, default value: 0>]"
            );
        }

        #[test]
        fn not_null_with_default() {
            assert_that_statement_parsed_into(
                "create table tab3 (col1 integer not null default 4, col2 integer);",
                "statement: 'create table', table name: 'tab3', columns: [<name: 'col1', type: 'integer', primary key: No, foreign key: No, nullable: No, default value: 4>, <name: 'col2', type: 'integer', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]"
            );
        }

        #[test]
        fn foreign_key_constraint() {
            assert_that_statement_parsed_into(
                "create table tab_4 (col1 integer primary key, col2 integer foreign key references table1(col));",
                "statement: 'create table', table name: 'tab_4', columns: [<name: 'col1', type: 'integer', primary key: Yes, foreign key: No, nullable: No, default value: NULL>, <name: 'col2', type: 'integer', primary key: No, foreign key: table1->col, nullable: Yes, default value: NULL>]"
            );
        }

        #[test]
        fn undefined_character_size() {
            assert_that_statement_parsed_into(
                "create table tab1 (col2 char);",
                "statement: 'create table', table name: 'tab1', columns: [<name: 'col2', type: 'character', primary key: No, foreign key: No, nullable: Yes, default value: NULL>]"
            );
        }

        #[test]
        fn character_size_more_than_256() {
            assert_that_statement_parsed_into(
                "create table tab2 (col1 char(456));",
                "number too large to fit in target type"
            );
        }

        #[test]
        fn character_size_less_than_0() {
            assert_that_statement_parsed_into(
                "create table tab3 (col6 char(-1));",
                "invalid digit found in string"
            );
        }
    }

    #[cfg(test)]
    mod delete_statements {
        use super::super::assert_that_statement_parsed_into;

        #[test]
        fn without_any_predicates() {
            assert_that_statement_parsed_into(
                "delete from table_name_1;",
                "statement: 'delete', table name: 'table_name_1', where: no predicate"
            );
        }

        #[test]
        fn with_column_const_predicate() {
            assert_that_statement_parsed_into(
                "delete from table_name_2 where col_1 = 5;",
                "statement: 'delete', table name: 'table_name_2', where: predicate <col_1 equals to 5>"
            );
        }

        #[test]
        fn with_const_column_predicate() {
            assert_that_statement_parsed_into(
                "delete from table_name_3 where 'str' = col_2;",
                "statement: 'delete', table name: 'table_name_3', where: predicate <'str' equals to col_2>"
            );
        }
    }

    #[cfg(test)]
    mod insert_statements {
        use super::super::assert_that_statement_parsed_into;

        #[test]
        fn with_one_column() {
            assert_that_statement_parsed_into(
                "insert into table_name_1 values(10);",
                "statement: 'insert', table name: 'table_name_1', columns: [], values: [<value: 10, type: integer>]"
            );
        }

        #[test]
        fn with_list_of_columns() {
            assert_that_statement_parsed_into(
                "insert into table_name_2 values (10, 'string');",
                "statement: 'insert', table name: 'table_name_2', columns: [], values: [<value: 10, type: integer>, <value: string, type: character size of 6>]"
            );
        }

        #[test]
        fn with_columns() {
            assert_that_statement_parsed_into(
                "insert into table_name_3 (col_1, col_2) values (10, 'string');",
                "statement: 'insert', table name: 'table_name_3', columns: [<name: 'col_1'>, <name: 'col_2'>], values: [<value: 10, type: integer>, <value: string, type: character size of 6>]"
            );
        }

        #[test]
        fn with_sub_select() {
            assert_that_statement_parsed_into(
                "insert into table_1 (col_1, col_2) select col_1, col_2 from table_1;",
                "statement: 'insert', table name: 'table_1', columns: [<name: 'col_1'>, <name: 'col_2'>], values: <substatement: 'select', tables: [<name: 'table_1'>], columns: [<name: 'col_1'>, <name: 'col_2'>], where: no predicate>"
            );
        }
    }

    #[cfg(test)]
    mod select_statements {
        use super::super::assert_that_statement_parsed_into;

        #[test]
        fn without_predicates() {
            assert_that_statement_parsed_into(
                "select col_1 from table_name_1;",
                "statement: 'select', tables: [<name: 'table_name_1'>], columns: [<name: 'col_1'>], where: no predicate"
            );
        }

        #[test]
        fn with_predicates() {
            assert_that_statement_parsed_into(
                "select col_2 from table_name_2 where col_2 = 10;",
                "statement: 'select', tables: [<name: 'table_name_2'>], columns: [<name: 'col_2'>], where: predicate <col_2 equals to 10>"
            );
        }

        #[test]
        fn with_limit_predicate() {
            assert_that_statement_parsed_into(
                "select col_2 from table_name_2 where limit = 10;",
                "statement: 'select', tables: [<name: 'table_name_2'>], columns: [<name: 'col_2'>], where: predicate <limit equals to 10>"
            );
        }

        #[test]
        fn with_not_equal_predicate() {
            assert_that_statement_parsed_into(
                "select col_2 from table_1 where col_1 <> \'a\';",
                "statement: 'select', tables: [<name: 'table_1'>], columns: [<name: 'col_2'>], where: predicate <col_1 not equals to 'a'>"
            );
        }
    }
}
