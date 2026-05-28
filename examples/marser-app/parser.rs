use std::rc::Rc;

use marser::capture;
use marser::{
    error::{AnnotationKind, FurthestFailError, InlineError},
    label::WithLabel,
    matcher::{
        AnyToken, MatcherCombinator,
        commit_matcher::commit_on,
        if_error::{if_error, if_error_else_fail},
        multiple::many,
        negative_lookahead,
        one_or_more::one_or_more,
        optional::optional,
        positive_lookahead, unwanted,
    },
    one_of::one_of,
    parser::{Parser, ParserCombinator, deferred::recursive, token_parser::TokenParser},
    trace::WithTrace,
};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue<'src> {
    Invalid(&'src str),
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue<'src>>),
    Object(Vec<(String, JsonValue<'src>)>),
}

impl<'src> JsonValue<'src> {
    /// Public method for pretty-printed JSON
    pub fn serialize_pretty(&self) -> String {
        self.serialize_internal(0)
    }

    fn serialize_internal(&self, indent_level: usize) -> String {
        let indent_size = 4;
        let current_indent = " ".repeat(indent_level * indent_size);
        let nested_indent = " ".repeat((indent_level + 1) * indent_size);

        match self {
            Self::Invalid(slice) => format!("invalid('{slice}')"),
            Self::Null => "null".to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Number(n) => n.to_string(),
            Self::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),

            Self::Array(arr) => {
                if arr.is_empty() {
                    return "[]".to_string();
                }
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| {
                        format!(
                            "{}{}",
                            nested_indent,
                            v.serialize_internal(indent_level + 1)
                        )
                    })
                    .collect();
                format!("[\n{}\n{current_indent}]", items.join(",\n"))
            }

            Self::Object(obj) => {
                if obj.is_empty() {
                    return "{}".to_string();
                }
                // Note: HashMap iteration order is random.
                // For deterministic output, you could collect and sort keys here.
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{}\"{}\": {}",
                            nested_indent,
                            k,
                            v.serialize_internal(indent_level + 1)
                        )
                    })
                    .collect();
                format!("{{\n{}\n{current_indent}}}", pairs.join(",\n"))
            }
        }
    }
}

pub fn get_json_grammar<'src>() -> impl Parser<'src, &'src str, Output = JsonValue<'src>> + Clone {
    recursive(|element| {
        let ws = Rc::new(many(
            one_of((' ', '\t', '\n', '\r')).with_label("whitespace"),
        ));

        let null = capture!(("null", ws.clone()) => JsonValue::Null).with_label("null");
        let bool_false =
            capture!(("false", ws.clone()) => JsonValue::Boolean(false)).with_label("false");
        let bool_true =
            capture!(("true", ws.clone()) => JsonValue::Boolean(true)).with_label("true");
        let boolean = one_of((bool_true, bool_false)).with_label("boolean");

        let invalid_element = capture!(
            if_error_else_fail(bind_slice!(unwanted(one_or_more(
                (
                    negative_lookahead(one_of((
                        '{',
                        '[',
                        '"',
                        ',',
                        ']',
                        '}',
                        ':',
                        // not just using ws, because that can return sucess by consuming 0 tokens
                        one_of((' ', '\t', '\n', '\r'))
                    ))),
                    AnyToken
                )
            ), "invalid element"), slice),
            ) => JsonValue::Invalid(slice)
        )
        .with_label("invalid element")
        .erase_types();

        let number = capture!(
            commit_on(positive_lookahead(one_of(('-', '.', '+', '0'..='9'))),
            bind_slice!((
                optional('-'),
                one_of((
                    '0',
                    ('1'..='9',many('0'..='9'))
                )),
                optional((
                    '.', one_or_more('0'..='9')
                )),
                optional((
                    one_of(('e', 'E')),
                    optional(one_of(('+', '-'))),
                    one_or_more('0'..='9')
                )),
                negative_lookahead(one_of(('+','-','0'..='9','.','e','E')))
            ), slice as &'src str))
             => {
                JsonValue::Number(slice.parse().unwrap_or(0.0))
            }
        )
        .add_error_info(one_of((
            capture!(
                (
                    optional('-'),
                    bind_span!('0', zero),
                    '0'..='9'
                )
                => Box::new(move |err: &mut FurthestFailError|{
                    err.add_annotation(
                        zero,
                        "leading zero",
                        AnnotationKind::Context,
                    )
                    .add_note("Leading zeros are not allowed in JSON numbers".to_string());
                }) as Box<dyn Fn(&mut FurthestFailError)>
            ),
            capture!(
                (
                    optional('-'),
                    bind_span!('.', dot),
                )
                => Box::new(move |err: &mut FurthestFailError|{
                    err.add_annotation(
                        dot,
                        "missing integer part",
                        AnnotationKind::Context,
                    )
                    .add_note("Floating point numbers need an integer part".to_string());
            }) as Box<dyn Fn(&mut FurthestFailError)>
            ),
        )))
        .recover_with(
            invalid_element.clone()
        )
        .with_label("number");

        let character = Rc::new(
            TokenParser::new(|c| *c != '"' && *c != '\\' && (*c as u32) >= 0x20, |x| *x)
                .with_label("string character"),
        );
        let hex_digit = Rc::new(one_of(('0'..='9', 'a'..='f', 'A'..='F')).with_label("hex digit"));
        let escaped_char = capture!({
            (
                '\\',
                bind!(one_of(('\"', '\\', '/', 'b', 'f', 'n', 'r', 't')), esc)
            )
        } => {
            match esc {
                '"' => '"',
                '\\' => '\\',
                '/' => '/',
                'b' => '\u{0008}',
                'f' => '\u{000C}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                _ => esc,
            }
        })
        .with_label("escape sequence");
        let unicode_escape = capture!({
            (
                '\\', 'u',
                bind!(hex_digit.clone(), *digits),
                bind!(hex_digit.clone(), *digits),
                bind!(hex_digit.clone(), *digits),
                bind!(hex_digit.clone(), *digits)
            )
        } => {
            let hex: String = digits.into_iter().collect();
            let codepoint = u32::from_str_radix(&hex, 16).unwrap_or(0xFFFD);
            std::char::from_u32(codepoint).unwrap_or('\u{FFFD}')
        })
        .with_label("unicode escape");
        let raw_string = Rc::new(capture!({
            commit_on(
                bind_span!('"', open_quote_span as (usize, usize)),
                (
                many(one_of((
                    bind!(character.clone(), *chars),
                    bind!(escaped_char, *chars),
                    bind!(unicode_escape, *chars),
                ))),
                '"'.err_if_no_match(use_binds!(|ctx| {
                    let open_quote_span: Option<(usize, usize)> = open_quote_span.copied();
                    InlineError::new("missing closing quote")
                        .with_span(Some(ctx.span()))
                        .with_annotation(
                            open_quote_span.unwrap(),
                            "quote opened here",
                            AnnotationKind::Context,
                        )
                })),
                ws.clone()
            ))
        } =>  {
            chars.into_iter().collect::<String>()
        }))
        .with_label("quoted string")
        .erase_types();

        let array = capture!({
            commit_on(
                (ws.clone(), bind_span!('[', open_bracket_span as (usize, usize))),
            (
                ws.clone().trace(),
                optional((
                    bind!(element.clone(), *elements).trace(),
                    many((
                        ','.trace().try_insert_if_missing("missing comma"),
                        ws.clone().trace(),
                        if_error(many((unwanted(',', "missing element"), ws.clone())))
                            .trace(),
                        bind!(element.clone(), *elements).trace(),
                        if_error(negative_lookahead(':')).trace()
                    ))
                )),
                ws.clone().trace(),
                if_error(many((unwanted(',', "trailing comma"), ws.clone())))
                    .trace(),
                ']'.err_if_no_match(use_binds!(|ctx| {
                    let open_bracket_span: Option<(usize, usize)> = open_bracket_span.copied();
                    InlineError::new("missing closing ']'")
                        .with_span(Some(ctx.span()))
                        .with_annotation(
                            open_bracket_span.unwrap(),
                            "bracket opened here",
                            AnnotationKind::Context,
                        )
                })),
                ws.clone().trace()
            ))
        } =>  {
            JsonValue::Array(elements)
        })
        .with_label("array")
        .erase_types();

        let key_value_pair = Rc::new(
            capture!({
                (
                bind!(raw_string.clone(), key).trace(),
                ':',
                ws.clone().trace(),
                bind!(element.clone(), value).trace(),
                if_error(optional(invalid_element.clone().ignore_result()))
                )
            } => {
                (key, value)
            })
            .with_label("key-value pair"),
        )
        .erase_types();

        let object = capture!({
                commit_on(
                    (ws.clone(), bind_span!('{', open_brace_span as (usize, usize))),
                (
                ws.clone().trace(),
                optional((
                    bind!(key_value_pair.clone(), *key_value_pairs),
                    many((
                        ','.trace().try_insert_if_missing("missing comma"),
                        ws.clone().trace(),
                        if_error(many((unwanted(',', "missing element"), ws.clone())))
                            .trace(),
                        bind!(key_value_pair.clone(), *key_value_pairs),
                    )),
                    if_error(
                        many((unwanted(',', "trailing comma"), ws.clone()))
                            .trace(),
                    )
                    .trace()
                )),
                '}'.err_if_no_match(use_binds!(|ctx| {
                    let open_brace_span: Option<(usize, usize)> = open_brace_span.copied();
                    InlineError::new("missing closing '}'")
                        .with_span(Some(ctx.span()))
                        .with_annotation(
                            open_brace_span.unwrap(),
                            "brace opened here",
                            AnnotationKind::Context,
                        )
                })),
                ws.clone().trace()
                )
                )
        } => {
            JsonValue::Object(key_value_pairs)
        })
        .with_label("object")
        .erase_types();

        let string = raw_string
            .map_output(JsonValue::String)
            .with_label("string");



        capture!((
            ws.clone().trace(),
            bind!(one_of((
                object.trace(),
                array.trace(),
                string.trace(),
                number.trace(),
                boolean.trace(),
                null.trace(),
                invalid_element.trace()
            )), result),
            ws.clone().trace()
        ) => result)
        .with_label("element")
    })
}
