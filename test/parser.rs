#[cfg(test)]
mod parses_create_table_statement {
    use expectest::prelude::be_equal_to;

    use sql::lexer::{Tokenizer, IntoTokenizer};
    use sql::parser::{QueryParser, IntoQueryParser};
    use sql::ast::{Type, Statement, Constraint, CreateTableQuery, ColumnTable};

    #[test]
    fn with_one_column() {
        expect!(String::from("create table table_name_1 (col integer);").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_name_1",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)])]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_list_of_columns() {
        expect!(String::from("create table table_name_2 (col1 integer, col2 integer, col3 integer);").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_name_2",
                            vec![
                                ColumnTable::new("col1", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)]),
                                ColumnTable::new("col2", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)]),
                                ColumnTable::new("col3", Type::Integer, vec![Constraint::Nullable(true), Constraint::DefaultValue(None)])
                            ]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_varchar_column_type() {
        expect!(String::from("create table table_1 (col_2 character(10));").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_1",
                            vec![ColumnTable::new("col_2", Type::VarChar(10), vec![Constraint::Nullable(true), Constraint::DefaultValue(None)])]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_default_value_constraint() {
        expect!(String::from("create table table1 (col integer default 1);").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table1",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::DefaultValue(Some("1".to_owned())), Constraint::Nullable(true)])]
                        )
                    )
                )
            );
    }

    #[test]
    fn infer_type_for_primary_key_column() {
        expect!(String::from("create table table_1 (col integer primary key);").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_1",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::PrimeryKey, Constraint::DefaultValue(None)])]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_primary_key_discard_default_value() {
        expect!(String::from("create table table_1 (col integer primary key default 1);").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_1",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::PrimeryKey, Constraint::DefaultValue(None)])]
                        )
                    )
                )
            );
    }

    #[test]
    #[ignore]
    fn not_null_constraint() {
        expect!(String::from("create table table_2 (col integer not null);").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Statement::Create(
                        CreateTableQuery::new(
                            "table_2",
                            vec![ColumnTable::new("col", Type::Integer, vec![Constraint::Nullable(false), Constraint::DefaultValue(Some("0".to_owned()))])]
                        )
                    )
                )
            );
    }
}

#[cfg(test)]
mod parses_delete_statements {
    use expectest::prelude::be_equal_to;

    use sql::lexer::{Tokenizer, IntoTokenizer};
    use sql::parser::{QueryParser, IntoQueryParser};
    use sql::ast::{Statement, DeleteQuery, Condition, CondArg};

    #[test]
    fn without_any_predicates() {
        expect!(String::from("delete from table_name_1;").into_tokenizer().tokenize().into_parser().parse())
            .to(be_equal_to(Statement::Delete(DeleteQuery::new("table_name_1", None))));
    }

    #[test]
    fn with_column_const_predicate() {
        expect!(String::from("delete from table_name_2 where col_1 = 5;").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
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
        expect!(String::from("delete from table_name_3 where 'str' = col_2;").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
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
    use expectest::prelude::be_equal_to;

    use sql::lexer::{Tokenizer, IntoTokenizer};
    use sql::parser::{QueryParser, IntoQueryParser};
    use sql::ast::{InsertQuery, SelectQuery};
    use sql::ast::Statement::Insert;
    use sql::ast::Value;
    use sql::ast::ValueSource;

    #[test]
    fn with_one_column() {
        expect!(String::from("insert into table_name_1 values(10);").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Insert(
                        InsertQuery::new("table_name_1", vec![], ValueSource::Row(vec![Value::num("10")]))
                    )
                )
            );
    }

    #[test]
    fn with_list_of_columns() {
        expect!(String::from("insert into table_name_2 values (10, 'string');").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Insert(
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
        expect!(String::from("insert into table_name_3 (col_1, col_2) values (10, 'string');").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Insert(
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
        expect!(String::from("insert into table_1 (col_1, col_2) select col_1, col_2 from table_1;").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Insert(
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

    use expectest::prelude::be_equal_to;

    use sql::lexer::{Tokenizer, IntoTokenizer};
    use sql::parser::{QueryParser, IntoQueryParser};
    use sql::ast::Statement::Select;
    use sql::ast::SelectQuery;
    use sql::ast::Condition;
    use sql::ast::CondArg;

    #[test]
    fn without_predicates() {
        expect!(String::from("select col_1 from table_name_1;").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Select(
                        SelectQuery::new("table_name_1", vec!["col_1"], None)
                    )
                )
            );
    }

    #[test]
    fn with_predicates() {
        expect!(String::from("select col_2 from table_name_2 where col_2 = 10;").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Select(
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
        expect!(String::from("select col_2 from table_name_2 where limit = 10;").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Select(
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
        expect!(String::from("select col_2 from table_1 where col_1 <> \'a\';").into_tokenizer().tokenize().into_parser().parse())
            .to(
                be_equal_to(
                    Select(
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
