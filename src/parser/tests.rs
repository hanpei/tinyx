use crate::{
    ast::{BinaryExpr, Expr, Identifier, NumericLiteral, Program, Statement, VariableDeclaration},
    lexer::Lexer,
    parser::parser::Parser,
    position::{Loc, Pos, Span},
    token::Operator,
    ParseResult,
};

#[test]
fn test_block_statment() -> ParseResult<()> {
    let slist = vec![
        r#"{1}"#,
        r#"{1;}"#,
        r#"{
            1
            2
        }"#,
        r#"{1;2}"#,
        r#"{1;2;}"#,
        r#"{
            1;
            2;
        }"#,
        r#"{1;
        2;}"#,
        r#"{
            1
            {
                2
                3
            }
        }"#,
    ];
    for s in slist.iter() {
        let mut program = Parser::new(Lexer::new(s.as_bytes(), "source.txt"));
        program.parse()?;
    }
    Ok(())
}

#[test]
fn test_statment_list() {
    let s = r#"
    1
        1
          2 
    1;1;1
    2   ;
    3;4
"#;

    let mut program = Parser::new(Lexer::new(s.as_bytes(), "source.txt"));
    program.parse().unwrap();
}

#[test]
fn test_binary_expr() {
    let s = r#"
    1+2*3
"#;

    let mut program = Parser::new(Lexer::new(s.as_bytes(), "source.txt"));
    let ast = program.parse().unwrap();
    // println!("{:#?}", ast);
    let right = BinaryExpr::new(
        Expr::NumericLiteral(NumericLiteral::new(
            2.0,
            Span::new(
                "source.txt".to_string(),
                Loc::new(Pos { ln: 2, col: 7 }, Pos { ln: 2, col: 8 }),
            ),
        )),
        Operator::Mul,
        Expr::NumericLiteral(NumericLiteral::new(
            3.0,
            Span::new(
                "source.txt".to_string(),
                Loc::new(Pos { ln: 2, col: 9 }, Pos { ln: 3, col: 0 }),
            ),
        )),
    );
    let binay = BinaryExpr::new(
        Expr::NumericLiteral(NumericLiteral::new(
            1.0,
            Span::new(
                "source.txt".to_string(),
                Loc::new(Pos { ln: 2, col: 5 }, Pos { ln: 2, col: 6 }),
            ),
        )),
        Operator::Add,
        Expr::Binary(right),
    );
    let ret = Program::new(vec![Statement::ExprStmt(Expr::Binary(binay))]);
    assert_eq!(ast, ret);
}

#[test]
fn test_binary_expr_parenparentheses() {
    let s = r#"
    (1+2)*3
"#;

    let mut program = Parser::new(Lexer::new(s.as_bytes(), "source.txt"));
    let ast = program.parse().unwrap();
    println!("{:#?}", ast);
    let b1 = BinaryExpr::new(
        Expr::NumericLiteral(NumericLiteral::new(
            1.0,
            Span::new(
                "source.txt".to_string(),
                Loc::new(Pos { ln: 2, col: 6 }, Pos { ln: 2, col: 7 }),
            ),
        )),
        Operator::Add,
        Expr::NumericLiteral(NumericLiteral::new(
            2.0,
            Span::new(
                "source.txt".to_string(),
                Loc::new(Pos { ln: 2, col: 8 }, Pos { ln: 2, col: 9 }),
            ),
        )),
    );
    let b2 = BinaryExpr::new(
        Expr::Binary(b1),
        Operator::Mul,
        Expr::NumericLiteral(NumericLiteral::new(
            3.0,
            Span::new(
                "source.txt".to_string(),
                Loc::new(Pos { ln: 2, col: 11 }, Pos { ln: 3, col: 0 }),
            ),
        )),
    );

    let ret = Program::new(vec![Statement::ExprStmt(Expr::Binary(b2))]);
    assert_eq!(ast, ret);
}

#[test]
fn test_let_stmt() {
    let s = r#"
    let a = 1
    let b = 1+2
"#;

    let mut program = Parser::new(Lexer::new(s.as_bytes(), "source.txt"));
    let ast = program.parse().unwrap();
    println!("{:#?}", ast);

    let ret = Program::new(vec![
        Statement::VariableDeclaration(VariableDeclaration::new(
            Identifier::new(
                "a".to_string(),
                Span::new(
                    "source.txt".to_string(),
                    Loc::new(Pos { ln: 2, col: 8 }, Pos { ln: 2, col: 9 }),
                ),
            ),
            Some(Expr::NumericLiteral(NumericLiteral::new(
                1.0,
                Span::new(
                    "source.txt".to_string(),
                    Loc::new(Pos { ln: 2, col: 13 }, Pos { ln: 3, col: 0 }),
                ),
            ))),
        )),
        Statement::VariableDeclaration(VariableDeclaration::new(
            Identifier::new(
                "b".to_string(),
                Span::new(
                    "source.txt".to_string(),
                    Loc::new(Pos { ln: 3, col: 8 }, Pos { ln: 3, col: 9 }),
                ),
            ),
            Some(Expr::Binary(BinaryExpr::new(
                Expr::NumericLiteral(NumericLiteral::new(
                    1.0,
                    Span::new(
                        "source.txt".to_string(),
                        Loc::new(Pos { ln: 3, col: 13 }, Pos { ln: 3, col: 14 }),
                    ),
                )),
                Operator::Add,
                Expr::NumericLiteral(NumericLiteral::new(
                    2.0,
                    Span::new(
                        "source.txt".to_string(),
                        Loc::new(Pos { ln: 3, col: 15 }, Pos { ln: 4, col: 0 }),
                    ),
                )),
            ))),
        )),
    ]);
    assert_eq!(ast, ret);
}
