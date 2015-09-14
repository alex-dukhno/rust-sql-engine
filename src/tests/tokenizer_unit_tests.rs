use sql::tokenizer::Tokenizer;

#[test]
fn test_create_tokenizer() {
    Tokenizer::new("some string");
}

#[test]
fn test_whitespace_delimeter() {
    let mut t = Tokenizer::new("one two");
    assert_eq!(t.next().unwrap(), "one");
    assert_eq!(t.next().unwrap(), " ");
    assert_eq!(t.next().unwrap(), "two");
}
