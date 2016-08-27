#[cfg(test)]
mod create_table_statements {
    mod parses_create_table_statement {
        use expectest::prelude::be_ok;

        use sql::lexer::Token::{IdentT, Semicolon, LeftParenthesis, RightParenthesis, Comma};
        use sql::parser::Parser;
        use sql::parser::ast::Type::Int;
        use sql::parser::ast::Node::{Table, TableColumn, Create};

        #[test]
        fn with_one_column() {
            let tokens = vec![
            IdentT("create".to_owned()),
            IdentT("table".to_owned()),
            IdentT("table_name".to_owned()),
            LeftParenthesis,
            IdentT("col".to_owned()),
            IdentT("int".to_owned()),
            RightParenthesis,
            Semicolon
        ];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Create(
                        Box::new(Table(
                            "table_name".to_owned(),
                            vec![TableColumn("col".to_owned(), Int, None)]
                        ))
                    )
                ));
        }

        #[test]
        fn with_list_of_columns() {
            let tokens = vec![
            IdentT("create".to_owned()),
            IdentT("table".to_owned()),
            IdentT("table_name".to_owned()),
            LeftParenthesis,
            IdentT("col1".to_owned()),
            IdentT("int".to_owned()),
            Comma,
            IdentT("col2".to_owned()),
            IdentT("int".to_owned()),
            Comma,
            IdentT("col3".to_owned()),
            IdentT("int".to_owned()),
            RightParenthesis,
            Semicolon
        ];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Create(
                        Box::new(Table(
                            "table_name".to_owned(),
                            vec![
                            TableColumn("col1".to_owned(), Int, None),
                            TableColumn("col2".to_owned(), Int, None),
                            TableColumn("col3".to_owned(), Int, None)
                        ]
                        ))
                    )
                ));
        }
    }

    mod does_not_parse_create_table_statement {
        use expectest::prelude::be_err;

        use sql::lexer::Token::{IdentT, Semicolon, LeftParenthesis, RightParenthesis};
        use sql::parser::Parser;

        #[test]
        fn without_comma_in_column_list() {
            let tokens = vec![
            IdentT("create".to_owned()),
            IdentT("table".to_owned()),
            IdentT("table_name".to_owned()),
            LeftParenthesis,
            IdentT("col1".to_owned()),
            IdentT("int".to_owned()),
            IdentT("col2".to_owned()),
            IdentT("int".to_owned()),
            RightParenthesis,
            Semicolon
        ];


            expect!(tokens.parse())
                .to(be_err().value("error: expected <,> found <col2>".to_owned()));
        }

        #[test]
        fn without_open_parenthesis() {
            let tokens = vec![
            IdentT("create".to_owned()),
            IdentT("table".to_owned()),
            IdentT("table_name".to_owned()),
            IdentT("col".to_owned()),
            IdentT("int".to_owned()),
            RightParenthesis,
            Semicolon
        ];

            expect!(tokens.parse())
                .to(be_err().value("error: expected <(> found <col>".to_owned()));
        }

        #[test]
        fn without_closing_parenthesis() {
            let tokens = vec![
            IdentT("create".to_owned()),
            IdentT("table".to_owned()),
            IdentT("table_name".to_owned()),
            LeftParenthesis,
            IdentT("col".to_owned()),
            IdentT("int".to_owned()),
            Semicolon
        ];

            expect!(tokens.parse())
                .to(be_err().value("error: expected <)> found <;>".to_owned()));
        }

        #[test]
        fn without_semicolon() {
            let tokens = vec![
            IdentT("create".to_owned()),
            IdentT("table".to_owned()),
            IdentT("table_name".to_owned()),
            LeftParenthesis,
            IdentT("col".to_owned()),
            IdentT("int".to_owned()),
            RightParenthesis
        ];

            expect!(tokens.parse())
                .to(be_err().value("error: expected <;>"));
        }

        #[test]
        fn found_left_parenthesis() {
            let tokens = vec![
            IdentT("create".to_owned()),
            IdentT("table".to_owned()),
            LeftParenthesis,
            IdentT("col".to_owned()),
            IdentT("int".to_owned()),
            RightParenthesis
        ];

            expect!(tokens.parse())
                .to(be_err().value("error: expected <table name> found <(>".to_owned()));
        }

        #[test]
        fn found_right_parenthesis() {
            let tokens = vec![
            IdentT("create".to_owned()),
            IdentT("table".to_owned()),
            RightParenthesis
        ];

            expect!(tokens.parse())
                .to(be_err().value("error: expected <table name> found <)>".to_owned()));
        }
    }
}

#[cfg(test)]
mod delete_statements {
    use expectest::prelude::be_ok;

    use sql::parser::Parser;
    use sql::parser::ast::Node::{Delete, From, Where, Id, NumberC};
    use sql::parser::ast::Condition::Eq;
    use sql::lexer::Token::{IdentT, Semicolon, EqualSign, NumberT};

    #[test]
    fn it_parses_delete_statement() {
        let tokens = vec![
            IdentT("delete".to_owned()),
            IdentT("from".to_owned()),
            IdentT("table_name".to_owned()),
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_ok().value(
                Delete(
                    Box::new(From("table_name".to_owned())),
                    Box::new(Where(None))
                )
            ));
    }

    #[test]
    fn it_parses_delete_statement_with_predicate() {
        let tokens = vec![
            IdentT("delete".to_owned()),
            IdentT("from".to_owned()),
            IdentT("table_name".to_owned()),
            IdentT("where".to_owned()),
            IdentT("col".to_owned()),
            EqualSign,
            NumberT("5".to_owned()),
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_ok().value(
                Delete(
                    Box::new(From("table_name".to_owned())),
                    Box::new(Where(Some(
                        Eq(
                            Box::new(Id("col".to_owned())),
                            Box::new(NumberC("5".to_owned()))
                        )
                    )))
                )
            ));
    }
}

#[cfg(test)]
mod insert_statements {
    use expectest::prelude::be_ok;

    use sql::parser::Parser;
    use sql::lexer::Token::{IdentT, LeftParenthesis, NumberT, RightParenthesis, Semicolon, Comma, StringT};
    use sql::parser::ast::Node::{Insert, Table, Values, NumberC, Column, StringC};

    #[test]
    fn it_parse_insert_statement_with_one_column() {
        let tokens = vec![
            IdentT("insert".to_owned()),
            IdentT("into".to_owned()),
            IdentT("table_name".to_owned()),
            IdentT("values".to_owned()),
            LeftParenthesis,
            NumberT("10".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_ok().value(
                Insert(
                    Box::new(Table("table_name".to_owned(), vec![])),
                    Box::new(Values(vec![NumberC("10".to_owned())]))
                )
            ));
    }

    #[test]
    fn it_parses_insert_statement_with_list_of_columns() {
        let tokens = vec![
            IdentT("insert".to_owned()),
            IdentT("into".to_owned()),
            IdentT("table_name".to_owned()),
            IdentT("values".to_owned()),
            LeftParenthesis,
            NumberT("10".to_owned()),
            Comma,
            StringT("string".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_ok().value(
                Insert(
                    Box::new(Table("table_name".to_owned(), vec![])),
                    Box::new(Values(vec![NumberC("10".to_owned()), StringC("string".to_owned())]))
                )
            ));
    }

    #[test]
    fn it_parses_insert_statement_with_columns() {
        let tokens = vec![
            IdentT("insert".to_owned()),
            IdentT("into".to_owned()),
            IdentT("table_name".to_owned()),
            LeftParenthesis,
            IdentT("col1".to_owned()),
            Comma,
            IdentT("col2".to_owned()),
            RightParenthesis,
            IdentT("values".to_owned()),
            LeftParenthesis,
            NumberT("10".to_owned()),
            Comma,
            StringT("string".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_ok().value(
                Insert(
                    Box::new(Table("table_name".to_owned(), vec![Column("col1".to_owned()), Column("col2".to_owned())])),
                    Box::new(Values(vec![NumberC("10".to_owned()), StringC("string".to_owned())]))
                )
            ));
    }
}
