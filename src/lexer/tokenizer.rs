use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{recognize, value},
    multi::many0,
    sequence::pair,
};

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
        match parse_tokens(input) {
            Ok((remaining, tokens)) => {
                if remaining.is_empty() {
                    Ok(tokens)
                } else {
                    Err(TokenizeError::ParseError(format!(
                        "Parse error: Unexpected remaining input: '{}'",
                        remaining
                    )))
                }
            }
            Err(e) => Err(TokenizeError::ParseError(format!("Parse error: {}", e))),
        }
    }
}

// Helper function to calculate line and column from position
fn calculate_position(input: &str, pos: usize) -> (usize, usize) {
    let prefix = &input[..pos.min(input.len())];
    let line = prefix.matches('\n').count() + 1;
    let column = prefix
        .rfind('\n')
        .map_or(pos + 1, |last_newline| pos - last_newline);
    (line, column)
}

fn parse_identifier_or_keyword(input: &str) -> IResult<&str, Token> {
    recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))
        .map(|s: &str| match s {
            "let" => Token::Let,
            _ => Token::Identifier(s.to_string()),
        })
        .parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    digit1.map_res(str::parse).map(Token::Number).parse(input)
}

fn parse_assign(input: &str) -> IResult<&str, Token> {
    value(Token::Assign, char('=')).parse(input)
}

fn parse_semicolon(input: &str) -> IResult<&str, Token> {
    value(Token::Semicolon, char(';')).parse(input)
}

fn parse_colon(input: &str) -> IResult<&str, Token> {
    value(Token::Colon, char(':')).parse(input)
}

fn parse_single_token(input: &str) -> IResult<&str, Token> {
    alt((
        parse_identifier_or_keyword,
        parse_number,
        parse_assign,
        parse_semicolon,
        parse_colon,
    ))
    .parse(input)
}

fn parse_token_with_whitespace<'a>(
    input: &'a str,
    original_input: &str,
    offset: usize,
) -> IResult<&'a str, Option<TokenWithSpan>> {
    let (input_after_ws, _) = multispace0(input)?;

    if input_after_ws.is_empty() {
        return Ok((input_after_ws, None));
    }

    let ws_consumed = input.len() - input_after_ws.len();
    let start_offset = offset + ws_consumed;
    let (rest, token) = parse_single_token(input_after_ws)?;
    let token_len = input_after_ws.len() - rest.len();
    let end_offset = start_offset + token_len;

    let (line, column) = calculate_position(original_input, start_offset);
    let span = Span::new(start_offset, end_offset, line, column);

    Ok((rest, Some(TokenWithSpan::new(token, span))))
}

fn parse_tokens(input: &str) -> IResult<&str, Vec<TokenWithSpan>> {
    let mut tokens = Vec::new();
    let mut remaining = input;
    let original_input = input;

    loop {
        let current_offset = original_input.len() - remaining.len();
        match parse_token_with_whitespace(remaining, original_input, current_offset) {
            Ok((rest, Some(token))) => {
                tokens.push(token);
                remaining = rest;
            }
            Ok((rest, None)) => {
                remaining = rest;
                break;
            }
            Err(_) => break,
        }
    }

    // Add EOF token
    let eof_offset = original_input.len() - remaining.len();
    let (line, column) = calculate_position(original_input, eof_offset);
    let eof_span = Span::new(eof_offset, eof_offset, line, column);
    tokens.push(TokenWithSpan::new(Token::Eof, eof_span));

    Ok((remaining, tokens))
}
