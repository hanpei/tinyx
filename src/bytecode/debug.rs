use super::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("\n== {} ==", name);

    for i in 0..chunk.codes.len() {
        disassemble_instruction(chunk, i)
    }
    println!();
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    print!("{:04} ", offset);

    let pos = &chunk.positions[offset];
    print!(" <{}:{}> ", pos.0, pos.1);

    use OpCode::*;
    let code = chunk.codes.get(offset).expect("chunk codes is empty");
    match code {
        OpReturn => simple_instruction(OpReturn),
        OpConstant(idx) => constant_instruction(OpConstant(*idx), chunk),
        OpNegate => simple_instruction(OpNegate),
        OpAdd => simple_instruction(OpAdd),
        OpSubtract => simple_instruction(OpSubtract),
        OpMultiply => simple_instruction(OpMultiply),
        OpDivide => simple_instruction(OpDivide),
        // _ => println!("Unknown opcode {}", *code),
    }
}

fn simple_instruction(code: OpCode) {
    println!("{}", code);
}

fn constant_instruction(code: OpCode, chunk: &Chunk) {
    let idx = code.get_const_index().unwrap();
    let constant = chunk.constants.get(idx).unwrap();
    println!("{:<16} {:4} '{}'", code, idx, constant);
}

#[cfg(test)]
mod tests {
    use crate::bytecode::vm::interpret;

    use super::*;

    #[test]
    fn it_works() {
        use OpCode::*;
        let mut chunk = Chunk::default();
        let idx = chunk.add_constant(1.2.into());
        chunk.write(OpConstant(idx), (1, 2));
        chunk.write(OpReturn, (1, 1));

        disassemble_chunk(&chunk, "test chunk");
        interpret(chunk).unwrap();
    }

    #[test]
    fn test_negate() {
        use OpCode::*;
        let mut chunk = Chunk::new();
        let idx = chunk.add_constant(123.into());
        chunk.write(OpConstant(idx), (1, 2));
        chunk.write(OpNegate, (1, 3));
        chunk.write(OpReturn, (1, 4));

        interpret(chunk).unwrap();
    }
}
