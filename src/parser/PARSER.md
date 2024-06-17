# Parser
Responsible for creating the AST (abstract syntax tree).

# Grammar

### Declarations
```
program                 -> declaration* EOF;
declaration             -> var | var_assign | statement;
var                     -> "var" IDENTIFIER "\n";
var_assign              -> "var" IDENTIFIER "=" expression "\n";
```

### Statements
```
statement               -> if | print | block | expr;
if                      -> "if" expression ":" "\n" statement ("else" statement)?;
print                   -> "print" expression "\n";
block                   -> INDENT declaration*;
expr                    -> expression "\n";
```

### Expressions
```
expression              -> assignment;
assignment              -> IDENTIFIER ("=" | "+=" | "-=" | "*=" | "/=" | "%=" | "**=" | "&=" | "|=" | "^=" | ">>=" | "<<=") expression | or;
or                      -> and ("or" logic_and)*;
and                     -> equality ("and" equality)*;
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