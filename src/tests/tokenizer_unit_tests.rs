use sql::tokenizer::Tokenizer;

#[test]
fn test_create_tokenizer() {
    Tokenizer::new("some string");
}

#[test]
fn test_whitespace_delimeter() {
    let mut t = Tokenizer::new("one two");
    let one = t.next();
    assert!(one.is_some());
    assert_eq!(one.unwrap(), "one");
    let two = t.next();
    assert!(two.is_some());
    assert_eq!(two.unwrap(), "two");
}
