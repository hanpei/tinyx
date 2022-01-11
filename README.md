tinyx

source:
1 + 2 \* 3 / 4

source -> token -> ast -> eval

## grammar

```
program:
    : statementList EOF
    ;

statementList:
    : statement*
    ;

// statement

statement:
    : ExpressionStatement
    | BlockStatement
    | EmptyStatement
    | IfStatement
    | ReturnStatement
    | VariableDeclarator
    | FunctionDeclaration
    ;


ExpressionStatement
    : Expression STMT_TERMINATOR
    ;

STMT_TERMINATOR:
    : ";"
    | "EOL"
    | ";" "EOL"
    | "}"
    ;

BlockStatement:
    : "{" statementList "}"
    ;


// expression

Expression:
    : AssignmentExpression
    ;

AssignmentExpression:
    : EqualityExpression
    | IDENTIFIER "=" AssignmentExpression
    ;

EqualityExpression
    : RelationalExpression ( ( "!=" | "==" ) RelationalExpression )*
    ;

RelationalExpression
    : AdditiveExpression ( ( "<" | "<=" | ">" | ">=" ) AdditiveExpression )*
    ;

AdditiveExpression
    :MultiplicativeExpression ( (ADD|MIN) MultiplicativeExpression )*
    ;

MultiplicativeExpression
    : UnaryExpression ( (MUL|DIV) UnaryExpression )*
    ;

UnaryExpression
    : PrimaryExpression
    ;

```
