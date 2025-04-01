//! This is a parser for JSON.
//! Run it with the following command:
//! cargo run --example json -- examples/sample.json

use chumsky::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Json {
    Invalid,
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Json> {
    recursive(|value| {
        let digits = one_of('0'..='9').repeated();

        let int = one_of('1'..='9')
            .then(one_of('0'..='9').repeated())
            .ignored()
            .or(just('0').ignored())
            .ignored();

        let frac = just('.').then(digits.clone());

        let exp = one_of("eE")
            .then(one_of("+-").or_not())
            .then(digits.clone());

        let number = just('-')
            .or_not()
            .then(int)
            .then(frac.or_not())
            .then(exp.or_not())
            .to_slice()
            .map(|s: &str| s.parse().unwrap());

        let escape = just('\\').then_ignore(choice((
            just('\\'),
            just('/'),
            just('"'),
            just('b').to('\x08'),
            just('f').to('\x0C'),
            just('n').to('\n'),
            just('r').to('\r'),
            just('t').to('\t'),
            just('u').ignore_then(text::digits(16).exactly(4).to_slice().try_map(|digits, _| {
                char::from_u32(u32::from_str_radix(digits, 16).unwrap())
                    .ok_or_else(Default::default)
            })),
        )));

        let string = none_of("\\\"")
            .or(escape)
            .repeated()
            .collect()
            .delimited_by(just('"'), just('"'));

        let array = value
            .clone()
            .separated_by(just(','))
            .collect()
            .padded()
            .delimited_by(just('['), just(']'));

        let member = string.clone().then_ignore(just(':').padded()).then(value);
        let object = member
            .clone()
            .separated_by(just(',').padded())
            .collect()
            .padded()
            .delimited_by(just('{'), just('}'));

        choice((
            just("null").to(Json::Null),
            just("true").to(Json::Bool(true)),
            just("false").to(Json::Bool(false)),
            number.map(Json::Num),
            string.map(Json::Str),
            array.map(Json::Array),
            object.map(Json::Object),
        ))
        .padded()
    })
}
