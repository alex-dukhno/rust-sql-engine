#[cfg(test)]
mod parses_create_table_statement {
    use expectest::prelude::be_ok;

    use sql::lexer::tokenize;
    use sql::parser::parse;
    use sql::ast::{Type, Statement, Constraint, CreateTableQuery, ColumnTable};

    #[test]
    fn with_one_column() {
        expect!(tokenize("create table table_name_1 (col integer);").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_name_1",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)].into_iter().collect())]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_list_of_columns() {
        expect!(tokenize("create table table_name_2 (col1 integer, col2 integer, col3 integer);").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_name_2",
                            vec![
                                ColumnTable::new("col1", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)].into_iter().collect()),
                                ColumnTable::new("col2", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)].into_iter().collect()),
                                ColumnTable::new("col3", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)].into_iter().collect())
                            ]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_varchar_column_type() {
        expect!(tokenize("create table table_1 (col_2 character(10));").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_1",
                            vec![ColumnTable::new("col_2", Type::VarChar(10), vec![Constraint::Nullable(true), Constraint::DefaultValue(None)].into_iter().collect())]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_default_value_constraint() {
        expect!(tokenize("create table table1 (col integer default 1);").and_then(|token| parse(token)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table1",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::DefaultValue(Some("1".to_owned())), Constraint::Nullable(true)].into_iter().collect())]
                        )
                    )
                )
            );
    }

    #[test]
    fn infer_type_for_primary_key_column() {
        expect!(tokenize("create table table_1 (col integer primary key);").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_1",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::PrimaryKey, Constraint::DefaultValue(None), Constraint::Nullable(false)].into_iter().collect())]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_primary_key_discard_default_value() {
        expect!(tokenize("create table table_1 (col integer primary key default 1);").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_1",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::PrimaryKey, Constraint::DefaultValue(None), Constraint::Nullable(false)].into_iter().collect())]
                        )
                    )
                )
            );
    }

    #[test]
    fn not_null_constraint() {
        expect!(tokenize("create table table_2 (col integer not null);").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_2",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::Nullable(false), Constraint::DefaultValue(Some("0".to_owned()))].into_iter().collect())]
                        )
                    )
                )
            );
    }

    #[test]
    fn not_null_with_default() {
        expect!(tokenize("create table tab3 (col1 integer not null default 4, col2 integer);").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "tab3",
                            vec![
                                ColumnTable::new("col1", Type::Integer, vec![Constraint::Nullable(false), Constraint::DefaultValue(Some("4".to_owned()))].into_iter().collect()),
                                ColumnTable::new("col2", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)].into_iter().collect())
                            ]
                        )
                    )
                )
            );
    }

    #[test]
    fn foreign_key_constraint() {
        expect!(tokenize("create table tab_4 (col1 integer primary key, col2 integer foreign key references table1(col));").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Create(
                        CreateTableQuery::new(
                            "tab_4",
                            vec![
                                ColumnTable::new("col1", Type::Integer, vec![Constraint::Nullable(false), Constraint::DefaultValue(None), Constraint::PrimaryKey].into_iter().collect()),
                                ColumnTable::new("col2", Type::Integer, vec![Constraint::Nullable(false), Constraint::DefaultValue(None), Constraint::ForeignKey("table1".to_owned(), "col".to_owned())].into_iter().collect())
                            ]
                        )
                    )
                )
            );
    }
}

#[cfg(test)]
mod parses_delete_statements {
    use expectest::prelude::be_ok;

    use sql::lexer::tokenize;
    use sql::parser::parse;
    use sql::ast::{Statement, DeleteQuery, Condition, CondArg};

    #[test]
    fn without_any_predicates() {
        expect!(tokenize("delete from table_name_1;").and_then(|tokens| parse(tokens)))
            .to(be_ok().value(Statement::Delete(DeleteQuery::new("table_name_1", None))));
    }

    #[test]
    fn with_column_const_predicate() {
        expect!(tokenize("delete from table_name_2 where col_1 = 5;").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Delete(
                        DeleteQuery::new(
                            "table_name_2",
                            Some(
                                Condition::equals(
                                    CondArg::column("col_1"),
                                    CondArg::num("5")
                                )
                            )
                        )
                    )
                )
            );
    }

    #[test]
    fn with_const_column_predicate() {
        expect!(tokenize("delete from table_name_3 where 'str' = col_2;").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Delete(
                        DeleteQuery::new(
                            "table_name_3",
                            Some(
                                Condition::equals(
                                    CondArg::str("str"),
                                    CondArg::column("col_2")
                                )
                            )
                        )
                    )
                )
            );
    }
}

#[cfg(test)]
mod parses_insert_statements {
    use expectest::prelude::be_ok;

    use sql::lexer::tokenize;
    use sql::parser::parse;
    use sql::ast::{InsertQuery, SelectQuery};
    use sql::ast::{Statement, Value, ValueSource};

    #[test]
    fn with_one_column() {
        expect!(tokenize("insert into table_name_1 values(10);").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Insert(
                        InsertQuery::new("table_name_1", vec![], ValueSource::Row(vec![Value::num("10")]))
                    )
                )
            );
    }

    #[test]
    fn with_list_of_columns() {
        expect!(tokenize("insert into table_name_2 values (10, 'string');").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Insert(
                        InsertQuery::new(
                            "table_name_2",
                            vec![],
                            ValueSource::Row(vec![Value::num("10"), Value::str("string")])
                        )
                    )
                )
            );
    }

    #[test]
    fn with_columns() {
        expect!(tokenize("insert into table_name_3 (col_1, col_2) values (10, 'string');").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Insert(
                        InsertQuery::new(
                            "table_name_3",
                            vec!["col_1", "col_2"],
                            ValueSource::Row(vec![Value::num("10"), Value::str("string")])
                        )
                    )
                )
            );
    }

    #[test]
    fn with_sub_select() {
        expect!(tokenize("insert into table_1 (col_1, col_2) select col_1, col_2 from table_1;").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Insert(
                        InsertQuery::new(
                            "table_1",
                            vec!["col_1", "col_2"],
                            ValueSource::SubQuery(SelectQuery::new("table_1", vec!["col_1", "col_2"], None))
                        )
                    )
                )
            );
    }
}

#[cfg(test)]
mod parse_select_statements {

    use expectest::prelude::be_ok;

    use sql::lexer::tokenize;
    use sql::parser::parse;
    use sql::ast::{Statement, SelectQuery, Condition, CondArg};

    #[test]
    fn without_predicates() {
        expect!(tokenize("select col_1 from table_name_1;").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Select(
                        SelectQuery::new("table_name_1", vec!["col_1"], None)
                    )
                )
            );
    }

    #[test]
    fn with_predicates() {
        expect!(tokenize("select col_2 from table_name_2 where col_2 = 10;").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Select(
                        SelectQuery::new(
                            "table_name_2",
                            vec!["col_2"],
                            Some(Condition::equals(CondArg::column("col_2"), CondArg::num("10")))
                        )
                    )
                )
            );
    }

    #[test]
    fn with_limit_predicate() {
        expect!(tokenize("select col_2 from table_name_2 where limit = 10;").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Select(
                        SelectQuery::new(
                            "table_name_2",
                            vec!["col_2"],
                            Some(Condition::equals(CondArg::Limit, CondArg::num("10")))
                        )
                    )
                )
            );
    }

    #[test]
    fn with_not_equal_predicate() {
        expect!(tokenize("select col_2 from table_1 where col_1 <> \'a\';").and_then(|tokens| parse(tokens)))
            .to(
                be_ok().value(
                    Statement::Select(
                        SelectQuery::new(
                            "table_1",
                            vec!["col_2"],
                            Some(Condition::not_equals(CondArg::column("col_1"), CondArg::str("a")))
                        )
                    )
                )
            );
    }
}
