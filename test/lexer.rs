use sql::lexer::tokenize;

fn assert_that_tokenized_into(src: &str, expected: &str) {
    match tokenize(src) {
        Ok(good) => assert_eq!(format!("{:?}", good), expected),
        _ => panic!("unimplemented assertion check for errors handling")
    }
}

#[cfg(test)]
mod should_emit {
    use super::assert_that_tokenized_into;

    #[test]
    fn none_when_given_an_empty_string() {
        assert_that_tokenized_into("", "[]");
    }

    #[test]
    fn identifier_token_when_given_a_single_word_string() {
        assert_that_tokenized_into("word", "[Ident('word')]");
    }

    #[test]
    fn identifiers_when_given_string_of_words() {
        assert_that_tokenized_into("this is a sentence", "[Ident('this'), Ident('is'), Ident('a'), Ident('sentence')]");
    }

    #[test]
    fn number_constant_when_given_number() {
        assert_that_tokenized_into("5", "[NumericConstant(5)]");
    }

    #[test]
    fn tokens_case_insensitive() {
        assert_that_tokenized_into("ABCD", "[Ident('abcd')]");
    }

    #[test]
    fn string_constant_when_given_letters_surrounded_by_single_quotes() {
        assert_that_tokenized_into("'str'", "[StringConstant(str)]");
    }

    #[test]
    fn string_constant_when_only_open_signle_quote() {
        assert_that_tokenized_into("'str", "[StringConstant(str)]");
    }
}

#[cfg(test)]
mod should_escape {
    use super::assert_that_tokenized_into;

    #[test]
    fn escapes_new_line_chars() {
        assert_that_tokenized_into("\nword", "[Ident('word')]");
    }

    #[test]
    fn escapes_tabs() {
        assert_that_tokenized_into("\tword", "[Ident('word')]");
    }

}

#[cfg(test)]
mod should_resolve_single_quotes {
    use super::assert_that_tokenized_into;

    #[test]
    fn inside_string_token() {
        assert_that_tokenized_into("'str''str'", "[StringConstant(str'str)]")
    }

    #[test]
    fn at_the_end() {
        assert_that_tokenized_into("'str'''", "[StringConstant(str')]");
    }

    #[test]
    fn at_the_begining() {
        assert_that_tokenized_into("'''str'", "[StringConstant('str)]");
    }

    #[test]
    fn everywhere() {
        assert_that_tokenized_into("'''str''str'''", "[StringConstant('str'str')]");
    }
}

#[cfg(test)]
mod should_understand_cmp_tokens_such_as {
    use super::assert_that_tokenized_into;

    #[test]
    fn equal_sign() {
        assert_that_tokenized_into("=", "[EqualTo]");
    }

    #[test]
    fn not_equal_sign_angle_brackets() {
        assert_that_tokenized_into("<>", "[NotEqualTo]");
    }

    #[test]
    fn not_equal_sign_exclamation_mark_equal_sign() {
        assert_that_tokenized_into("!=", "[NotEqualTo]");
    }

    #[test]
    fn less_then_sign() {
        assert_that_tokenized_into("<", "[LessThan]");
    }

    #[test]
    fn less_or_equal_sign() {
        assert_that_tokenized_into("<=", "[LessThanOrEqualTo]");
    }

    #[test]
    fn greater_then_sign() {
        assert_that_tokenized_into(">", "[GreaterThan]");
    }

    #[test]
    fn greate_or_equal_sign() {
        assert_that_tokenized_into(">=", "[GreaterThanOrEqualTo]");
    }
}

#[cfg(test)]
mod should_skip {
    use super::assert_that_tokenized_into;

    #[test]
    fn double_dashe() {
        assert_that_tokenized_into("text here--but not here", "[Ident('text'), Ident('here')]");
    }

    #[test]
    fn double_dashe_only_till_new_line() {
        assert_that_tokenized_into(
            "test here -- and not here\nbut here",
            "[Ident('test'), Ident('here'), Ident('but'), Ident('here')]"
        );
    }

    #[test]
    fn from_slash_star_till_star_slash() {
        assert_that_tokenized_into(
            "text here /* is commented */ is not commented",
            "[Ident('text'), Ident('here'), Ident('is'), KeyWord('NOT'), Ident('commented')]"
        );
    }

    #[test]
    fn multiple_one_line_comments() {
        assert_that_tokenized_into(
            "text--comment 1\n and text--comment 2\n and text--comment 3\n and text",
            "[Ident('text'), KeyWord('AND'), Ident('text'), KeyWord('AND'), Ident('text'), KeyWord('AND'), Ident('text')]"
        );
    }

    #[test]
    fn multiple_multy_line_comments() {
        assert_that_tokenized_into(
            "text/*comment 1*/ and text/*comment 2*/ and text/*comment 3*/ and text",
            "[Ident('text'), KeyWord('AND'), Ident('text'), KeyWord('AND'), Ident('text'), KeyWord('AND'), Ident('text')]"
        );
    }
}

#[cfg(test)]
mod should_not_skip {
    use super::assert_that_tokenized_into;

    #[test]
    fn one_dash() {
        assert_that_tokenized_into(
            "text here - and here",
            "[Ident('text'), Ident('here'), Symbol(-), KeyWord('AND'), Ident('here')]"
        );
    }


    #[test]
    fn till_star_slash() {
        assert_that_tokenized_into(
            "text here--and till the new line*/ should be skipped\n",
            "[Ident('text'), Ident('here')]"
        );
    }

    #[cfg(test)]
    mod inside_string_leterals {
        use super::super::assert_that_tokenized_into;

        #[test]
        fn double_dashes() {
            assert_that_tokenized_into("'text here--but not here'", "[StringConstant(text here--but not here)]");
        }

        #[test]
        fn double_dashes_till_new_line() {
            assert_that_tokenized_into(
                "'test here -- and not here\nbut here'",
                "[StringConstant(test here -- and not here\nbut here)]"
            );
        }

        #[test]
        fn from_slash_star_till_star_slash() {
            assert_that_tokenized_into(
                "'text here /* is commented */ is not commented'",
                "[StringConstant(text here /* is commented */ is not commented)]"
            );
        }

        #[test]
        fn multiple_one_line_comments() {
            assert_that_tokenized_into(
                "'text--comment 1\n and text--comment 2\n and text--comment 3\n and text'",
                "[StringConstant(text--comment 1\n and text--comment 2\n and text--comment 3\n and text)]"
            );
        }

        #[test]
        fn multiple_multy_line_comments() {
            assert_that_tokenized_into(
                "'text/*comment 1*/ and text/*comment 2*/ and text/*comment 3*/ and text'",
                "[StringConstant(text/*comment 1*/ and text/*comment 2*/ and text/*comment 3*/ and text)]"
            );
        }
    }

}

#[cfg(test)]
mod should_understand_operations_such_as {
    use super::assert_that_tokenized_into;

    #[test]
    fn simple_plus() {
        assert_that_tokenized_into("+", "[Symbol(+)]");
    }

    #[test]
    fn addition() {
        assert_that_tokenized_into("4 + 5", "[NumericConstant(4), Symbol(+), NumericConstant(5)]");
    }

    #[test]
    fn addition_without_spaces() {
        assert_that_tokenized_into("4+5", "[NumericConstant(4), Symbol(+), NumericConstant(5)]");
    }

    #[test]
    fn simple_minus() {
        assert_that_tokenized_into("-", "[Symbol(-)]");
    }

    #[test]
    fn subtraction() {
        assert_that_tokenized_into("5 - 6", "[NumericConstant(5), Symbol(-), NumericConstant(6)]");
    }

    #[test]
    fn subtraction_without_spaces() {
        assert_that_tokenized_into("5-6", "[NumericConstant(5), Symbol(-), NumericConstant(6)]");
    }

    #[test]
    fn simple_asterisk() {
        assert_that_tokenized_into("*", "[Symbol(*)]");
    }

    #[test]
    fn multiplication() {
        assert_that_tokenized_into("5 * 4", "[NumericConstant(5), Symbol(*), NumericConstant(4)]");
    }

    #[test]
    fn multiplication_without_spaces() {
        assert_that_tokenized_into("5*4", "[NumericConstant(5), Symbol(*), NumericConstant(4)]");
    }

    #[test]
    fn simple_slash() {
        assert_that_tokenized_into("/", "[Symbol(/)]");
    }

    #[test]
    fn division() {
        assert_that_tokenized_into("78 / 34", "[NumericConstant(78), Symbol(/), NumericConstant(34)]");
    }

    #[test]
    fn division_without_spaces() {
        assert_that_tokenized_into("78/34", "[NumericConstant(78), Symbol(/), NumericConstant(34)]");
    }
}

#[cfg(test)]
mod sql_query {
    use super::assert_that_tokenized_into;

    #[test]
    fn semicolon_symbol_token() {
        assert_that_tokenized_into(";", "[Symbol(';')]");
    }

    #[test]
    fn comma_symbol_token() {
        assert_that_tokenized_into(",", "[Symbol(',')]");
    }

    #[test]
    fn open_parenthes_token() {
        assert_that_tokenized_into("(", "[Symbol('(')]");
    }

    #[test]
    fn closed_parenthes_token() {
        assert_that_tokenized_into(")", "[Symbol(')')]");
    }

    #[test]
    fn insert_keyword_token() {
        assert_that_tokenized_into("insert", "[KeyWord('INSERT')]");
    }

    #[test]
    fn into_keyword_token() {
        assert_that_tokenized_into("into", "[KeyWord('INTO')]");
    }

    #[test]
    fn values_keyword_token() {
        assert_that_tokenized_into("values", "[KeyWord('VALUES')]");
    }

    #[test]
    fn select_keyword_token() {
        assert_that_tokenized_into("select", "[KeyWord('SELECT')]");
    }

    #[test]
    fn from_keyword_token() {
        assert_that_tokenized_into("from", "[KeyWord('FROM')]");
    }

    #[test]
    fn where_keyword_token() {
        assert_that_tokenized_into("where", "[KeyWord('WHERE')]");
    }

    #[test]
    fn create_keyword_token() {
        assert_that_tokenized_into("create", "[KeyWord('CREATE')]");
    }

    #[test]
    fn table_keyword_token() {
        assert_that_tokenized_into("table", "[KeyWord('TABLE')]");
    }

    #[test]
    fn character_keyword_token() {
        assert_that_tokenized_into("character", "[KeyWord('CHARACTER')]");
    }

    #[test]
    fn integer_keyword_token() {
        assert_that_tokenized_into("integer", "[KeyWord('INTEGER')]");
    }

    #[test]
    fn primary_keyword_token() {
        assert_that_tokenized_into("primary", "[KeyWord('PRIMARY')]");
    }

    #[test]
    fn foreign_keyword_token() {
        assert_that_tokenized_into("foreign", "[KeyWord('FOREIGN')]");
    }

    #[test]
    fn key_keyword_token() {
        assert_that_tokenized_into("key", "[KeyWord('KEY')]");
    }

    #[test]
    fn references_keyword_token() {
        assert_that_tokenized_into("references", "[KeyWord('REFERENCES')]");
    }

    #[test]
    fn not_keyword_token() {
        assert_that_tokenized_into("not", "[KeyWord('NOT')]");
    }

    #[test]
    fn null_keyword_token() {
        assert_that_tokenized_into("null", "[KeyWord('NULL')]");
    }
}
