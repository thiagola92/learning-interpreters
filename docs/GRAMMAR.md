# Grammar

expression -> literal | unary | binary | grouping;

literal -> INTEGER | FLOATING | CHARACTER | STRING | "true" | "false" | "null";

grouping -> "(" expression ")";

unary -> ( "not" | "-" ) expression;

binary -> expression operator expression;

operator -> "==" | "!=" | "<=" | "=>" | "<" | ">"
            | "+" | "-" | "*" | "/" | "%" | "**"
            | "and" | "or"
            | "&" | "|" | "^" | ">>" | "<<"