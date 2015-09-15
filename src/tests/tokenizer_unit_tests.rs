use std::vec::Vec;
use std::boxed::Box;

use sql::tokenizer::Tokenizer;

#[test]
fn test_one_word_in_line() {
    let tokens = vec!["one"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Tokenizer::new(&(*token_string));
    let actual = t.next();
    assert!(actual.is_some());
}

#[test]
fn test_simple_whitespace_delimeter() {
    let tokens = vec!["one", " ", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Tokenizer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

fn build_tokens_string<'a>(tokens: &Vec<&'a str>) -> Box<String> {
    let mut s = "".to_string();
    for &token in (*tokens).iter() {
        s = s + token;
    }
    Box::new(s)
}

fn test_tokens<'a>(expected_tokens: &Vec<&'a str>, actual_tokens: &mut Tokenizer<'a>) {
    for &expected in (*expected_tokens).iter() {
        let actual = actual_tokens.next();
        assert_eq!(actual.unwrap(), expected);
    }
}

#[test]
#[ignore]
fn test_simple_tabulation_delimeter() {
    let mut t = Tokenizer::new("one\ttwo");
    let one = t.next();
    assert_eq!(one.unwrap(), "one");
    // let tabulation = t.next();
    // assert_eq!(tabulation.unwrap(), "\t");
    let two = t.next();
    assert_eq!(two.unwrap(), "two");
}

#[test]
#[ignore]
fn test_simple_new_line_delimeter() {
    let mut t = Tokenizer::new("one\ntwo");
    let one = t.next();
    assert_eq!(one.unwrap(), "one");
    // let new_line = t.next();
    // assert_eq!(new_line.unwrap(), "\n");
    let two = t.next();
    assert_eq!(two.unwrap(), "two");
}
