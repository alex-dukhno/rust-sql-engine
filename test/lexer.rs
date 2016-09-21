use expectest::prelude::be_ok;

use sql::lexer::{Token, tokenize};

#[test]
fn emits_none_when_given_an_empty_string() {
    expect!(tokenize(""))
        .to(be_ok().value(vec![]));
}

#[test]
fn emits_identifier_token_when_given_a_single_word_string() {
    expect!(tokenize("word"))
        .to(be_ok().value(vec![Token::ident("word")]));
}

#[test]
fn emits_identifiers_when_given_string_of_words() {
    expect!(tokenize("this is a sentence"))
        .to(
            be_ok().value(
                vec![
                    Token::ident("this"),
                    Token::ident("is"),
                    Token::ident("a"),
                    Token::ident("sentence")
                ]
            )
        );
}

#[test]
fn emits_number_token_when_given_number() {
    expect!(tokenize("5"))
        .to(be_ok().value(vec![Token::number("5")]));
}

#[test]
fn escapes_new_line_chars() {
    expect!(tokenize("\nword"))
        .to(be_ok().value(vec![Token::ident("word")]));
}

#[test]
fn escapes_tabs() {
    expect!(tokenize("\tword"))
        .to(be_ok().value(vec![Token::ident("word")]));
}

#[test]
fn case_insensitive() {
    expect!(tokenize("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
        .to(be_ok().value(vec![Token::ident("abcdefghijklmnopqrstuvwxyz")]));
}

#[cfg(test)]
mod single_quotes {
    use expectest::prelude::be_ok;

    use sql::lexer::{Token, tokenize};

    #[test]
    fn inside_string_token() {
        expect!(tokenize("'str''str'"))
            .to(be_ok().value(vec![Token::string("str'str")]));
    }

    #[test]
    fn at_the_end() {
        expect!(tokenize("'str'''"))
            .to(be_ok().value(vec![Token::string("str'")]));
    }

    #[test]
    fn at_the_begining() {
        expect!(tokenize("'''str'"))
            .to(be_ok().value(vec![Token::string("'str")]));
    }

    #[test]
    fn everywhere() {
        expect!(tokenize("'''str''str'''"))
            .to(be_ok().value(vec![Token::string("'str'str'")]));
    }

    #[test]
    fn emits_string_when_only_open_signle_quote() {
        expect!(tokenize("'str"))
            .to(be_ok().value(vec![Token::string("str")]));
    }
}

#[cfg(test)]
mod cmp_tokens {
    use expectest::prelude::be_ok;

    use sql::lexer::{Token, tokenize};

    #[test]
    fn equal_sign() {
        expect!(tokenize("="))
            .to(be_ok().value(vec![Token::EqualSign]));
    }

    #[test]
    fn not_equal_sign_angle_brackets() {
        expect!(tokenize("<>"))
            .to(be_ok().value(vec![Token::NotEqualSign]));
    }

    #[test]
    fn not_equal_sign_exclamation_mark_equal_sign() {
        expect!(tokenize("!="))
            .to(be_ok().value(vec![Token::NotEqualSign]));
    }

    #[test]
    fn less_then_sign() {
        expect!(tokenize("<"))
            .to(be_ok().value(vec![Token::Less]));
    }

    #[test]
    fn less_or_equal_sign() {
        expect!(tokenize("<="))
            .to(be_ok().value(vec![Token::LessEqual]));
    }

    #[test]
    fn greater_then_sign() {
        expect!(tokenize(">"))
            .to(be_ok().value(vec![Token::Greater]));
    }

    #[test]
    fn greate_or_equal_sign() {
        expect!(tokenize(">="))
            .to(be_ok().value(vec![Token::GreaterEqual]));
    }
}

#[cfg(test)]
mod sql_query {
    use expectest::prelude::be_ok;

    use sql::lexer::{Token, tokenize};

    #[test]
    fn tokenize_insert_query_numeric_value() {
        expect!(tokenize("insert into table_name values(1);"))
            .to(
                be_ok().value(
                    vec![
                        Token::Insert,
                        Token::Into,
                        Token::ident("table_name"),
                        Token::Values,
                        Token::from("("),
                        Token::number("1"),
                        Token::from(")"),
                        Token::from(";")
                    ]
                )
            );
    }

    #[test]
    fn tokenize_insert_query_varchar_value() {
        expect!(tokenize("insert into table_name values('string');"))
            .to(
                be_ok().value(
                    vec![
                        Token::Insert,
                        Token::Into,
                        Token::ident("table_name"),
                        Token::Values,
                        Token::from("("),
                        Token::string("string"),
                        Token::from(")"),
                        Token::from(";")
                    ]
                )
            );
    }

    #[test]
    fn tokenize_select_with_not_equal_predicate() {
        expect!(tokenize("select col from table_1 where col <> 5;"))
            .to(
                be_ok().value(
                    vec![
                        Token::Select,
                        Token::ident("col"),
                        Token::From,
                        Token::ident("table_1"),
                        Token::Where,
                        Token::ident("col"),
                        Token::from("<>"),
                        Token::number("5"),
                        Token::from(";")
                    ]
                )
            );
    }

    #[test]
    fn tokenize_create_table_with_primary_key() {
        expect!(tokenize("create table tab1 (col1 char(3) primary key);"))
            .to(
                be_ok().value(
                    vec![
                        Token::Create,
                        Token::Table,
                        Token::ident("tab1"),
                        Token::LParent,
                        Token::ident("col1"),
                        Token::Character,
                        Token::LParent,
                        Token::number("3"),
                        Token::RParent,
                        Token::Primary,
                        Token::Key,
                        Token::RParent,
                        Token::Semicolon
                    ]
                )
            );
    }

    #[test]
    fn tokenize_create_table_with_not_null() {
        expect!(tokenize("create table tab2 (col integer not null);"))
            .to(
                be_ok().value(
                    vec![
                        Token::Create,
                        Token::Table,
                        Token::ident("tab2"),
                        Token::LParent,
                        Token::ident("col"),
                        Token::Int,
                        Token::Not,
                        Token::Null,
                        Token::RParent,
                        Token::Semicolon
                    ]
                )
            );
    }

    #[test]
    fn tokenize_create_table_with_foreign_key() {
        expect!(tokenize("create table tab_4 (col1 integer primary key, col2 integer foreign key references table1(col));"))
            .to(
                be_ok().value(
                    vec![
                        Token::Create,
                        Token::Table,
                        Token::ident("tab_4"),
                        Token::LParent,
                        Token::ident("col1"),
                        Token::Int,
                        Token::Primary,
                        Token::Key,
                        Token::Comma,
                        Token::ident("col2"),
                        Token::Int,
                        Token::Foreign,
                        Token::Key,
                        Token::References,
                        Token::ident("table1"),
                        Token::LParent,
                        Token::ident("col"),
                        Token::RParent,
                        Token::RParent,
                        Token::Semicolon
                    ]
                )
            );
    }
}
