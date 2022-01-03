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
    
    let mut lexer = Lexer::new(&contents.as_bytes(), "source.txt");
    let mut parser = Parser::new(&contents, "source.txt");

    println!("\nToken START");
    lexer.log();
    println!("Token END\n");

    println!("AST START");
    let ast = parser.parse();
    println!("{:#?}", ast);
    println!("AST END\n");
}
