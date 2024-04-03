use std::{borrow::Cow, collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

peg::parser!(pub grammar parser() for str {

pub rule json() -> JsonValue
    = _ value:value() _ { value }

rule _() = [' ' | '\t' | '\r' | '\n']*
rule value_separator() = _ "," _

rule value() -> JsonValue
    = boolean() / null() / object() / array() / number() / string()

rule null() -> JsonValue
    = "null" { JsonValue::Null }

rule boolean() -> JsonValue
    = "true" { JsonValue::Boolean(true) }
    / "false" { JsonValue::Boolean(false) }

rule object() -> JsonValue
    = "{" _ elements:(member() ** value_separator()) _ "}" {
        JsonValue::Object(elements.into_iter().collect())
    }

rule member() -> (String, JsonValue)
    = key:raw_string() _ ":" _ value:value() { (key, value) }

rule array() -> JsonValue
    = "[" _ elements:(value() ** value_separator()) _ "]" {
        JsonValue::Array(elements)
    }

rule string() -> JsonValue
    = value:raw_string() { JsonValue::Str(value) }

rule raw_string() -> String
    = "\"" slices:string_slice()* "\"" { slices.concat() }

/// A substring of same-kind (escaped or unescaped) characters
rule string_slice() -> Cow<'input, str>
    = value:string_characters() { Cow::Borrowed(value) }
    / value:string_escapes() { Cow::Owned(value.into_iter().collect()) }

/// A substring of unescaped characters
rule string_characters() -> &'input str
    = $([^ '\"' | '\\']+)

/// A substring of escaped characters
rule string_escapes() -> Vec<char>
    = ("\\" value:string_escape_char() { value })+

/// Handles a single escape
rule string_escape_char() -> char
    = "\"" { '"' }
    / "\\" { '\\' }
    / "/"  { '/' }
    / "b" { '\x08' }
    / "f" { '\x0C' }
    / "n" { '\n' }
    / "r" { '\r' }
    / "t" { '\t' }
    / "u" digits:$(hex_digit()*<4>) { ?
        let value = u16::from_str_radix(digits, 16).unwrap();
        char::from_u32(value.into()).ok_or("invalid unicode escape")
    }

rule hex_digit()
    = ['0'..='9' | 'a'..='f' | 'A'..='F']

rule number() -> JsonValue
    = "-"? value:$(int() frac()? exp()?) { ?
        Ok(JsonValue::Num(f64::from_str(value).map_err(|_| "invalid number")?))
    }

rule int()
    = ['0'] / ['1'..='9']['0'..='9']*

rule exp()
    = ("e" / "E") ("-" / "+")? ['0'..='9']*<1,>

rule frac()
    = "." ['0'..='9']*<1,>
});
