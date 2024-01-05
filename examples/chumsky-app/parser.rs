//! This is a parser for JSON.
//! Run it with the following command:
//! cargo run --example json -- examples/sample.json

use chumsky::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

pub fn parser<'a>() -> impl Parser<'a, &'a [u8], Json, extra::Err<Cheap>> {
    recursive(|value| {
        let digits = one_of(b'0'..=b'9').repeated();

        let int = one_of(b'1'..=b'9')
            .then(one_of(b'0'..=b'9').repeated())
            .ignored()
            .or(just(b'0').ignored())
            .ignored();

        let frac = just(b'.').then(digits.clone());

        let exp = one_of(b"eE")
            .then(one_of(b"+-").or_not())
            .then(digits.clone());

        let number = just(b'-')
            .or_not()
            .then(int)
            .then(frac.or_not())
            .then(exp.or_not())
            .to_slice()
            .map(|bytes| std::str::from_utf8(bytes).unwrap().parse().unwrap())
            .boxed();

        let escape = just(b'\\').then_ignore(one_of(b"\\/\"bfnrt"));

        let string = none_of(b"\\\"")
            .or(escape)
            .repeated()
            .to_slice()
            .delimited_by(just(b'"'), just(b'"'))
            .map(|bytes| std::str::from_utf8(bytes).unwrap().to_owned())
            .boxed();

        let array = value
            .clone()
            .separated_by(just(b','))
            .collect()
            .padded()
            .delimited_by(just(b'['), just(b']'))
            .boxed();

        let member = string.clone().then_ignore(just(b':').padded()).then(value);
        let object = member
            .clone()
            .separated_by(just(b',').padded())
            .collect()
            .padded()
            .delimited_by(just(b'{'), just(b'}'))
            .boxed();

        choice((
            just(b"null").to(Json::Null),
            just(b"true").to(Json::Bool(true)),
            just(b"false").to(Json::Bool(false)),
            number.map(Json::Num),
            string.map(Json::Str),
            array.map(Json::Array),
            object.map(Json::Object),
        ))
        .padded()
    })
}
