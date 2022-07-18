#[macro_use]
extern crate combine;

use std::collections::HashMap;
use std::{env, fs};

use combine::error::ParseError;
use combine::{Parser, RangeStream, StreamOnce};

use combine::parser::byte::{byte, spaces};
use combine::parser::choice::{choice, optional};
use combine::parser::combinator::no_partial;
use combine::parser::item::{one_of, satisfy_map};
use combine::parser::range;
use combine::parser::repeat::{escaped, sep_by};
use combine::parser::sequence::between;

#[derive(PartialEq, Debug)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}

#[inline(always)]
fn json_value<'a, I>() -> impl Parser<Input = I, Output = Value> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    spaces().with(json_value_())
}

// We need to use `parser!` to break the recursive use of `value` to prevent the returned parser
// from containing itself
parser! {
    #[inline(always)]
    fn json_value_['a, I]()(I) -> Value
        where [ I: RangeStream<Item = u8, Range = &'a [u8]> + 'a ]
    {
        choice((
            json_string().map(Value::String),
            object().map(Value::Object),
            array().map(Value::Array),
            number().map(Value::Number),
            lex(range::range(&b"false"[..]).map(|_| Value::Bool(false))),
            lex(range::range(&b"true"[..]).map(|_| Value::Bool(true))),
            lex(range::range(&b"null"[..]).map(|_| Value::Null)),
        ))
    }
}

fn object<'a, I>() -> impl Parser<Input = I, Output = HashMap<String, Value>> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let field = (json_string(), lex(byte(b':')), json_value_()).map(|t| (t.0, t.2));
    let fields = sep_by(field, lex(byte(b',')));
    between(lex(byte(b'{')), lex(byte(b'}')), fields).expected("object")
}

fn array<'a, I>() -> impl Parser<Input = I, Output = Vec<Value>> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        lex(byte(b'[')),
        lex(byte(b']')),
        sep_by(json_value_(), lex(byte(b','))),
    )
    .expected("array")
}

fn json_string<'a, I>() -> impl Parser<Input = I, Output = String> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let back_slash_byte = satisfy_map(|c| {
        Some(match c {
            b'"' => b'"',
            b'\\' => b'\\',
            b'/' => b'/',
            b'b' => '\u{0008}' as u8,
            b'f' => '\u{000c}' as u8,
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            _ => return None,
        })
    });
    let inner = range::recognize(escaped(
        range::take_while1(|b| b != b'\\' && b != b'"'),
        b'\\',
        back_slash_byte,
    ))
    .map(|s| std::str::from_utf8(s).unwrap().to_owned());
    between(byte(b'"'), lex(byte(b'"')), inner).expected("string")
}

fn number<'a, I>() -> impl Parser<Input = I, Output = f64> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    no_partial(
        lex(range::recognize(no_partial((
            optional(one_of("+-".bytes())),
            byte(b'0').or((digits(), optional((byte(b'.'), digits()))).map(|_| b'0')),
            optional((
                (one_of("eE".bytes()), optional(one_of("+-".bytes()))),
                digits(),
            )),
        ))))
        .map(|s: &'a [u8]| std::str::from_utf8(s).unwrap().parse().unwrap())
        .expected("number"),
    )
}

fn digits<'a, I>() -> impl Parser<Input = I, Output = &'a [u8]> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range::take_while1(|b| b >= b'0' && b <= b'9')
}

fn lex<'a, P>(p: P) -> impl Parser<Input = P::Input, Output = P::Output>
where
    P: Parser,
    P::Input: RangeStream<Item = u8, Range = &'a [u8]>,
    <P::Input as StreamOnce>::Error: ParseError<
        <P::Input as StreamOnce>::Item,
        <P::Input as StreamOnce>::Range,
        <P::Input as StreamOnce>::Position,
    >,
{
    no_partial(p.skip(range::take_while(|b| {
        b == b' ' || b == b'\t' || b == b'\r' || b == b'\n'
    })))
}

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let mut parser = json_value();
    match parser.easy_parse(src.as_bytes()) {
        Ok(json) => {
            println!("{:#?}", json);
        }
        Err(err) => {
            eprintln!("{:#?}", err);
            std::process::exit(1);
        }
    };
}
