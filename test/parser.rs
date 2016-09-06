#[cfg(test)]
mod parses_create_table_statement {
    use expectest::prelude::be_equal_to;

    use sql::lexer::Tokenizer;
    use sql::parser::Parser;
    use sql::parser::ast::Type::Int;
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
}

#[cfg(test)]
mod parses_delete_statements {
    use expectest::prelude::be_equal_to;

    use sql::lexer::Tokenizer;
    use sql::parser::Parser;
    use sql::parser::ast::Statement::Delete;
    use sql::parser::ast::DeleteQuery;
    use sql::parser::ast::Condition::Eq;
    use sql::parser::ast::PredicateArgument::{ColumnName, StringConstant, NumberConstant};

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
                                Eq(
                                    ColumnName("col_1".to_owned()),
                                    NumberConstant("5".to_owned())
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
                                Eq(
                                    StringConstant("str".to_owned()),
                                    ColumnName("col_2".to_owned())
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
    use sql::parser::ast::ValueParameter::{NumberConst, StringConst};

    #[test]
    fn with_one_column() {
        expect!(Tokenizer::from("insert into table_name_1 values(10);").tokenize().parse())
            .to(
                be_equal_to(
                    Insert(
                        InsertQuery::new("table_name_1", vec![], vec![NumberConst("10".to_owned())])
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
                            vec![NumberConst("10".to_owned()), StringConst("string".to_owned())]
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
                            vec![NumberConst("10".to_owned()), StringConst("string".to_owned())]
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
    use sql::parser::ast::Condition::Eq;
    use sql::parser::ast::PredicateArgument::{ColumnName, NumberConstant, Limit};

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
                            Some(
                                Eq(ColumnName("col_2".to_owned()), NumberConstant("10".to_owned()))
                            )
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
                            Some(
                                Eq(Limit, NumberConstant("10".to_owned()))
                            )
                        )
                    )
                )
            );
    }
}
