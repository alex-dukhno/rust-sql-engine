extern crate sql;

pub use sql::lexer::Lexer;
pub use sql::lexer::Token::*;

describe! lexer_test {

    it "create lexer" {
        Lexer::new("some line here");
    }

    it "emptyline" {
        let mut lexer = Lexer::new("");

        assert_eq!(lexer.next_lexem(), None);
    }

    it "word token" {
        let mut lexer = Lexer::new("iNseRt");

        assert_eq!(lexer.next_lexem(), Some(Word("insert".to_string())));
        assert_eq!(lexer.next_lexem(), None);
    }

    it "left parenthesis" {
        let mut lexer = Lexer::new("(");

        assert_eq!(lexer.next_lexem(), Some(LeftParenthesis));
        assert_eq!(lexer.next_lexem(), None);
    }

    it "right parenthesis" {
        let mut lexer = Lexer::new(")");

        assert_eq!(lexer.next_lexem(), Some(RightParenthesis));
        assert_eq!(lexer.next_lexem(), None);
    }

    it "semicolon" {
        let mut lexer = Lexer::new(";");

        assert_eq!(lexer.next_lexem(), Some(SemiColon));
        assert_eq!(lexer.next_lexem(), None);
    }

    it "single quote" {
        let mut lexer = Lexer::new("'");

        assert_eq!(lexer.next_lexem(), Some(SingleQuote));
        assert_eq!(lexer.next_lexem(), None);
    }

    it "new lines between words" {
        let mut lexer = Lexer::new("one\n\n\n\ntwo\n\n\nthree");

        assert_eq!(lexer.next_lexem(), Some(Word("one".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Word("two".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Word("three".to_string())));
        assert_eq!(lexer.next_lexem(), None);
    }

    it "new lines in the end of lexer line" {
        let mut lexer = Lexer::new("one two three\n\n\n");

        assert_eq!(lexer.next_lexem(), Some(Word("one".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Word("two".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Word("three".to_string())));
        assert_eq!(lexer.next_lexem(), None);
    }
}
