use expectest::prelude::{be_ok, be_err};

use sql::lexer::Tokenizer;
use sql::lexer::Token::{IdentT, NumberT, StringT};

#[test]
fn it_emits_none_when_given_an_empty_string() {
    expect!("".tokenize())
        .to(be_ok().value(vec![]));
}

#[test]
fn it_emits_identifier_token_when_given_a_single_word_string() {
    expect!("word".tokenize())
        .to(be_ok().value(vec![IdentT("word".to_owned())]));
}

#[test]
fn it_emits_identifiers_when_given_string_of_words() {
    expect!("this is a sentence".tokenize())
        .to(be_ok().value(
            vec![
                IdentT("this".to_owned()),
                IdentT("is".to_owned()),
                IdentT("a".to_owned()),
                IdentT("sentence".to_owned())
            ]
        ));
}

#[test]
fn it_emits_number_token_when_given_number() {
    expect!("5".tokenize())
        .to(be_ok().value(vec![NumberT("5".to_owned())]));
}

#[test]
fn it_emits_number_token_when_given_number_with_float_point() {
    expect!("2.01".tokenize())
        .to(be_ok().value(vec![NumberT("2.01".to_owned())]));
}

#[test]
fn it_emits_error_when_given_number_with_two_delimeters() {
    expect!("2.0.1".tokenize())
        .to(be_err().value("Number format error"));
}

#[test]
fn it_escapes_single_quote_inside_string_token() {
    expect!("\'str\'\'str\'".tokenize())
        .to(be_ok().value(vec![StringT("str\'str".to_owned())]));
}

#[test]
fn it_escapes_new_line_chars() {
    expect!("\nword".tokenize())
        .to(be_ok().value(vec![IdentT("word".to_owned())]));
}

#[test]
fn it_escapes_tabs() {
    expect!("\tword".tokenize())
        .to(be_ok().value(vec![IdentT("word".to_owned())]));
}

#[test]
fn it_emits_error_when_string_token_is_not_closed() {
    expect!("\'str".tokenize())
        .to(be_err().value("string const should be closed by \'".to_owned()));
}

#[test]
fn it_case_insensitive() {
    expect!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".tokenize())
        .to(be_ok().value(vec![IdentT("abcdefghijklmnopqrstuvwxyz".to_owned())]));
}
