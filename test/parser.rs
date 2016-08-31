#[cfg(test)]
mod parses_create_table_statement {
    use expectest::prelude::be_equal_to;

    use sql::lexer::Token::{Ident, Semicolon, LeftParenthesis, RightParenthesis, Comma};
    use sql::parser::Parser;
    use sql::parser::ast::Type::Int;
    use sql::parser::ast::Node::{Table, TableColumn, Create};

    #[test]
    fn with_one_column() {
        let tokens = vec![
            Ident("create".to_owned()),
            Ident("table".to_owned()),
            Ident("table_name".to_owned()),
            LeftParenthesis,
            Ident("col".to_owned()),
            Ident("int".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_equal_to(
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
            Ident("create".to_owned()),
            Ident("table".to_owned()),
            Ident("table_name".to_owned()),
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
            .to(be_equal_to(
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

#[cfg(test)]
mod parses_delete_statements {
    use expectest::prelude::be_equal_to;

    use sql::parser::Parser;
    use sql::parser::ast::Node::{Delete, From, Where, Id, Numeric};
    use sql::parser::ast::Condition::Eq;
    use sql::lexer::Token::{Ident, Semicolon, EqualSign, NumericConstant};

    #[test]
    fn without_any_predicates() {
        let tokens = vec![
            Ident("delete".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name".to_owned()),
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_equal_to(
                Delete(
                    Box::new(From("table_name".to_owned())),
                    Box::new(Where(None))
                )
            ));
    }

    #[test]
    fn with_predicate() {
        let tokens = vec![
            Ident("delete".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name".to_owned()),
            Ident("where".to_owned()),
            Ident("col".to_owned()),
            EqualSign,
            NumericConstant("5".to_owned()),
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_equal_to(
                Delete(
                    Box::new(From("table_name".to_owned())),
                    Box::new(Where(Some(
                        Eq(
                            Box::new(Id("col".to_owned())),
                            Box::new(Numeric("5".to_owned()))
                        )
                    )))
                )
            ));
    }
}

#[cfg(test)]
mod parses_insert_statements {
    use expectest::prelude::be_equal_to;

    use sql::parser::Parser;
    use sql::lexer::Token::{Ident, LeftParenthesis, NumericConstant, RightParenthesis, Semicolon, Comma, CharactersConstant};
    use sql::parser::ast::Node::{Insert, Table, Values, Numeric, Column, CharSequence};

    #[test]
    fn with_one_column() {
        let tokens = vec![
            Ident("insert".to_owned()),
            Ident("into".to_owned()),
            Ident("table_name".to_owned()),
            Ident("values".to_owned()),
            LeftParenthesis,
            NumericConstant("10".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_equal_to(
                Insert(
                    Box::new(Table("table_name".to_owned(), vec![])),
                    Box::new(Values(vec![Numeric("10".to_owned())]))
                )
            ));
    }

    #[test]
    fn with_list_of_columns() {
        let tokens = vec![
            Ident("insert".to_owned()),
            Ident("into".to_owned()),
            Ident("table_name".to_owned()),
            Ident("values".to_owned()),
            LeftParenthesis,
            NumericConstant("10".to_owned()),
            Comma,
            CharactersConstant("string".to_owned()),
            RightParenthesis,
            Semicolon
        ];

        expect!(tokens.parse())
            .to(be_equal_to(
                Insert(
                    Box::new(Table("table_name".to_owned(), vec![])),
                    Box::new(Values(vec![Numeric("10".to_owned()), CharSequence("string".to_owned())]))
                )
            ));
    }

    #[test]
    fn with_columns() {
        let tokens = vec![
            Ident("insert".to_owned()),
            Ident("into".to_owned()),
            Ident("table_name".to_owned()),
            LeftParenthesis,
            Ident("col1".to_owned()),
            Comma,
            Ident("col2".to_owned()),
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
            .to(be_equal_to(
                Insert(
                    Box::new(Table("table_name".to_owned(), vec![Column("col1".to_owned()), Column("col2".to_owned())])),
                    Box::new(Values(vec![Numeric("10".to_owned()), CharSequence("string".to_owned())]))
                )
            ));
    }
}

#[cfg(test)]
mod parse_select_statements {

    use expectest::prelude::be_equal_to;

    use sql::lexer::Token::Ident;
    use sql::parser::Parser;
    use sql::parser::ast::Node::{Select, Table, Column};

    #[test]
    fn without_predicates() {
        let tokens = vec![
            Ident("select".to_owned()),
            Ident("col".to_owned()),
            Ident("from".to_owned()),
            Ident("table_name".to_owned())
        ];

        expect!(tokens.parse())
            .to(be_equal_to(
                Select(
                    Box::new(Table("table_name".to_owned(), vec![])),
                    vec![Column("col".to_owned())]
                )
            ));
    }
}
