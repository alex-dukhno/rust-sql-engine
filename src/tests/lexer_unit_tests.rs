use std::vec::Vec;
use std::boxed::Box;

use sql::lexer::Lexer;

#[test]
fn test_one_word_in_line() {
    let tokens = vec!["one"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    let actual = t.next();
    assert!(actual.is_some());
}

fn build_tokens_string<'a>(tokens: &Vec<&'a str>) -> Box<String> {
    let mut s = "".to_string();
    for &token in (*tokens).iter() {
        s = s + token;
    }
    Box::new(s)
}

#[test]
fn test_one_word_with_first_and_last_same_char() {
    let tokens = vec!["eone"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_simple_whitespace_delimeter() {
    let tokens = vec!["one", " ", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

fn test_tokens<'a>(expected_tokens: &Vec<&'a str>, actual_tokens: &mut Lexer<'a>) {
    for &expected in (*expected_tokens).iter() {
        let actual = actual_tokens.next();
        match actual {
            Some(v) => assert_eq!(v, expected),
            None => assert!(expected == ""),
        }
    }
}

#[test]
fn test_simple_tabulation_delimeter() {
    let tokens = vec!["one", "\t", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_simple_new_line_delimeter() {
    let tokens = vec!["one", "\n", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_end_of_line_as_none() {
    let tokens = vec!["one", "\n", "two", ""];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_whitespaces_as_one_token() {
    let tokens = vec!["one", " \t \n ", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_exclamation_mark() {
    let tokens = vec!["one", "!", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_white_spaces_delimited_with_special_char() {
    let tokens = vec![" \n\t", "!", "\n "];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_question_mark() {
    let tokens = vec!["one", "?", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_percent() {
    let tokens = vec!["one", "%", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_open_parenthesis() {
    let tokens = vec!["one", "(", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_close_parenthesis() {
    let tokens = vec!["one", ")", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_quote() {
    let tokens = vec!["one", "'", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}

#[test]
fn test_double_quote() {
    let tokens = vec!["one", "\"", "two"];
    let token_string = build_tokens_string(&tokens);
    let mut t = Lexer::new(&(*token_string));
    test_tokens(&tokens, &mut t);
}
