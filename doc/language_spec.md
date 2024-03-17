```
program
    : [ prologue ] statement [ ';' statement ]* [ ';' ] [ epilogue ]

prologue
    : 'prologue' '{' statement [ ';' statement ]* [ ';' ] '}'

epilogue
    : 'epilogue' '{' statement [ ';' statement ]* [ ';' ] '}'

statement
    : function-call
    | assignment
    | function-definition

function-call
    : identifier '(' [ expr [ ',' expr ]* ]? ')'

assignment
    : identifier '=' expr

function-definition
    : fn identifier '(' identifier [ ',' identifier ]* ')'
        '{' statement [ ';' statement ]* '}'

# implementation note: This rule is left-recursing
expr
    : function-call
    | value
    | expr '[' integer ']'

value
    : '_'
    | identifier
    | string
    | integer

identifier
    : alpha-char [ alpha-char | digit | '_' ]*

string
    : '"' char* '"'

integer
    : digit*

digit
    : 0 .. 9

alpha-char
    : a .. z
    | A .. Z

char
    : any UTF-8 char
```
