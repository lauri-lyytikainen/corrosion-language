#[cfg(test)]
mod comment_tests {
    use crate::lexer::{Token, TokenizeError, Tokenizer};

    fn tokenize_input(input: &str) -> Result<Vec<Token>, TokenizeError> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens_with_span = tokenizer.tokenize(input)?;
        Ok(tokens_with_span.into_iter().map(|t| t.token).collect())
    }

    #[test]
    fn test_single_line_comment() {
        let tokens = tokenize_input("let x = 42; // this is a comment").unwrap();
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
    fn test_single_line_comment_at_start() {
        let tokens = tokenize_input("// comment at start\nlet x = 5;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(5),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_multi_line_comment() {
        let tokens = tokenize_input("let x /* multi line\ncomment */ = 42;").unwrap();
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
    fn test_nested_operators_with_comments() {
        let tokens = tokenize_input("1 + /* addition */ 2 // result is 3").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(1), Token::Plus, Token::Number(2), Token::Eof]
        );
    }

    #[test]
    fn test_comment_with_special_characters() {
        let tokens = tokenize_input("let x = 1; // comment with symbols: ()[]{}+=*/<>!").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(1),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_multiple_single_line_comments() {
        let tokens = tokenize_input("// first comment\n// second comment\nlet x = 1;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(1),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }
}
