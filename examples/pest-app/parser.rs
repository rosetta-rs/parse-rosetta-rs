// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::HashMap;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

#[derive(Debug, PartialEq)]
pub enum Json<'i> {
    Null,
    Bool(bool),
    Number(f64),
    String(&'i str),
    Array(Vec<Json<'i>>),
    Object(HashMap<&'i str, Json<'i>>),
}

pub fn parse_json_file(input: &str) -> Result<Json<'_>, Error<Rule>> {
    use pest::iterators::Pair;

    let json = JSONParser::parse(Rule::json, input)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> Json {
        match pair.as_rule() {
            Rule::object => Json::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => Json::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => Json::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => Json::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => Json::Bool(pair.as_str().parse().unwrap()),
            Rule::null => Json::Null,
            Rule::json
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(json))
}
