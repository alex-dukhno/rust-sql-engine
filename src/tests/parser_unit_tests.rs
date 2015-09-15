use sql::lexer::Lexer;
use sql::parser::Parser;

#[test]
fn test_create_parser() {
    let query = "inser into tab1(col1)values('1');";
    let lexer = Lexer::new(query);
    Parser::new(&lexer);
}
