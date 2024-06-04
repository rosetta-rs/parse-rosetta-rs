%%
"[^"]*" "STRING"
-?(0|([1-9][0-9]*))(\.[0-9]*)?([eE][-+]?[0-9]+)? "FLOAT"
\[ "["
\] "]"
\{ "{"
\} "}"
: ":"
, ","
false "FALSE"
null "NULL"
true "TRUE"
[\n\r\t ]+ ;
. "UNMATCHED"
