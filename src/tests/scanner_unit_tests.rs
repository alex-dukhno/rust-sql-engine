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

// fn run_test(lexems: &Vec<&'a str>) {

// }

#[test]
fn test_one_word_in_line() {
    let lexems = vec!["one"];
    let lexem_string = build_lexems_string(&lexems);
    let mut scanner = Scanner::new(&(*lexem_string));
    let actual = scanner.next();
    assert!(actual.is_some());
}

#[test]
fn test_simple_whitespace_delimeter() {
    let lexems = vec!["lexem1", " ", "lexem2"];
    let lexem_string = build_lexems_string(&lexems);
    let mut scanner = Scanner::new(&(*lexem_string));
    test_lexems(&lexems, &mut scanner);
}
