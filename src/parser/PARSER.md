# Parser
Responsible for creating the AST (abstract syntax tree).

# Grammar

### Declarations
```
program                 -> declaration* EOF;
declaration             -> var | var_ass | statement;
var                     -> "var" IDENTIFIER ("=" expression)? "\n";
```

### Statements
```
statement               -> expr | print;
print                   -> "print" expression "\n";
expr                    -> expression "\n";
```

### Expressions
```
assignment              -> IDENTIFIER ("=" | "+=" | "-=" | "*=" | "/=" | "%=" | "**=" | "&=" | "|=" | "^=" | ">>=" | "<<=") expression;
expression              -> equality;
equality                -> comparison (("==" | "!=") comparison)*;
comparison              -> term (("<=" | "=>" | "<" | ">") term)*;
term                    -> factorization (("+" | "-" | "&" | "|" | "^") factorization)*;
factorization           -> unary (("\*" | "/" | "%" | "**" | | ">>" | "<<") unary)*;
unary                   -> ("-" | "not" | "!") unary | primary;
primary                 -> INTEGER | FLOATING | CHARACTER | STRING | BOOLEAN | "null" | "(" expression ")" | IDENTIFIER;
```

# References
- https://en.wikipedia.org/wiki/Equality_(mathematics)
- https://en.wikipedia.org/wiki/Inequality_(mathematics)
- https://en.wikipedia.org/wiki/Bitwise_operation
- https://simple.wikipedia.org/wiki/Term_(mathematics)
- https://en.wikipedia.org/wiki/Factorization
- https://en.wikipedia.org/wiki/Unary_operation