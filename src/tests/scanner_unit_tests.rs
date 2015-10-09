use std::vec::Vec;
use std::boxed::Box;

use sql::lexer::Scanner;

fn build_lexems_string<'a>(lexems: &Vec<&'a str>) -> Box<String> {
    let mut s = "".to_string();
    for &token in (*lexems).iter() {
        s = s + token;
    }
    Box::new(s)
}

fn test_lexems<'a>(expected_lexems: &Vec<&'a str>, actual_lexems: &mut Scanner<'a>) {
    for &expected in (*expected_lexems).iter() {
        let actual = actual_lexems.next();
        match actual {
            Some(actual) => assert_eq!(actual, expected),
            None => assert_eq!("", expected),
        }
    }
}

fn run_test<'a>(lexems: &Vec<&'a str>) {
    let lexem_string = build_lexems_string(lexems);
    let mut scanner = Scanner::new(&(*lexem_string));
    test_lexems(&lexems, &mut scanner);
}

#[test]
fn test_one_word_in_line() {
    run_test(&(vec!["one"]));
}

#[test]
fn test_simple_whitespace_delimeter() {
    run_test(&(vec!["lexem1", " ", "lexem2"]));
}

#[test]
fn test_simple_tabulation_delimeter() {
    run_test(&(vec!["lexem1", "\t", "lexem2"]));
}

#[test]
fn test_whitespace_and_tabulation_as_one_delimeter() {
    run_test(&(vec!["lexem1", " \t\t\t   \t  ", "lexem2"]));
}

#[test]
fn test_simple_new_line_delimeter() {
    run_test(&(vec!["lexem1", "\n", "lexem2"]));
}

#[test]
fn test_new_lines_divide_whitespace_and_tabulation() {
    run_test(&(vec!["  \t\t   ", "\n\n", "  \t\t   "]));
}

#[test]
fn test_empty_line() {
    run_test(&(vec![""]));
}

#[test]
fn test_left_parenthesis_as_lexem() {
    run_test(&(vec!["lexem1", "(", "(", "(", "lexem2"]));
}

#[test]
fn test_quote_as_lexem() {
    run_test(&(vec!["lexem1", "'", "'", "lexem2"]));
}

#[test]
fn test_right_parenthesis_as_lexem() {
    run_test(&(vec!["lexem1", ")", ")", ")", "lexem2"]));
}

#[test]
fn test_samicolon_as_lexem() {
    run_test(&(vec!["lexem1", ";", ";", ";", "lexem2"]));
}

#[test]
// #[ignore]
fn test_simple_sql_query() {
    run_test(&(vec!["insert", " ", "into", " ", "table1", " ", "values", "(", "'", "1", "'", ")", ";"]));
}
