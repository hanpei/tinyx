use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    lexer::Lexer,
    parser::parser::Parser,
    position::{Loc, Pos, Span},
};

use super::Identifier;

#[test]
fn test_parse_ast() {
    let file = File::open("ast_test.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    println!("\n-------- AST START ----------\n");
    let lexer = Lexer::new(&contents.as_bytes(), "ast_test.txt");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    println!("{}", ast);
    println!("\n-------- AST END -----------\n\n");
}

#[test]
fn test_identifier_tostring() {
    let ident = Identifier::new(
        String::from("abc"),
        Span {
            filename: "test.txt".to_string(),
            loc: Loc::new(Pos { ln: 1, col: 2 }, Pos { ln: 1, col: 5 }),
        },
    );
    assert_eq!(ident.to_string(), String::from("abc@1:2"))
}
