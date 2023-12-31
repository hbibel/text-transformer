The TT program prepares your program to be interpreted by the TT language. The
program defined in the TT language is executed on each item of the input, where
the definition of an item is responsibility of the program. Per default, the
text input is split into lines, and each line is an item. Then, the program is
run for each item.

A simple TT program simply prints each item it receives as input:

```
println(_);
```

Here, `println` is a built-in function whose parameter list is enclosed by
parenthese `(` and `)`. The function takes one parameter, in this example the
parameter is the item, as denoted by `_`. The instruction is terminated by a
semicolon `;`.

Since we programmers tend to forget semicolons, the last semicolon in a program
is optional.

The input is per default first split by newlines, then whitespace. Per default
the item the program runs on is the first dimension, so the lines per default.
Dimensions can be accessed by index notation e.g. `_[2][3]`. More dimensions
can be defined by adding more dimension separators, e.g. `-`.

TODO remove value expressions as top level statements

```
program := expr [ ';' expr ]* [ ';' ]

expr :=
    | function-call
    | value

function-call := function-name '(' [ expr [ ',' expr ]* ]? ')'

value :=
    | '_'
    | identifier
    | string
    | number

identifier := char [ char | digit | '_' ]*

string := '"' char* '"'

number := digit*

digit := 0 .. 9

char := any UTF-8 char
```
