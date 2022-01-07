use std::{
    fs::File,
    io::{BufReader, Read},
};

use tinyx::{lexer::Lexer, parser::parser::Parser};

fn main() {
    let file = File::open("source.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    println!("\n------- TOKEN START -----------\n");
    let mut lexer = Lexer::new(&contents.as_bytes(), "source.txt");
    lexer.log();
    println!("\n-------TOKEN END -----------\n\n");

    println!("\n-------- AST START ----------\n");
    let mut parser = Parser::new(&contents, "source.txt");
    let ast = parser.parse();
    println!("{:#?}", ast);
    println!("\n-------- AST END -----------\n\n");

    // println!("\n------ INTERPRETER START ------------\n");
    // evaluator::Evaluator::eval();
    // println!("\n------- INTERPRETER END -----------\n\n");
}
