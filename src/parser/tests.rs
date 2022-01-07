use crate::{
    ast::{BinaryExpr, Expr, Identifier, Program, Statement, VariableDeclaration},
    parser::parser::Parser,
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
        let mut program = Parser::new(s, "");
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

    let mut program = Parser::new(s, "");
    program.parse().unwrap();
}

#[test]
fn test_binary_expr() {
    let s = r#"
    1+2*3
"#;

    let mut program = Parser::new(s, "");
    let ast = program.parse().unwrap();
    // println!("{:#?}", ast);
    let right = BinaryExpr::new(
        Expr::NumericLiteral(2.0),
        Operator::Mul,
        Expr::NumericLiteral(3.0),
    );
    let binay = BinaryExpr::new(
        Expr::NumericLiteral(1.0),
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

    let mut program = Parser::new(s, "");
    let ast = program.parse().unwrap();
    println!("{:#?}", ast);
    let b1 = BinaryExpr::new(
        Expr::NumericLiteral(1.0),
        Operator::Add,
        Expr::NumericLiteral(2.0),
    );
    let b2 = BinaryExpr::new(Expr::Binary(b1), Operator::Mul, Expr::NumericLiteral(3.0));

    let ret = Program::new(vec![Statement::ExprStmt(Expr::Binary(b2))]);
    assert_eq!(ast, ret);
}

#[test]
fn test_let_stmt() {
    let s = r#"
    let a = 1
    let b = 1+2
"#;

    let mut program = Parser::new(s, "");
    let ast = program.parse().unwrap();
    println!("{:#?}", ast);

    let ret = Program::new(vec![
        Statement::VariableDeclaration(VariableDeclaration::new(
            Identifier::new("a".to_string()),
            Some(Expr::NumericLiteral(1.0)),
        )),
        Statement::VariableDeclaration(VariableDeclaration::new(
            Identifier::new("b".to_string()),
            Some(Expr::Binary(BinaryExpr::new(
                Expr::NumericLiteral(1.0),
                Operator::Add,
                Expr::NumericLiteral(2.0),
            ))),
        )),
    ]);
    assert_eq!(ast, ret);
}
