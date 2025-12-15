//! JSON parser written in Rust, using Logos.
//!
//! If the file is a valid JSON value, it will be printed
//! to the terminal using the debug format.
//!
//! Otherwise, an error will be printed with its location.
//!
//! Usage:
//!     cargo run --example json <path/to/file>
//!
//! Example:
//!     cargo run --example json examples/example.json

use logos::{Logos, Span};

use std::collections::HashMap;

type Error = (String, Span);

type Result<T> = std::result::Result<T, Error>;

/// All meaningful JSON tokens.
///
/// > NOTE: regexes for [`Token::Number`] and [`Token::String`] may not
/// > catch all possible values, especially for strings. If you find
/// > errors, please report them so that we can improve the regex.
#[derive(Debug, Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token<'a> {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("null")]
    Null,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice())]
    Number(&'a str),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice())]
    String(&'a str),
}

pub struct JsonLexer<'a>(pub logos::Lexer<'a, Token<'a>>);

impl<'a> JsonLexer<'a> {
    #[inline(always)]
    fn next(&mut self) -> Result<Option<Token<'a>>> {
        // This can also be implemented in terms of .transpose().map_err()...
        match self.0.next() {
            Some(Err(())) => Err(("Lex failed".to_owned(), self.span())),
            Some(Ok(a)) => Ok(Some(a)),
            None => Ok(None),
        }
    }

    #[cold]
    fn span(&self) -> Span {
        self.0.span()
    }
}

/// Represent any valid JSON value.
#[derive(Debug)]
pub enum Value<'a> {
    /// null.
    Null,
    /// true or false.
    Bool(bool),
    /// Any floating point number.
    Number(&'a str),
    /// Any quoted string.
    String(&'a str),
    /// An array of values
    Array(Vec<Value<'a>>),
    /// An dictionary mapping keys and values.
    Object(HashMap<&'a str, Value<'a>>),
}

pub fn parse_value<'a>(lexer: &mut JsonLexer<'a>) -> Result<Value<'a>> {
    if let Some(token) = lexer.next()? {
        match token {
            Token::Bool(b) => Ok(Value::Bool(b)),
            Token::BraceOpen => parse_object(lexer),
            Token::BracketOpen => parse_array(lexer),
            Token::Null => Ok(Value::Null),
            Token::Number(n) => Ok(Value::Number(n)),
            Token::String(s) => Ok(Value::String(s)),
            _ => Err((
                "unexpected token here (context: value)".to_owned(),
                lexer.span(),
            )),
        }
    } else {
        Err(("empty values are not allowed".to_owned(), lexer.span()))
    }
}

/// Parse a token stream into an array and return when
/// a valid terminator is found.
///
/// > NOTE: we assume '[' was consumed.
fn parse_array<'a>(lexer: &mut JsonLexer<'a>) -> Result<Value<'a>> {
    let mut array = Vec::new();
    let span = lexer.span();

    let unmatched = || "unmatched opening bracket".to_owned();
    let unexpected = || "unexpected token here (context: array)".to_owned();

    let Some(mut token) = lexer.next()? else {
        return Err((unmatched(), lexer.span()));
    };

    if matches!(token, Token::BraceClose) {
        return Ok(Value::Array(array));
    }

    loop {
        match token {
            Token::Bool(b) => {
                array.push(Value::Bool(b));
            }
            Token::BraceOpen => {
                let object = parse_object(lexer)?;
                array.push(object);
            }
            Token::BracketOpen => {
                let sub_array = parse_array(lexer)?;
                array.push(sub_array);
            }
            Token::Null => {
                array.push(Value::Null);
            }
            Token::Number(n) => {
                array.push(Value::Number(n));
            }
            Token::String(s) => {
                array.push(Value::String(s));
            }
            _ => return Err((unexpected(), lexer.span())),
        }

        match lexer.next()? {
            Some(Token::Comma) => {
                let Some(t) = lexer.next()? else {
                    break;
                };
                token = t;
            }
            Some(Token::BracketClose) => return Ok(Value::Array(array)),
            None => break,
            _ => return Err((unexpected(), lexer.span())),
        }
    }
    Err(("unmatched opening bracket defined here".to_owned(), span))
}

/// Parse a token stream into an object and return when
/// a valid terminator is found.
///
/// > NOTE: we assume '{' was consumed.
fn parse_object<'a>(lexer: &mut JsonLexer<'a>) -> Result<Value<'a>> {
    let mut map = HashMap::new();
    let span = lexer.span();
    let mut awaits_comma = false;
    let mut awaits_key = false;

    while let Some(token) = lexer.next()? {
        match token {
            Token::BraceClose if !awaits_key => return Ok(Value::Object(map)),
            Token::Comma if awaits_comma => awaits_key = true,
            Token::String(key) if !awaits_comma => {
                match lexer.next()? {
                    Some(Token::Colon) => (),
                    _ => {
                        return Err((
                            "unexpected token here, expecting ':'".to_owned(),
                            lexer.span(),
                        ))
                    }
                }
                let value = parse_value(lexer)?;
                map.insert(key, value);
                awaits_key = false;
            }
            _ => {
                return Err((
                    "unexpected token here (context: object)".to_owned(),
                    lexer.span(),
                ))
            }
        }
        awaits_comma = !awaits_key;
    }
    Err(("unmatched opening brace defined here".to_owned(), span))
}
