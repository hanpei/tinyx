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

FunctionDeclaration
    : "fn" Identifier ( "(" ( FormalParameterList )? ")" ) BlockStatement
    ;

FormalParameterList
    : Identifier ( "," Identifier )*
    ;



// expression

Expression:
    : AssignmentExpression
    ;

AssignmentExpression:
    : Identifier "=" AssignmentExpression
    | LogicORExpression
    ;

LogicORExpression:
    : LogicANDExpress ( "or" LogicANDExpress )*
    ;

LogicANDExpress:
    : EqualityExpression ( "and" EqualityExpression )*
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
    | CallExpression
    ;

CallExpression
    : PrimaryExpression ( "(" arguments? ")" )*
    ;

Arguments:
    : expression ( "," expression )*
    ;

PrimaryExpression
    : Literal
    | Identifier
    | Expression
    ;


```

## lexical scope

```
let a = "global";
{
  fn showA() {
    print a;
  }

  showA();
  let a = "block";
  showA();
}

// global
// block
```
