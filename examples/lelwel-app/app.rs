mod lexer;
mod parser;

use std::{env, fs};

use codespan_reporting::diagnostic::Severity;
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Config};
use lexer::Token;
use parser::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Cst<'_> {
    pub fn to_value(&self, node_ref: NodeRef) -> Option<Value> {
        match self.get(node_ref) {
            Node::Rule(rule, _) => match rule {
                Rule::File => self
                    .children(node_ref)
                    .find_map(|child_node_ref| self.to_value(child_node_ref)),
                Rule::Literal => self.to_value(self.children(node_ref).next()?),
                Rule::Array => Some(Value::Array(
                    self.children(node_ref)
                        .filter_map(|child_node_ref| self.to_value(child_node_ref))
                        .collect(),
                )),
                Rule::Object => {
                    let mut members = HashMap::new();
                    for mut member_node_refs in self
                        .children(node_ref)
                        .filter(|&child_node_ref| self.match_rule(child_node_ref, Rule::Member))
                        .map(|child_node_ref| self.children(child_node_ref))
                    {
                        let Some(key) = member_node_refs
                            .find_map(|member_node_ref| {
                                self.match_token(member_node_ref, Token::String)
                            })
                            .map(|(key_str, _)| key_str[1..key_str.len() - 1].to_owned())
                        else {
                            continue;
                        };
                        let Some(val) = member_node_refs
                            .find_map(|member_node_ref| self.to_value(member_node_ref))
                        else {
                            continue;
                        };
                        members.insert(key, val);
                    }
                    Some(Value::Object(members))
                }
                _ => None,
            },
            Node::Token(token, idx) => match token {
                Token::String => {
                    let val = self.span_text(idx);
                    Some(Value::String(val[1..val.len() - 1].to_owned()))
                }
                Token::Number => Some(Value::Number(str::parse(self.span_text(idx)).ok()?)),
                Token::True => Some(Value::Bool(true)),
                Token::False => Some(Value::Bool(false)),
                Token::Null => Some(Value::Null),
                _ => None,
            },
        }
    }
}

fn main() {
    let path = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&path).expect("Failed to read file");

    let mut diags = vec![];
    let cst = Parser::new(&src, &mut diags).parse(&mut diags);
    let json = cst.to_value(NodeRef::ROOT);

    #[cfg(debug_assertions)]
    {
        println!("{:#?}", json);
    }
    #[cfg(not(debug_assertions))]
    {
        std::hint::black_box(json);
    }

    if !diags.is_empty() {
        let writer = StandardStream::stderr(ColorChoice::Auto);
        let config = Config::default();
        let file = SimpleFile::new(&path, &src);
        for diag in diags.iter() {
            term::emit_to_write_style(&mut writer.lock(), &config, &file, diag).unwrap();
        }
        if diags.iter().any(|d| d.severity == Severity::Error) {
            std::process::exit(1);
        }
    }
}
