use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
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
            "Int" => Token::Int,
            "Bool" => Token::Bool,
            "List" => Token::List,
            "Rec" => Token::Rec,
            "fn" => Token::Fn,
            "fix" => Token::Fix,
            "fst" => Token::Fst,
            "snd" => Token::Snd,
            "cons" => Token::Cons,
            "head" => Token::Head,
            "tail" => Token::Tail,
            "print" => Token::Print,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "in" => Token::In,
            "range" => Token::Range,
            "inl" => Token::Inl,
            "inr" => Token::Inr,
            "case" => Token::Case,
            "of" => Token::Of,
            "true" => Token::True,
            "false" => Token::False,
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

fn parse_arrow(input: &str) -> IResult<&str, Token> {
    value(Token::Arrow, tag("->")).parse(input)
}

fn parse_plus(input: &str) -> IResult<&str, Token> {
    value(Token::Plus, char('+')).parse(input)
}

fn parse_minus(input: &str) -> IResult<&str, Token> {
    value(Token::Minus, char('-')).parse(input)
}

fn parse_multiply(input: &str) -> IResult<&str, Token> {
    value(Token::Multiply, char('*')).parse(input)
}

fn parse_divide(input: &str) -> IResult<&str, Token> {
    value(Token::Divide, char('/')).parse(input)
}

fn parse_left_paren(input: &str) -> IResult<&str, Token> {
    value(Token::LeftParen, char('(')).parse(input)
}

fn parse_right_paren(input: &str) -> IResult<&str, Token> {
    value(Token::RightParen, char(')')).parse(input)
}

fn parse_left_bracket(input: &str) -> IResult<&str, Token> {
    value(Token::LeftBracket, char('[')).parse(input)
}

fn parse_right_bracket(input: &str) -> IResult<&str, Token> {
    value(Token::RightBracket, char(']')).parse(input)
}

fn parse_comma(input: &str) -> IResult<&str, Token> {
    value(Token::Comma, char(',')).parse(input)
}

fn parse_left_brace(input: &str) -> IResult<&str, Token> {
    value(Token::LeftBrace, char('{')).parse(input)
}

fn parse_right_brace(input: &str) -> IResult<&str, Token> {
    value(Token::RightBrace, char('}')).parse(input)
}

fn parse_equal(input: &str) -> IResult<&str, Token> {
    value(Token::Equal, tag("==")).parse(input)
}

fn parse_not_equal(input: &str) -> IResult<&str, Token> {
    value(Token::NotEqual, tag("!=")).parse(input)
}

fn parse_less_than_equal(input: &str) -> IResult<&str, Token> {
    value(Token::LessThanEqual, tag("<=")).parse(input)
}

fn parse_greater_than_equal(input: &str) -> IResult<&str, Token> {
    value(Token::GreaterThanEqual, tag(">=")).parse(input)
}

fn parse_less_than(input: &str) -> IResult<&str, Token> {
    value(Token::LessThan, char('<')).parse(input)
}

fn parse_greater_than(input: &str) -> IResult<&str, Token> {
    value(Token::GreaterThan, char('>')).parse(input)
}

fn parse_logical_and(input: &str) -> IResult<&str, Token> {
    value(Token::LogicalAnd, tag("&&")).parse(input)
}

fn parse_logical_or(input: &str) -> IResult<&str, Token> {
    value(Token::LogicalOr, tag("||")).parse(input)
}

fn parse_logical_not(input: &str) -> IResult<&str, Token> {
    value(Token::LogicalNot, char('!')).parse(input)
}

fn parse_pipe(input: &str) -> IResult<&str, Token> {
    value(Token::Pipe, char('|')).parse(input)
}

fn parse_fat_arrow(input: &str) -> IResult<&str, Token> {
    value(Token::FatArrow, tag("=>")).parse(input)
}

fn parse_operators(input: &str) -> IResult<&str, Token> {
    alt((
        // Multi-character operators must come before their prefixes
        parse_fat_arrow,          // => must come before =
        parse_arrow,              // -> must come before -
        parse_equal,              // == must come before =
        parse_not_equal,          // != must come before !
        parse_less_than_equal,    // <= must come before <
        parse_greater_than_equal, // >= must come before >
        parse_logical_and,        // && must come before individual &
        parse_logical_or,         // || must come before individual |
        // Single character operators
        parse_assign,
        parse_plus,
        parse_minus,
        parse_multiply,
        parse_divide,
        parse_less_than,
        parse_greater_than,
        parse_logical_not,
        parse_pipe,
    ))
    .parse(input)
}

fn parse_punctuation(input: &str) -> IResult<&str, Token> {
    alt((
        parse_semicolon,
        parse_colon,
        parse_left_paren,
        parse_right_paren,
        parse_left_bracket,
        parse_right_bracket,
        parse_left_brace,
        parse_right_brace,
        parse_comma,
    ))
    .parse(input)
}

fn parse_single_line_comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    Ok((input, ()))
}

fn parse_multi_line_comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("/*")(input)?;
    let mut remaining = input;

    loop {
        if remaining.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }

        if remaining.starts_with("*/") {
            let (rest, _) = tag("*/")(remaining)?;
            return Ok((rest, ()));
        }

        remaining = &remaining[1..];
    }
}

fn parse_comment(input: &str) -> IResult<&str, ()> {
    alt((parse_single_line_comment, parse_multi_line_comment)).parse(input)
}

fn parse_single_token(input: &str) -> IResult<&str, Token> {
    alt((
        parse_operators,
        parse_identifier_or_keyword,
        parse_number,
        parse_punctuation,
    ))
    .parse(input)
}

fn skip_whitespace_and_comments(input: &str) -> IResult<&str, &str> {
    let mut remaining = input;

    loop {
        let start_len = remaining.len();

        // Skip whitespace
        let (after_ws, _) = multispace0(remaining)?;
        remaining = after_ws;

        // Try to skip a comment
        if let Ok((after_comment, _)) = parse_comment(remaining) {
            remaining = after_comment;
        }

        // If nothing was consumed, we're done
        if remaining.len() == start_len {
            break;
        }
    }

    Ok((remaining, &input[..input.len() - remaining.len()]))
}

fn parse_token_with_whitespace<'a>(
    input: &'a str,
    original_input: &str,
    offset: usize,
) -> IResult<&'a str, Option<TokenWithSpan>> {
    let (input_after_ws, _) = skip_whitespace_and_comments(input)?;

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
