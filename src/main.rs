use std::{
    fs::File,
    io::{BufReader, Read},
};

use tinyx::{
    analizer::resolver::Resolver, interpreter::Interpreter, lexer::Lexer, parser::parser::Parser,
};

fn main() {
    let file = File::open("source.txt").unwrap();
    println!("{:?}", file);
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    // println!("\n------- TOKEN START -----------\n");
    // let mut lexer_for_log = Lexer::new(&contents.as_bytes(), "source.txt");
    // lexer_for_log.log();
    // println!("\n-------TOKEN END -----------\n\n");

    println!("\n-------- AST START ----------\n");
    let lexer = Lexer::new(contents.as_bytes(), "source.txt");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    // println!("{:#}", ast);
    println!("\n-------- AST END -----------\n\n");

    println!("\n------ INTERPRETER START ------------\n");
    let mut interpreter = Interpreter::default();
    let mut r = Resolver::new(&mut interpreter);
    match r.resolve(&ast) {
        Ok(_) => interpreter
            .interpret(ast)
            .unwrap_or_else(|e| eprintln!("ERROR: {}", e)),
        Err(e) => eprintln!("ERROR: {}", e),
    }

    println!("\n------- INTERPRETER END -----------\n\n");
}

#[cfg(test)]
mod tests {
    use tinyx::bytecode::{debug::disassemble_chunk, Chunk, OpCode};

    #[test]
    fn test_write_chunk() {
        let mut chunk = Chunk::new();
        chunk.write(OpCode::OpReturn, (1, 1));
        disassemble_chunk(&chunk, "test chunk");
    }
}
