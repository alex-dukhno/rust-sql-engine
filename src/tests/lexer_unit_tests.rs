use std::vec::Vec;
use std::boxed::Box;

use sql::lexer::Lexer;

#[test]
fn test_one_word_in_line() {
    let lexems = vec!["one"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    let actual = lexer.next();
    assert!(actual.is_some());
}

fn build_lexems_string<'a>(lexems: &Vec<&'a str>) -> Box<String> {
    let mut s = "".to_string();
    for &token in (*lexems).iter() {
        s = s + token;
    }
    Box::new(s)
}

#[test]
fn test_one_word_with_first_and_last_same_char() {
    let lexems = vec!["eone"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_simple_whitespace_delimeter() {
    let lexems = vec!["one", " ", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

fn test_lexems<'a>(expected_lexems: &Vec<&'a str>, actual_lexems: &mut Lexer<'a>) {
    for &expected in (*expected_lexems).iter() {
        let actual = actual_lexems.next();
        match actual {
            Some(v) => assert_eq!(v, expected),
            None => assert!(expected == ""),
        }
    }
}

#[test]
fn test_simple_tabulation_delimeter() {
    let lexems = vec!["one", "\t", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_simple_new_line_delimeter() {
    let lexems = vec!["one", "\n", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_end_of_line_as_none() {
    let lexems = vec!["one", "\n", "two", ""];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_single_special_symbol_as_single_token() {
    let lexems = vec!["one", "%", "?", "!", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_single_whitespace_as_single_token() {
    let lexems = vec!["one", " ", "\t", "\n", " ", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_exclamation_mark() {
    let lexems = vec!["one", "!", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_white_spaces_delimited_with_special_char() {
    let lexems = vec![" ", "\n", "\t", "!", "\n", " "];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_question_mark() {
    let lexems = vec!["one", "?", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_percent() {
    let lexems = vec!["one", "%", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_open_parenthesis() {
    let lexems = vec!["one", "(", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_close_parenthesis() {
    let lexems = vec!["one", ")", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_quote() {
    let lexems = vec!["one", "'", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_double_quote() {
    let lexems = vec!["one", "\"", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_grate_then() {
    let lexems = vec!["one", ">", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}

#[test]
fn test_less_then() {
    let lexems = vec!["one", "<", "two"];
    let lexem_string = build_lexems_string(&lexems);
    let mut lexer = Lexer::new(&(*lexem_string));
    test_lexems(&lexems, &mut lexer);
}
