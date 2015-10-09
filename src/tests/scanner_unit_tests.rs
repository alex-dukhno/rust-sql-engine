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
fn test_empty_line() {
    run_test(&(vec![""]));
}

#[test]
fn test_one_word_in_line() {
    run_test(&(vec!["one"]));
}

#[test]
fn test_whitespace_and_tabulation_as_one_delimeter() {
    run_test(&(vec!["lexem1", " \t\t\t   \t  ", "lexem2"]));
}

#[test]
fn test_new_lines_divide_whitespace_and_tabulation() {
    run_test(&(vec!["  \t\t   ", "\n\n", "  \t\t   "]));
}

#[test]
fn test_simple_sql_query() {
    run_test(&(vec!["insert", " ", "into", " ", "table1", " ", "values", "(", "'", "1", "'", ")", ";"]));
}

#[test]
fn test_simple_select_query() {
    run_test(&(vec!["select", " ", "*", " ", "from", " ", "table1", ";"]));
}

#[test]
fn test_select_query_with_alias() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", ";"]))
}

#[test]
fn test_select_with_where_clause() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "=", " ", "1", ";"]));
}

#[test]
fn test_select_with_sum_in_where_clause() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "=", "2", "+", "1", ";"]));
}

#[test]
fn test_select_with_subtraction_in_where_clause() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "=", "2", "-", "1", ";"]));
}

#[test]
fn test_select_with_multiplication_in_where_clause() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "=", "2", "*", "1", ";"]));
}

#[test]
fn test_select_with_division_in_where_clause() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "=", "2", "/", "1", ";"]));
}

#[test]
fn test_select_with_less_then_condition() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "<", "1", ";"]));
}

#[test]
fn test_select_with_less_then_or_equals_condition() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "<=", "1", ";"]));
}

#[test]
fn test_select_with_more_then_condition() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", ">", "1", ";"]));
}

#[test]
fn test_select_with_more_then_or_equals_condition() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", ">=", "1", ";"]));
}

#[test]
fn test_select_with_not_equals_condition_sql_standard() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "<>", "1", ";"]));
}

#[test]
fn test_select_with_not_equals_condition_sql_not_standard() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "!=", "1", ";"]));
}

#[test]
fn test_select_with_equals_condition() {
    run_test(&(vec!["select", " ", "t1", ".", "col1", " ", "from", " ", "table1", " ", "t1", " ", "where", " ", "t1", ".", "col1", " ", "==", "1", ";"]));
}

#[test]
fn test_line_comments() {
    run_test(&(vec!["select", " ", "*", "--", "some", " ", "comment", " ", "here", "\n", "from", " ", "tab", ";"]));
}

#[test]
//corner case when query is like 'select */*blabla*/ from tab1'
fn test_block_comments() {
    run_test(&(vec!["select", " ", "*", " ", "/*", "some", " ", "comment", " ", "here", "*/", "from", " ", "tab", ";"]))
}
