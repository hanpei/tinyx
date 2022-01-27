use std::{
    fs::File,
    io::{BufReader, Read},
};

use tinyx::{
    analizer::resolver::Resolver, interpreter::Interpreter, lexer::Lexer, parser::parser::Parser,
};

fn main() {
    let file = File::open("source.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    // println!("\n------- TOKEN START -----------\n");
    // let mut lexer_for_log = Lexer::new(&contents.as_bytes(), "source.txt");
    // lexer_for_log.log();
    // println!("\n-------TOKEN END -----------\n\n");

    println!("\n-------- AST START ----------\n");
    let lexer = Lexer::new(&contents.as_bytes(), "source.txt");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    println!("{:#}", ast);
    println!("\n-------- AST END -----------\n\n");

    println!("\n------ INTERPRETER START ------------\n");
    let mut interpreter = Interpreter::new();
    let mut r = Resolver::new(&mut interpreter);
    r.resolve(&ast)
        .unwrap_or_else(|e| eprintln!("ERROR: {}", e));
    interpreter
        .interpret(ast)
        .unwrap_or_else(|e| eprintln!("ERROR: {}", e));
    println!("\n------- INTERPRETER END -----------\n\n");
}
