use expectest::prelude::be_equal_to;

use sql::lexer::Tokenizer;
use sql::lexer::Token;

#[test]
fn emits_none_when_given_an_empty_string() {
    expect!(Tokenizer::from("").tokenize())
        .to(be_equal_to(vec![]));
}

#[test]
fn emits_identifier_token_when_given_a_single_word_string() {
    expect!(Tokenizer::from("word").tokenize())
        .to(be_equal_to(vec![Token::ident("word")]));
}

#[test]
fn emits_identifiers_when_given_string_of_words() {
    expect!(Tokenizer::from("this is a sentence").tokenize())
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
    expect!(Tokenizer::from("5").tokenize())
        .to(be_equal_to(vec![Token::number("5")]));
}

#[test]
fn escapes_single_quote_inside_string_token() {
    expect!(Tokenizer::from("\'str\'\'str\'").tokenize())
        .to(be_equal_to(vec![Token::string("str\'str")]));
}

#[test]
fn escapes_single_quote_at_the_end() {
    expect!(Tokenizer::from("\'str\'\'\'").tokenize())
        .to(be_equal_to(vec![Token::string("str\'")]));
}

#[test]
fn escapes_new_line_chars() {
    expect!(Tokenizer::from("\nword").tokenize())
        .to(be_equal_to(vec![Token::ident("word")]));
}

#[test]
fn escapes_tabs() {
    expect!(Tokenizer::from("\tword").tokenize())
        .to(be_equal_to(vec![Token::ident("word")]));
}

#[test]
fn emits_string_when_only_open_signle_quote() {
    expect!(Tokenizer::from("\'str").tokenize())
        .to(be_equal_to(vec![Token::string("str")]));
}

#[test]
fn case_insensitive() {
    expect!(Tokenizer::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ").tokenize())
        .to(be_equal_to(vec![Token::ident("abcdefghijklmnopqrstuvwxyz")]));
}

#[cfg(test)]
mod sql_query {

    use expectest::prelude::be_equal_to;

    use sql::lexer::Tokenizer;
    use sql::lexer::Token;

    #[test]
    fn tokenize_insert_query_numeric_value() {
        expect!(Tokenizer::from("insert into table_name values(1);").tokenize())
            .to(be_equal_to(
                vec![
                    Token::ident("insert"),
                    Token::ident("into"),
                    Token::ident("table_name"),
                    Token::ident("values"),
                    Token::from('('),
                    Token::number("1"),
                    Token::from(')'),
                    Token::from(';')
                ]
            ));
    }

    #[test]
    fn tokenize_insert_query_varchar_value() {
        expect!(Tokenizer::from("insert into table_name values('string');").tokenize())
            .to(be_equal_to(
                vec![
                    Token::ident("insert"),
                    Token::ident("into"),
                    Token::ident("table_name"),
                    Token::ident("values"),
                    Token::from('('),
                    Token::string("string"),
                    Token::from(')'),
                    Token::from(';')
                ]
            ));
    }

    #[test]
    fn tokenize_select_with_not_equal_predicate() {
        expect!(Tokenizer::from("select col from table_1 where col <> 5;").tokenize())
            .to(be_equal_to(
                vec![
                    Token::ident("select"),
                    Token::ident("col"),
                    Token::ident("from"),
                    Token::ident("table_1"),
                    Token::ident("where"),
                    Token::ident("col"),
                    Token::from('<'),
                    Token::from('>'),
                    Token::number("5"),
                    Token::from(';')
                ]
            ));
    }
}
