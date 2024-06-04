%start Object
%expect-unused Unmatched "UNMATCHED"

%%

Object -> Result<Value, Box<dyn Error>>:
    "{" ObjectMembersOpt "}" { Ok(Value::Object(HashMap::from_iter($2?))) }
  ;

ObjectMembersOpt -> Result<Vec<(String, Value)>, Box<dyn Error>>:
    ObjectMembers { $1 }
  | { Ok(Vec::new()) }
  ;

ObjectMembers -> Result<Vec<(String, Value)>, Box<dyn Error>>:
    ObjectMembers "," ObjectMember { flatten($1, $3) }
  | ObjectMember { Ok(vec![$1?]) }
  ;

ObjectMember -> Result<(String, Value), Box<dyn Error>>:
    "STRING" ":" Member {
      let s = $lexer.span_str($1.unwrap().span());
      Ok((s[1..s.len() - 1].to_owned(), $3?))
    }
  ;

Member -> Result<Value, Box<dyn Error>>:
    "[" ArrayMembersOpt "]" { Ok(Value::Array($2?)) }
  | "FALSE" { Ok(Value::Boolean(false)) }
  | "FLOAT" { Ok(Value::Num($lexer.span_str($1?.span()).parse::<f64>().unwrap())) }
  | "NULL" { Ok(Value::Null) }
  | Object { $1 }
  | "STRING" {
      let s = $lexer.span_str($1.unwrap().span());
      Ok(Value::Str(s[1..s.len() - 1].to_owned()))
    }
  | "TRUE" { Ok(Value::Boolean(true)) }
  ;

ArrayMembersOpt -> Result<Vec<Value>, Box<dyn Error>>:
    ArrayMembers { $1 }
  | { Ok(Vec::new()) }
  ;

ArrayMembers -> Result<Vec<Value>, Box<dyn Error>>:
    ArrayMembers "," Member { flatten($1, $3) }
  | Member { Ok(vec![$1?])}
  ;

Unmatched -> ():
  "UNMATCHED" { }
  ;

%%

use crate::json_val::Value;
use std::{collections::HashMap, error::Error};

fn flatten<T>(lhs: Result<Vec<T>, Box<dyn Error>>, rhs: Result<T, Box<dyn Error>>)
  -> Result<Vec<T>, Box<dyn Error>>
{
    let mut lhs = lhs?;
    let rhs = rhs?;
    lhs.push(rhs);
    Ok(lhs)
}
