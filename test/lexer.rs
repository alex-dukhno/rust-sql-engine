use expectest::prelude::be_equal_to;

use sql::lexer::{Tokenizer, Token, IntoTokenizer};

#[test]
fn emits_none_when_given_an_empty_string() {
    expect!(String::from("").into_tokenizer().tokenize())
        .to(be_equal_to(vec![]));
}

#[test]
fn emits_identifier_token_when_given_a_single_word_string() {
    expect!(String::from("word").into_tokenizer().tokenize())
        .to(be_equal_to(vec![Token::ident("word")]));
}

#[test]
fn emits_identifiers_when_given_string_of_words() {
    expect!(String::from("this is a sentence").into_tokenizer().tokenize())
        .to(be_equal_to(
            vec![
                Token::ident("this"),
                Token::ident("is"),
                Token::ident("a"),
                Token::ident("sentence")
            ]
        ));
}

#[test]
fn emits_number_token_when_given_number() {
    expect!(String::from("5").into_tokenizer().tokenize())
        .to(be_equal_to(vec![Token::number("5")]));
}

#[test]
fn escapes_new_line_chars() {
    expect!(String::from("\nword").into_tokenizer().tokenize())
        .to(be_equal_to(vec![Token::ident("word")]));
}

#[test]
fn escapes_tabs() {
    expect!(String::from("\tword").into_tokenizer().tokenize())
        .to(be_equal_to(vec![Token::ident("word")]));
}

#[test]
fn case_insensitive() {
    expect!(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ").into_tokenizer().tokenize())
        .to(be_equal_to(vec![Token::ident("abcdefghijklmnopqrstuvwxyz")]));
}

#[cfg(test)]
mod single_quotes {
    use expectest::prelude::be_equal_to;

    use sql::lexer::{Tokenizer, Token, IntoTokenizer};

    #[test]
    fn inside_string_token() {
        expect!(String::from("'str''str'").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::string("str'str")]));
    }

    #[test]
    fn at_the_end() {
        expect!(String::from("'str'''").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::string("str'")]));
    }

    #[test]
    fn at_the_begining() {
        expect!(String::from("'''str'").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::string("'str")]));
    }

    #[test]
    fn everywhere() {
        expect!(String::from("'''str''str'''").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::string("'str'str'")]));
    }

    #[test]
    fn emits_string_when_only_open_signle_quote() {
        expect!(String::from("'str").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::string("str")]));
    }
}

#[cfg(test)]
mod cmp_tokens {
    use expectest::prelude::be_equal_to;

    use sql::lexer::{Tokenizer, Token, IntoTokenizer};

    #[test]
    fn equal_sign() {
        expect!(String::from("=").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::EqualSign]));
    }

    #[test]
    fn not_equal_sign_angle_brackets() {
        expect!(String::from("<>").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::NotEqualSign]));
    }

    #[test]
    fn not_equal_sign_exclamation_mark_equal_sign() {
        expect!(String::from("!=").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::NotEqualSign]));
    }

    #[test]
    fn less_then_sign() {
        expect!(String::from("<").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::Less]));
    }

    #[test]
    fn less_or_equal_sign() {
        expect!(String::from("<=").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::LessEqual]));
    }

    #[test]
    fn greater_then_sign() {
        expect!(String::from(">").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::Greater]));
    }

    #[test]
    fn greate_or_equal_sign() {
        expect!(String::from(">=").into_tokenizer().tokenize())
            .to(be_equal_to(vec![Token::GreaterEqual]));
    }
}

#[cfg(test)]
mod sql_query {
    use expectest::prelude::be_equal_to;

    use sql::lexer::{Tokenizer, Token, IntoTokenizer};

    #[test]
    fn tokenize_insert_query_numeric_value() {
        expect!(String::from("insert into table_name values(1);").into_tokenizer().tokenize())
            .to(be_equal_to(
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
            ));
    }

    #[test]
    fn tokenize_insert_query_varchar_value() {
        expect!(String::from("insert into table_name values('string');").into_tokenizer().tokenize())
            .to(be_equal_to(
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
            ));
    }

    #[test]
    fn tokenize_select_with_not_equal_predicate() {
        expect!(String::from("select col from table_1 where col <> 5;").into_tokenizer().tokenize())
            .to(be_equal_to(
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
            ));
    }

    #[test]
    fn tokenize_create_table_with_primary_key() {
        expect!(String::from("create table tab1 (col1 char(3) primary key);").into_tokenizer().tokenize())
            .to(
                be_equal_to(
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
                        Token::PrimaryKey,
                        Token::RParent,
                        Token::Semicolon
                    ]
                )
            );
    }

    #[test]
    fn tokenize_create_table_with_not_null() {
        expect!(String::from("create table tab2 (col integer not null);").into_tokenizer().tokenize())
            .to(
                be_equal_to(
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
        expect!(String::from("create table tab_4 (col1 integer primary key, col2 integer foreign key references table1(col));").into_tokenizer().tokenize())
            .to(
                be_equal_to(
                    vec![
                        Token::Create,
                        Token::Table,
                        Token::ident("tab_4"),
                        Token::LParent,
                        Token::ident("col1"),
                        Token::Int,
                        Token::PrimaryKey,
                        Token::Comma,
                        Token::ident("col2"),
                        Token::Int,
                        Token::ForeignKey,
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
