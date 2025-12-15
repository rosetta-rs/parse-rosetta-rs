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

use bumpalo::Bump;
use logos::{Logos, Span};

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

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

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
// Lifetimes: 'i is the input, 'a is the arena
#[derive(Debug)]
pub enum Value<'i, 'a> {
    /// null.
    Null,
    /// true or false.
    Bool(bool),
    /// Any floating point number.
    Number(f64),
    /// Any quoted string.
    String(&'i str),
    /// An array of values
    Array(&'a [Value<'i, 'a>]),
    /// An dictionary mapping keys and values.
    Object(&'a [(&'i str, Value<'i, 'a>)]),
}

pub fn parse<'i, 'a>(arena: &'a Bump, lexer: &mut JsonLexer<'i>) -> Result<Value<'i, 'a>> {
    Parser {
        arena,
        str_and_value_buf: Vec::new(),
        lexer,
    }
    .parse_value()
}

struct Parser<'i, 'a, 'l> {
    arena: &'a Bump,
    str_and_value_buf: Vec<(&'i str, Value<'i, 'a>)>,
    lexer: &'l mut JsonLexer<'i>,
}

impl<'i, 'a, 'l> Parser<'i, 'a, 'l> {
    /// Parse a token stream into a JSON value.
    fn parse_value(&mut self) -> Result<Value<'i, 'a>> {
        if let Some(token) = self.lexer.next()? {
            match token {
                Token::Bool(b) => Ok(Value::Bool(b)),
                Token::BraceOpen => self.parse_object(),
                Token::BracketOpen => self.parse_array(),
                Token::Null => Ok(Value::Null),
                Token::Number(n) => Ok(Value::Number(n)),
                Token::String(s) => Ok(Value::String(s)),
                _ => Err((
                    "unexpected token here (context: value)".to_owned(),
                    self.lexer.span(),
                )),
            }
        } else {
            Err(("empty values are not allowed".to_owned(), self.lexer.span()))
        }
    }

    /// Parse a token stream into an array and return when
    /// a valid terminator is found.
    ///
    /// > NOTE: we assume '[' was consumed.
    fn parse_array(&mut self) -> Result<Value<'i, 'a>> {
        let stack_len = self.str_and_value_buf.len();
        let span = self.lexer.span();

        let unmatched = || "unmatched opening bracket".to_owned();
        let unexpected = || "unexpected token here (context: array)".to_owned();

        let Some(mut token) = self.lexer.next()? else {
            return Err((unmatched(), self.lexer.span()));
        };

        if matches!(token, Token::BraceClose) {
            return Ok(Value::Array(&[]));
        }

        loop {
            let el = match token {
                Token::Bool(b) => Value::Bool(b),
                Token::BraceOpen => {
                    let object = self.parse_object()?;
                    object
                }
                Token::BracketOpen => {
                    let sub_array = self.parse_array()?;
                    sub_array
                }
                Token::Null => Value::Null,
                Token::Number(n) => Value::Number(n),
                Token::String(s) => Value::String(s),
                _ => return Err((unexpected(), self.lexer.span())),
            };
            self.str_and_value_buf.push(("", el));

            match self.lexer.next()? {
                Some(Token::Comma) => {
                    let Some(t) = self.lexer.next()? else {
                        break;
                    };
                    token = t;
                }
                Some(Token::BracketClose) => return Ok(self.pop_array(stack_len)),
                None => break,
                _ => return Err((unexpected(), self.lexer.span())),
            }
        }
        Err(("unmatched opening bracket defined here".to_owned(), span))
    }

    fn pop_array(&mut self, from: usize) -> Value<'i, 'a> {
        Value::Array(
            self.arena
                .alloc_slice_fill_iter(self.str_and_value_buf.drain(from..).map(|(_, a)| a)),
        )
    }

    fn pop_obj(&mut self, from: usize) -> Value<'i, 'a> {
        Value::Object(
            self.arena
                .alloc_slice_fill_iter(self.str_and_value_buf.drain(from..)),
        )
    }

    /// Parse a token stream into an object and return when
    /// a valid terminator is found.
    ///
    /// > NOTE: we assume '{' was consumed.
    fn parse_object(&mut self) -> Result<Value<'i, 'a>> {
        let stack_len = self.str_and_value_buf.len();
        let span = self.lexer.span();
        let mut awaits_comma = false;
        let mut awaits_key = false;

        while let Some(token) = self.lexer.next()? {
            match token {
                Token::BraceClose if !awaits_key => return Ok(self.pop_obj(stack_len)),
                Token::Comma if awaits_comma => awaits_key = true,
                Token::String(key) if !awaits_comma => {
                    match self.lexer.next()? {
                        Some(Token::Colon) => (),
                        _ => {
                            return Err((
                                "unexpected token here, expecting ':'".to_owned(),
                                self.lexer.span(),
                            ))
                        }
                    }
                    let value = self.parse_value()?;
                    self.str_and_value_buf.push((key, value));
                    awaits_key = false;
                }
                _ => {
                    return Err((
                        "unexpected token here (context: object)".to_owned(),
                        self.lexer.span(),
                    ))
                }
            }
            awaits_comma = !awaits_key;
        }
        Err(("unmatched opening brace defined here".to_owned(), span))
    }
}
