#[cfg(test)]
mod tests {
    use crate::lexer::{Token, TokenizeError, Tokenizer};

    fn tokenize_input(input: &str) -> Result<Vec<Token>, TokenizeError> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens_with_span = tokenizer.tokenize(input)?;
        Ok(tokens_with_span.into_iter().map(|t| t.token).collect())
    }

    #[test]
    fn test_empty_input() {
        let tokens = tokenize_input("").unwrap();
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_whitespace_only() {
        let tokens = tokenize_input("   \t  \n  ").unwrap();
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_single_let_keyword() {
        let tokens = tokenize_input("let").unwrap();
        assert_eq!(tokens, vec![Token::Let, Token::Eof]);
    }

    #[test]
    fn test_single_identifier() {
        let tokens = tokenize_input("variable").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Identifier("variable".to_string()), Token::Eof]
        );
    }

    #[test]
    fn test_identifier_with_numbers() {
        let tokens = tokenize_input("var123").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Identifier("var123".to_string()), Token::Eof]
        );
    }

    #[test]
    fn test_single_number() {
        let tokens = tokenize_input("42").unwrap();
        assert_eq!(tokens, vec![Token::Number(42), Token::Eof]);
    }

    #[test]
    fn test_large_number() {
        let tokens = tokenize_input("999999").unwrap();
        assert_eq!(tokens, vec![Token::Number(999999), Token::Eof]);
    }

    #[test]
    fn test_zero() {
        let tokens = tokenize_input("0").unwrap();
        assert_eq!(tokens, vec![Token::Number(0), Token::Eof]);
    }

    #[test]
    fn test_single_operators() {
        let tokens = tokenize_input("=").unwrap();
        assert_eq!(tokens, vec![Token::Assign, Token::Eof]);

        let tokens = tokenize_input(";").unwrap();
        assert_eq!(tokens, vec![Token::Semicolon, Token::Eof]);

        let tokens = tokenize_input(":").unwrap();
        assert_eq!(tokens, vec![Token::Colon, Token::Eof]);
    }

    #[test]
    fn test_simple_let_assignment() {
        let tokens = tokenize_input("let x = 42").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(42),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_let_assignment_with_semicolon() {
        let tokens = tokenize_input("let x = 42;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(42),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_typed_let_assignment() {
        let tokens = tokenize_input("let x: i32 = 42;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Colon,
                Token::Identifier("i32".to_string()),
                Token::Assign,
                Token::Number(42),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_whitespace_handling() {
        let tokens = tokenize_input("  let   x   =   42  ;  ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(42),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_multiple_assignments() {
        let tokens = tokenize_input("let x = 1; let y = 2;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(1),
                Token::Semicolon,
                Token::Let,
                Token::Identifier("y".to_string()),
                Token::Assign,
                Token::Number(2),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_identifier_variations() {
        let test_cases = vec![
            ("a", "a"),
            ("abc", "abc"),
            ("camelCase", "camelCase"),
            ("snake_case", "snake_case"),
            ("PascalCase", "PascalCase"),
            ("var1", "var1"),
            ("a1b2c3", "a1b2c3"),
        ];

        for (input, expected) in test_cases {
            let tokens = tokenize_input(input).unwrap();
            assert_eq!(
                tokens,
                vec![Token::Identifier(expected.to_string()), Token::Eof],
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_keyword_vs_identifier() {
        // 'let' should be tokenized as a keyword
        let tokens = tokenize_input("let").unwrap();
        assert_eq!(tokens, vec![Token::Let, Token::Eof]);

        // identifiers containing 'let' but not equal to 'let' should be identifiers
        let tokens = tokenize_input("letter").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Identifier("letter".to_string()), Token::Eof]
        );

        let tokens = tokenize_input("letme").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Identifier("letme".to_string()), Token::Eof]
        );
    }

    #[test]
    fn test_consecutive_operators() {
        let tokens = tokenize_input("=;:").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Assign, Token::Semicolon, Token::Colon, Token::Eof]
        );
    }

    #[test]
    fn test_mixed_tokens() {
        let tokens = tokenize_input("let count: i32 = 0; count = count;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("count".to_string()),
                Token::Colon,
                Token::Identifier("i32".to_string()),
                Token::Assign,
                Token::Number(0),
                Token::Semicolon,
                Token::Identifier("count".to_string()),
                Token::Assign,
                Token::Identifier("count".to_string()),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_span_information() {
        let mut tokenizer = Tokenizer::new("");
        let tokens_with_span = tokenizer.tokenize("let x = 42").unwrap();

        // Check that spans are properly calculated
        assert_eq!(tokens_with_span.len(), 5); // let, x, =, 42, EOF

        // Check 'let' token span
        assert_eq!(tokens_with_span[0].span.start, 0);
        assert_eq!(tokens_with_span[0].span.end, 3);

        // Check 'x' token span
        assert_eq!(tokens_with_span[1].span.start, 4);
        assert_eq!(tokens_with_span[1].span.end, 5);

        // Check '=' token span
        assert_eq!(tokens_with_span[2].span.start, 6);
        assert_eq!(tokens_with_span[2].span.end, 7);

        // Check '42' token span
        assert_eq!(tokens_with_span[3].span.start, 8);
        assert_eq!(tokens_with_span[3].span.end, 10);
    }

    #[test]
    fn test_multiline_input() {
        let input = "let x = 1;\nlet y = 2;";
        let tokens = tokenize_input(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(1),
                Token::Semicolon,
                Token::Let,
                Token::Identifier("y".to_string()),
                Token::Assign,
                Token::Number(2),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_line_and_column_calculation() {
        let mut tokenizer = Tokenizer::new("");
        let tokens_with_span = tokenizer.tokenize("let x = 1;\nlet y = 2;").unwrap();

        // First line tokens should have line 1
        assert_eq!(tokens_with_span[0].span.line, 1); // let
        assert_eq!(tokens_with_span[1].span.line, 1); // x
        assert_eq!(tokens_with_span[4].span.line, 1); // ;

        // Second line tokens should have line 2
        assert_eq!(tokens_with_span[5].span.line, 2); // let
        assert_eq!(tokens_with_span[6].span.line, 2); // y
    }

    #[test]
    fn test_error_on_invalid_character() {
        let mut tokenizer = Tokenizer::new("");
        let result = tokenizer.tokenize("let x = @");
        assert!(result.is_err());

        if let Err(TokenizeError::ParseError(msg)) = result {
            assert!(msg.contains("Parse error"));
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_error_on_invalid_input() {
        let mut tokenizer = Tokenizer::new("");
        let result = tokenizer.tokenize("let x # invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_tokens_except_eof() {
        let tokens = tokenize_input("").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_numbers_edge_cases() {
        // Single digit
        let tokens = tokenize_input("1").unwrap();
        assert_eq!(tokens, vec![Token::Number(1), Token::Eof]);

        // Multiple digits
        let tokens = tokenize_input("12345").unwrap();
        assert_eq!(tokens, vec![Token::Number(12345), Token::Eof]);

        // Number followed by identifier (should be separate tokens)
        let tokens = tokenize_input("123abc").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123),
                Token::Identifier("abc".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_complex_expression() {
        let input = "let result: i32 = x = y;";
        let tokens = tokenize_input(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("result".to_string()),
                Token::Colon,
                Token::Identifier("i32".to_string()),
                Token::Assign,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Identifier("y".to_string()),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_new_keywords() {
        let tokens = tokenize_input("fn fix inl inr").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Fix,
                Token::Inl,
                Token::Inr,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_new_operators() {
        let tokens = tokenize_input("== != <= >= && || !").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Equal,
                Token::NotEqual,
                Token::LessThanEqual,
                Token::GreaterThanEqual,
                Token::LogicalAnd,
                Token::LogicalOr,
                Token::LogicalNot,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_new_punctuation() {
        let tokens = tokenize_input("{ }").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::RightBrace,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_function_syntax_tokenization() {
        let tokens = tokenize_input("fn(x) { x + 1 }").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::LeftParen,
                Token::Identifier("x".to_string()),
                Token::RightParen,
                Token::LeftBrace,
                Token::Identifier("x".to_string()),
                Token::Plus,
                Token::Number(1),
                Token::RightBrace,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_pair_destructuring_keywords() {
        let tokens = tokenize_input("fst snd").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fst,
                Token::Snd,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_pair_destructuring_syntax() {
        let tokens = tokenize_input("fst(pair) snd(pair)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fst,
                Token::LeftParen,
                Token::Identifier("pair".to_string()),
                Token::RightParen,
                Token::Snd,
                Token::LeftParen,
                Token::Identifier("pair".to_string()),
                Token::RightParen,
                Token::Eof
            ]
        );
    }
}
