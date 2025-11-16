use super::tokens::{Span, Token, TokenWithSpan};

#[derive(Debug, Clone)]
pub enum TokenizeError {
    ParseError(String),
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenizeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for TokenizeError {}

pub struct Tokenizer;

impl Tokenizer {
    pub fn new(_input: &str) -> Self {
        Self
    }

    pub fn tokenize(&mut self, input: &str) -> Result<Vec<TokenWithSpan>, TokenizeError> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = input.chars().collect();
        let mut position = 0;

        while position < chars.len() {
            // Skip whitespace
            while position < chars.len() && chars[position].is_whitespace() {
                position += 1;
            }

            if position >= chars.len() {
                break;
            }

            let start_pos = position;
            let token = match chars[position] {
                '=' => {
                    position += 1;
                    Token::Assign
                }
                ';' => {
                    position += 1;
                    Token::Semicolon
                }
                ':' => {
                    position += 1;
                    Token::Colon
                }
                c if c.is_ascii_digit() => {
                    let start = position;
                    while position < chars.len() && chars[position].is_ascii_digit() {
                        position += 1;
                    }
                    let number_str: String = chars[start..position].iter().collect();
                    Token::Number(number_str.parse().unwrap_or(0))
                }
                c if c.is_alphabetic() => {
                    let start = position;
                    while position < chars.len() && (chars[position].is_alphanumeric()) {
                        position += 1;
                    }
                    let identifier: String = chars[start..position].iter().collect();
                    match identifier.as_str() {
                        "let" => Token::Let,
                        _ => Token::Identifier(identifier),
                    }
                }
                c => {
                    return Err(TokenizeError::ParseError(format!(
                        "Unexpected character: '{}'",
                        c
                    )));
                }
            };

            let end_pos = position;
            let span = Span::new(start_pos, end_pos, 1, 1);
            tokens.push(TokenWithSpan::new(token, span));
        }

        // Add EOF token
        let span = Span::new(input.len(), input.len(), 1, 1);
        tokens.push(TokenWithSpan::new(Token::Eof, span));

        Ok(tokens)
    }
}
