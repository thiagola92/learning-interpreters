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

# Grammar 2.0

expression      -> equality;
equality        -> comparison (("==" | "!=") comparison)*;
comparison      -> term (("<=" | "=>" | "<" | ">") term)*;
term            -> factorization (("+" | "-") factorization)*;
factorization   -> unary (("\*" | "/") unary)*;
unary           -> ("!" | "-") unary | primary;
primary         -> INTEGER | FLOATING | CHARACTER | STRING | "true" | "false" | "null" | "(" expression ")";

## References
https://en.wikipedia.org/wiki/Equality_(mathematics)  
https://en.wikipedia.org/wiki/Inequality_(mathematics)  
https://simple.wikipedia.org/wiki/Term_(mathematics)  
https://en.wikipedia.org/wiki/Factorization  
https://en.wikipedia.org/wiki/Unary_operation  