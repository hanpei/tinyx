use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::{lexer::Lexer, parser::parser::Parser};

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
