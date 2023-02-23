peg::parser!(pub grammar parser() for str {
// JSON grammar (RFC 4627). Note that this only checks for valid JSON and does not build a syntax
// tree.

pub rule json() = _ (object() / array()) _

rule _() = [' ' | '\t' | '\r' | '\n']*
rule value_separator() = _ "," _

rule value()
    = "false" / "true" / "null" / object() / array() / number() / string()

rule object()
    = "{" _ member() ** value_separator() _ "}"

rule member()
    = string() _ ":" _ value()

rule array()
    = "[" _ (value() ** value_separator()) _ "]"

// note: escaped chars not handled
rule string()
    = "\"" (!"\"" [_])* "\""

rule number()
    = "-"? int() frac()? exp()? {}

rule int()
    = ['0'] / ['1'..='9']['0'..='9']*

rule exp()
    = ("e" / "E") ("-" / "+")? ['0'..='9']*<1,>

rule frac()
    = "." ['0'..='9']*<1,>
});
