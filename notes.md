# Lexer
Lexer turns code into tokens. For instance,
```cs
1 + 1

=>

[INT: 1, PLUS, INT: 1]
```

# Parser
The parser then converts these tokens into an AST.
1.) Figure out if the tokens match our language syntax
2.) Convert into AST
## AST => Abstract Syntax Tree
Examples...
```cs
1 + 2 * 3

=>

                BinOp(PLUS)
                /         \
            Number(1)  BinOp(MUL)
                        /       \   
                    Number(2) Number(3)
```
```cs
(1 + 2) * 3

=>

                BinOp(MUL)
                /         \
            BinOp(PLUS)  Number(3)
             /       \   
         Number(1) Number(2)
```


