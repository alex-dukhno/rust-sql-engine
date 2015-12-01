extern crate sql;

pub use sql::lexer::Lexer;
pub use sql::lexer::Token;

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

        assert_eq!(lexer.next_lexem(), Some(Token::Word("insert".to_string())));
    }

    it "left parenthesis" {
        let mut lexer = Lexer::new("(");

        assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
    }

    it "right parenthesis" {
        let mut lexer = Lexer::new(")");

        assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
    }

    it "semicolon" {
        let mut lexer = Lexer::new(";");

        assert_eq!(lexer.next_lexem(), Some(Token::SemiColon));
    }

    it "single quote" {
        let mut lexer = Lexer::new("'");

        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
    }

    it "insert query" {
        let mut lexer = Lexer::new("insert into tab1 values (1 , '1');");

        assert_eq!(lexer.next_lexem(), Some(Token::Word("insert".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("into".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("tab1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("values".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Colon));
        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
        assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::SemiColon));
    }

    it "insert query with column sequence" {
        let mut lexer = Lexer::new("insert into tab1 (col_1 , col2) values(1, '1');");

        assert_eq!(lexer.next_lexem(), Some(Token::Word("insert".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("into".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("tab1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("col_1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Colon));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("col2".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("values".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Colon));
        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
        assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::SemiColon));
    }

    it "escaping by double single qout" {
        let mut lexer = Lexer::new("insert into tab1\n(col_1\t, col2 ) values (1, 'ab''s');");

        assert_eq!(lexer.next_lexem(), Some(Token::Word("insert".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("into".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("tab1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("col_1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Colon));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("col2".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("values".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Colon));
        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("ab's".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
        assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::SemiColon));
    }

    it "line with comments" {
        let mut lexer = Lexer::new("insert into -- coment here finished -> \n tab1");

        assert_eq!(lexer.next_lexem(), Some(Token::Word("insert".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("into".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("tab1".to_string())));
        assert_eq!(lexer.next_lexem(), None);
        assert_eq!(lexer.next_lexem(), None);
    }

    it "insert query with escaped single quote in the begining" {
        let mut lexer = Lexer::new("insert into tab1 values ('''abs');");

        assert_eq!(lexer.next_lexem(), Some(Token::Word("insert".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("into".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("tab1".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("values".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
        assert_eq!(lexer.next_lexem(), Some(Token::Word("'abs".to_string())));
        assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
        assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next_lexem(), Some(Token::SemiColon));
    }

}
