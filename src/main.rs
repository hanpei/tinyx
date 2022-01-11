use std::{
    fs::File,
    io::{BufReader, Read},
};

use tinyx::{evaluator::Evaluator, lexer::Lexer, parser::parser::Parser};

fn main() {
    let file = File::open("source.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    println!("\n------- TOKEN START -----------\n");
    let mut lexer_for_log = Lexer::new(&contents.as_bytes(), "source.txt");
    lexer_for_log.log();
    println!("\n-------TOKEN END -----------\n\n");

    println!("\n-------- AST START ----------\n");
    let lexer = Lexer::new(&contents.as_bytes(), "source.txt");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:#?}", ast);
    println!("\n-------- AST END -----------\n\n");

    println!("\n------ INTERPRETER START ------------\n");
    let mut evaluator = Evaluator::new();
    evaluator.eval(&ast.unwrap());
    println!("\n------- INTERPRETER END -----------\n\n");
}
