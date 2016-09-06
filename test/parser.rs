#[cfg(test)]
mod parses_create_table_statement {
    use expectest::prelude::be_equal_to;

    use sql::lexer::Token::{Ident, Semicolon, LeftParenthesis, RightParenthesis, Comma};
    use sql::parser::Parser;
    use sql::parser::ast::Type::Int;
    use sql::parser::ast::Statement::Create;
    use sql::parser::ast::CreateTableQuery;
    use sql::parser::ast::table::Column;

    #[test]
    fn with_one_column() {
        let tokens = vec![
            Ident("create".to_owned()),
            Ident("table".to_owned()),
            Ident("table_name_1".to_owned()),
            LeftParenthesis,
            Ident("col".to_owned()),
            Ident("int".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_equal_to(Create(CreateTableQuery::new("table_name_1", vec![Column::new("col", Int)]))));
    }

    #[test]
    fn with_list_of_columns() {
        let tokens = vec![
            Ident("create".to_owned()),
            Ident("table".to_owned()),
            Ident("table_name_2".to_owned()),
            LeftParenthesis,
            Ident("col1".to_owned()),
            Ident("int".to_owned()),
            Comma,
            Ident("col2".to_owned()),
            Ident("int".to_owned()),
            Comma,
            Ident("col3".to_owned()),
            Ident("int".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
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

    use sql::lexer::Token::{Ident, Semicolon, EqualSign, NumericConstant, CharactersConstant};
    use sql::parser::Parser;
    use sql::parser::ast::Statement::Delete;
    use sql::parser::ast::DeleteQuery;
    use sql::parser::ast::Condition::Eq;
    use sql::parser::ast::PredicateArgument::{ColumnName, StringConstant, NumberConstant};

    #[test]
    fn without_any_predicates() {
        let tokens = vec![
            Ident("delete".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name_1".to_owned()),
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_equal_to(Delete(DeleteQuery::new("table_name_1", None))));
    }

    #[test]
    fn with_column_const_predicate() {
        let tokens = vec![
            Ident("delete".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name_2".to_owned()),
            Ident("where".to_owned()),
            Ident("col_1".to_owned()),
            EqualSign,
            NumericConstant("5".to_owned()),
            Semicolon
        ];

        expect!(tokens.parse())
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
        let tokens = vec![
            Ident("delete".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name_3".to_owned()),
            Ident("where".to_owned()),
            CharactersConstant("str".to_owned()),
            EqualSign,
            Ident("col_2".to_owned()),
            Semicolon
        ];

        expect!(tokens.parse())
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

    use sql::lexer::Token::{Ident, LeftParenthesis, NumericConstant, RightParenthesis, Semicolon, Comma, CharactersConstant};
    use sql::parser::Parser;
    use sql::parser::ast::InsertQuery;
    use sql::parser::ast::Statement::Insert;
    use sql::parser::ast::ValueParameter::{NumberConst, StringConst};

    #[test]
    fn with_one_column() {
        let tokens = vec![
            Ident("insert".to_owned()),
            Ident("into".to_owned()),
            Ident("table_name_1".to_owned()),
            Ident("values".to_owned()),
            LeftParenthesis,
            NumericConstant("10".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
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
        let tokens = vec![
                Ident("insert".to_owned()),
                Ident("into".to_owned()),
                Ident("table_name_2".to_owned()),
                Ident("values".to_owned()),
                LeftParenthesis,
                NumericConstant("10".to_owned()),
                Comma,
                CharactersConstant("string".to_owned()),
                RightParenthesis,
                Semicolon
            ];

        expect!(tokens.parse())
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
        let tokens = vec![
                Ident("insert".to_owned()),
                Ident("into".to_owned()),
                Ident("table_name_3".to_owned()),
                LeftParenthesis,
                Ident("col_1".to_owned()),
                Comma,
                Ident("col_2".to_owned()),
                RightParenthesis,
                Ident("values".to_owned()),
                LeftParenthesis,
                NumericConstant("10".to_owned()),
                Comma,
                CharactersConstant("string".to_owned()),
                RightParenthesis,
                Semicolon
            ];

        expect!(tokens.parse())
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

    use sql::lexer::Token::{Ident, EqualSign, NumericConstant};
    use sql::parser::Parser;
    use sql::parser::ast::Statement::Select;
    use sql::parser::ast::SelectQuery;
    use sql::parser::ast::Condition::Eq;
    use sql::parser::ast::PredicateArgument::{ColumnName, NumberConstant, Limit};

    #[test]
    fn without_predicates() {
        let tokens = vec![
            Ident("select".to_owned()),
            Ident("col_1".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name_1".to_owned())
        ];

        expect!(tokens.parse())
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
        let tokens = vec![
            Ident("select".to_owned()),
            Ident("col_2".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name_2".to_owned()),
            Ident("where".to_owned()),
            Ident("col_2".to_owned()),
            EqualSign,
            NumericConstant("10".to_owned())
        ];

        expect!(tokens.parse())
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
        let tokens = vec![
            Ident("select".to_owned()),
            Ident("col_2".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name_2".to_owned()),
            Ident("where".to_owned()),
            Ident("limit".to_owned()),
            EqualSign,
            NumericConstant("10".to_owned())
        ];

        expect!(tokens.parse())
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
