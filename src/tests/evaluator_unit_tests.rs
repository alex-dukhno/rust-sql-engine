use std::vec::Vec;

use sql::lexer::Evaluator;

#[test]
fn test_evaluate_spaces() {
    let query_string = "select 1 from tab1";
    let mut e = Evaluator::new(query_string);
    let tokens = vec!["select", "1", "from", "tab1"];
    for &expected in tokens.iter() {
        let actual = e.next();
        assert_eq!(actual.unwrap(), expected);
    }
}

#[test]
#[ignore]
fn test_evaluate_line_comment() {
    let query_string = "insert into tab1(col1)--comment here\n values('1');";
}
