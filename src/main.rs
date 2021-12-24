use std::{fs::File, io::{BufReader, Read}};

use tinyx::lexer::Lexer;

fn main() {
    println!("start");
    let file = File::open("source.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let mut lexer = Lexer::new(&contents.as_bytes());
    // println!("{:#}", lexer);
    while let Some(token) = lexer.tokenize() {
        println!("{:?}", token);
    }
    println!("end");
}
