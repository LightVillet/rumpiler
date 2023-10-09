# Rumpiler: blazingly fast compiling subset of C

Currently we are supporting following subset of C:
```
<program> := <function>
<function> := int <function_name> ( ) { <statement> }
<statement> := return <expression> ;
<expression> := <unary_op> <expression> | <int>
<unary_op> := ! | - | ~ 
```