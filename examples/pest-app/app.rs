// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate pest;

use std::collections::HashMap;
use std::{env, fs};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::{state, ParseResult, Parser, ParserState, Span};

#[derive(Debug, PartialEq)]
enum Json<'i> {
    Null,
    Bool(bool),
    Number(f64),
    String(Span<'i>),
    Array(Vec<Json<'i>>),
    Object(HashMap<Span<'i>, Json<'i>>),
}

impl<'i> Json<'i> {
    fn parse(input: &'i str) -> Result<Self, Error<Rule>> {
        let json = Self::consume(JsonParser::parse(Rule::json, input)?.next().unwrap());
        Ok(json)
    }

    fn consume(pair: Pair<'i, Rule>) -> Self {
        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::null => Self::Null,
            Rule::bool => match pair.as_str() {
                "false" => Self::Bool(false),
                "true" => Self::Bool(true),
                _ => unreachable!(),
            },
            Rule::number => Self::Number(pair.as_str().parse().unwrap()),
            Rule::string => Self::String(pair.as_span()),
            Rule::array => Self::Array(pair.into_inner().map(Self::consume).collect()),
            Rule::object => {
                let pairs = pair.into_inner().map(|pos| {
                    let mut pair = pos.into_inner();

                    let key = pair.next().unwrap().as_span();
                    let value = Self::consume(pair.next().unwrap());

                    (key, value)
                });

                Self::Object(pairs.collect())
            }
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Rule {
    json,
    object,
    pair,
    array,
    value,
    string,
    escape,
    unicode,
    hex,
    number,
    int,
    exp,
    bool,
    null,
}

struct JsonParser;

impl Parser<Rule> for JsonParser {
    fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        fn json(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            value(state)
        }

        fn object(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::object, |s| {
                s.sequence(|s| {
                    s.match_string("{")
                        .and_then(|s| skip(s))
                        .and_then(|s| pair(s))
                        .and_then(|s| skip(s))
                        .and_then(|s| {
                            s.repeat(|s| {
                                s.sequence(|s| {
                                    s.match_string(",")
                                        .and_then(|s| skip(s))
                                        .and_then(|s| pair(s))
                                        .and_then(|s| skip(s))
                                })
                            })
                        })
                        .and_then(|s| s.match_string("}"))
                })
                .or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("{")
                            .and_then(|s| skip(s))
                            .and_then(|s| s.match_string("}"))
                    })
                })
            })
        }

        fn pair(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::pair, |s| {
                s.sequence(|s| {
                    string(s)
                        .and_then(|s| skip(s))
                        .and_then(|s| s.match_string(":"))
                        .and_then(|s| skip(s))
                        .and_then(|s| value(s))
                })
            })
        }

        fn array(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::array, |s| {
                s.sequence(|s| {
                    s.match_string("[")
                        .and_then(|s| skip(s))
                        .and_then(|s| value(s))
                        .and_then(|s| skip(s))
                        .and_then(|s| {
                            s.repeat(|s| {
                                s.sequence(|s| {
                                    s.match_string(",")
                                        .and_then(|s| skip(s))
                                        .and_then(|s| value(s))
                                        .and_then(|s| skip(s))
                                })
                            })
                        })
                        .and_then(|s| s.match_string("]"))
                })
                .or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("[")
                            .and_then(|s| skip(s))
                            .and_then(|s| s.match_string("]"))
                    })
                })
            })
        }

        fn value(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::value, |s| {
                string(s)
                    .or_else(|s| number(s))
                    .or_else(|s| object(s))
                    .or_else(|s| array(s))
                    .or_else(|s| bool(s))
                    .or_else(|s| null(s))
            })
        }

        fn string(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::string, |s| {
                s.match_string("\"")
                    .and_then(|s| {
                        s.repeat(|s| {
                            escape(s).or_else(|s| {
                                s.sequence(|s| {
                                    s.lookahead(false, |s| {
                                        s.match_string("\"").or_else(|s| s.match_string("\\"))
                                    })
                                    .and_then(|s| s.skip(1))
                                })
                            })
                        })
                    })
                    .and_then(|pos| pos.match_string("\""))
            })
        }

        fn escape(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.sequence(|s| {
                s.match_string("\\").and_then(|s| {
                    s.match_string("\"")
                        .or_else(|s| s.match_string("\\"))
                        .or_else(|s| s.match_string("/"))
                        .or_else(|s| s.match_string("b"))
                        .or_else(|s| s.match_string("f"))
                        .or_else(|s| s.match_string("n"))
                        .or_else(|s| s.match_string("r"))
                        .or_else(|s| s.match_string("t"))
                        .or_else(|s| unicode(s))
                })
            })
        }

        fn unicode(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.sequence(|s| {
                s.match_string("u")
                    .and_then(|s| hex(s))
                    .and_then(|s| hex(s))
                    .and_then(|s| hex(s))
            })
        }

        fn hex(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state
                .match_range('0'..'9')
                .or_else(|s| s.match_range('a'..'f'))
                .or_else(|s| s.match_range('A'..'F'))
        }

        fn number(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::number, |s| {
                s.sequence(|s| {
                    s.optional(|s| s.match_string("-"))
                        .and_then(|s| int(s))
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    s.match_string(".")
                                        .and_then(|s| s.match_range('0'..'9'))
                                        .and_then(|s| s.repeat(|s| s.match_range('0'..'9')))
                                        .and_then(|s| s.optional(|s| exp(s)))
                                        .or_else(|s| exp(s))
                                })
                            })
                        })
                })
            })
        }

        fn int(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.match_string("0").or_else(|s| {
                s.sequence(|s| {
                    s.match_range('1'..'9')
                        .and_then(|s| s.repeat(|s| s.match_range('0'..'9')))
                })
            })
        }

        fn exp(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.sequence(|s| {
                s.match_string("E")
                    .or_else(|s| s.match_string("e"))
                    .and_then(|s| {
                        s.optional(|s| s.match_string("+").or_else(|s| s.match_string("-")))
                    })
                    .and_then(|s| int(s))
            })
        }

        fn bool(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::bool, |s| {
                s.match_string("true").or_else(|s| s.match_string("false"))
            })
        }

        fn null(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::null, |s| s.match_string("null"))
        }

        fn skip(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.repeat(|s| {
                s.match_string(" ")
                    .or_else(|s| s.match_string("\t"))
                    .or_else(|s| s.match_string("\r"))
                    .or_else(|s| s.match_string("\n"))
            })
        }

        state(input, |state| match rule {
            Rule::json => json(state),
            Rule::object => object(state),
            Rule::pair => pair(state),
            Rule::array => array(state),
            Rule::value => value(state),
            Rule::string => string(state),
            Rule::escape => escape(state),
            Rule::unicode => unicode(state),
            Rule::hex => hex(state),
            Rule::number => number(state),
            Rule::int => int(state),
            Rule::exp => exp(state),
            Rule::bool => bool(state),
            Rule::null => null(state),
        })
    }
}

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    match Json::parse(&src) {
        Ok(json) => {
            println!("{:#?}", json);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
}
