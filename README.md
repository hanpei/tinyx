# tinyx

a tiny lang interpreter;

## Reference:

- Crafting Interpreters: https://craftinginterpreters.com/
- Let’s Build A Simple Interpreter: https://ruslanspivak.com/lsbasi-part1/

## grammar

- ast: https://astexplorer.net
- bnf: https://tomcopeland.blogs.com/EcmaScript.html#prod14

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
    | ClassDeclaration
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
    : "fn" FunctionBody
    ;

FormalParameterList
    : Identifier ( "," Identifier )*
    ;

ClassDeclaration
    : "class" IDENTIFIER ( "extends" IDENTIFIER )? "{" FunctionBody* "}"
    ;

FunctionBody
    : Identifier ( "(" ( FormalParameterList )? ")" ) BlockStatement
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

## example

```
// if stmt
let a =11
let b = 3
if (a-b<0) {
    // print "a < b"
    a
} else if(a - b >0) {
    //print "a > b"
    b
} else {
    print a
}


// while stmt
let a = 1
while (a<10) {
    print a
    a = a + 1
}


// while loop
fn count(n) {
  while (n < 10) {
    if (n == 3) return n; // <--
    print n;
    n = n + 1;
  }
}

count(1);


// closure
fn makeCounter() {
  let i = 0;
  fn count() {
    i = i + 1;
    print i;
  }
  return count;
}

let counter = makeCounter();
counter(); // "1".
counter(); // "2".


// recursion
fun fib(n) {
  if (n <= 1) return n;
  return fib(n - 2) + fib(n - 1);
}
fib(5);
```
