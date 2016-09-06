#[cfg(test)]
mod parses_create_table_statement {
    use expectest::prelude::be_equal_to;

    use sql::lexer::Tokenizer;
    use sql::parser::Parser;
    use sql::parser::ast::Type::{Int, VarChar};
    use sql::parser::ast::Statement::Create;
    use sql::parser::ast::CreateTableQuery;
    use sql::parser::ast::table::Column;

    #[test]
    fn with_one_column() {
        expect!(Tokenizer::from("create table table_name_1 (col int);").tokenize().parse())
            .to(be_equal_to(Create(CreateTableQuery::new("table_name_1", vec![Column::new("col", Int)]))));
    }

    #[test]
    fn with_list_of_columns() {
        expect!(Tokenizer::from("create table table_name_2 (col1 int, col2 int, col3 int);").tokenize().parse())
            .to(
                be_equal_to(
                    Create(
                        CreateTableQuery::new(
                            "table_name_2",
                            vec![
                                Column::new("col1", Int),
                                Column::new("col2", Int),
                                Column::new("col3", Int)
                            ]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_varchar_column_type() {
        expect!(Tokenizer::from("create table table_1 (col_2 varchar(10));").tokenize().parse())
            .to(
                be_equal_to(
                    Create(
                        CreateTableQuery::new("table_1", vec![Column::new("col_2", VarChar(10))])
                    )
                )
            );
    }
}

#[cfg(test)]
mod parses_delete_statements {
    use expectest::prelude::be_equal_to;

    use sql::lexer::Tokenizer;
    use sql::parser::Parser;
    use sql::parser::ast::Statement::Delete;
    use sql::parser::ast::DeleteQuery;
    use sql::parser::ast::Condition;
    use sql::parser::ast::CondArg;

    #[test]
    fn without_any_predicates() {
        expect!(Tokenizer::from("delete from table_name_1;").tokenize().parse())
            .to(be_equal_to(Delete(DeleteQuery::new("table_name_1", None))));
    }

    #[test]
    fn with_column_const_predicate() {
        expect!(Tokenizer::from("delete from table_name_2 where col_1 = 5;").tokenize().parse())
            .to(
                be_equal_to(
                    Delete(
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
        expect!(Tokenizer::from("delete from table_name_3 where 'str' = col_2;").tokenize().parse())
            .to(
                be_equal_to(
                    Delete(
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

    use sql::lexer::Tokenizer;
    use sql::parser::Parser;
    use sql::parser::ast::InsertQuery;
    use sql::parser::ast::Statement::Insert;
    use sql::parser::ast::Value;

    #[test]
    fn with_one_column() {
        expect!(Tokenizer::from("insert into table_name_1 values(10);").tokenize().parse())
            .to(
                be_equal_to(
                    Insert(
                        InsertQuery::new("table_name_1", vec![], vec![Value::num("10")])
                    )
                )
            );
    }

    #[test]
    fn with_list_of_columns() {
        expect!(Tokenizer::from("insert into table_name_2 values (10, 'string');").tokenize().parse())
            .to(
                be_equal_to(
                    Insert(
                        InsertQuery::new(
                            "table_name_2",
                            vec![],
                            vec![Value::num("10"), Value::str("string")]
                        )
                    )
                )
            );
    }

    #[test]
    fn with_columns() {
        expect!(Tokenizer::from("insert into table_name_3 (col_1, col_2) values (10, 'string');").tokenize().parse())
            .to(
                be_equal_to(
                    Insert(
                        InsertQuery::new(
                            "table_name_3",
                            vec!["col_1", "col_2"],
                            vec![Value::num("10"), Value::str("string")]
                        )
                    )
                )
            );
    }
}

#[cfg(test)]
mod parse_select_statements {

    use expectest::prelude::be_equal_to;

    use sql::lexer::Tokenizer;
    use sql::parser::Parser;
    use sql::parser::ast::Statement::Select;
    use sql::parser::ast::SelectQuery;
    use sql::parser::ast::Condition;
    use sql::parser::ast::CondArg;

    #[test]
    fn without_predicates() {
        expect!(Tokenizer::from("select col_1 from table_name_1").tokenize().parse())
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
        expect!(Tokenizer::from("select col_2 from table_name_2 where col_2 = 10;").tokenize().parse())
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
        expect!(Tokenizer::from("select col_2 from table_name_2 where limit = 10;").tokenize().parse())
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
        expect!(Tokenizer::from("select col_2 from table_1 where col_1 <> \'a\';").tokenize().parse())
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
