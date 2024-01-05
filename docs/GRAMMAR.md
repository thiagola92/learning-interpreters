# Grammar

program                 -> statement* EOF;
statement               -> expression_statement;
expression_statement    -> expression "\n";

expression              -> equality;
equality                -> comparison (("==" | "!=") comparison)*;
comparison              -> term (("<=" | "=>" | "<" | ">") term)*;
term                    -> factorization (("+" | "-" | "&" | "|" | "^") factorization)*;
factorization           -> unary (("\*" | "/" | "%" | "**" | | ">>" | "<<") unary)*;
unary                   -> ("not" | "-" | "~") unary | primary;
primary                 -> INTEGER | FLOATING | CHARACTER | STRING | "true" | "false" | "null" | "(" expression ")";

## References
https://en.wikipedia.org/wiki/Equality_(mathematics)  
https://en.wikipedia.org/wiki/Inequality_(mathematics)  
https://en.wikipedia.org/wiki/Bitwise_operation  
https://simple.wikipedia.org/wiki/Term_(mathematics)  
https://en.wikipedia.org/wiki/Factorization  
https://en.wikipedia.org/wiki/Unary_operation  