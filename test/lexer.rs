use expectest::prelude::be_equal_to;

use sql::lexer::Tokenizer;
use sql::lexer::Token::{Ident, NumericConstant, CharactersConstant};

#[test]
fn emits_none_when_given_an_empty_string() {
    expect!("".tokenize())
        .to(be_equal_to(vec![]));
}

#[test]
fn emits_identifier_token_when_given_a_single_word_string() {
    expect!("word".tokenize())
        .to(be_equal_to(vec![Ident("word".to_owned())]));
}

#[test]
fn emits_identifiers_when_given_string_of_words() {
    expect!("this is a sentence".tokenize())
        .to(be_equal_to(
            vec![
                Ident("this".to_owned()),
                Ident("is".to_owned()),
                Ident("a".to_owned()),
                Ident("sentence".to_owned())
            ]
        ));
}

#[test]
fn emits_number_token_when_given_number() {
    expect!("5".tokenize())
        .to(be_equal_to(vec![NumericConstant("5".to_owned())]));
}

#[test]
fn escapes_single_quote_inside_string_token() {
    expect!("\'str\'\'str\'".tokenize())
        .to(be_equal_to(vec![CharactersConstant("str\'str".to_owned())]));
}

#[test]
fn escapes_new_line_chars() {
    expect!("\nword".tokenize())
        .to(be_equal_to(vec![Ident("word".to_owned())]));
}

#[test]
fn escapes_tabs() {
    expect!("\tword".tokenize())
        .to(be_equal_to(vec![Ident("word".to_owned())]));
}

#[test]
fn emits_string_when_only_open_signle_quote() {
    expect!("\'str".tokenize())
        .to(be_equal_to(vec![CharactersConstant("str".to_owned())]));
}

#[test]
fn case_insensitive() {
    expect!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".tokenize())
        .to(be_equal_to(vec![Ident("abcdefghijklmnopqrstuvwxyz".to_owned())]));
}
