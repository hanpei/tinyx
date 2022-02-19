use super::debug;
use super::Chunk;
use super::OpCode;
use super::OpCode::*;
use super::Value;

#[derive(Debug)]
enum InterpretError {
    CompileError,
    RuntimeError,
}

#[derive(Debug)]
struct Vm {
    chunk: Chunk,
    // instruction pointer
    ip: usize,
    stack: Vec<Value>,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            chunk: Default::default(),
            ip: Default::default(),
            stack: Default::default(),
        }
    }
}

impl Vm {
    fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: Default::default(),
            stack: Default::default(),
        }
    }

    fn interpret(&mut self) -> Result<(), InterpretError> {
        self.run()
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        while self.ip < self.chunk.codes.len() {
            // self.log();
            match self.chunk.codes.get(self.ip).unwrap() {
                OpReturn => println!("{}", self.pop()),
                OpConstant(idx) => {
                    let value = self.chunk.read_constant(*idx);
                    self.push(value);
                }
                OpNegate => {
                    let val = -self.pop();
                    self.push(val);
                }
                OpAdd => self.binaryOp(OpAdd),
                OpSubtract => self.binaryOp(OpSubtract),
                OpMultiply => self.binaryOp(OpMultiply),
                OpDivide => self.binaryOp(OpDivide),
            };
            self.ip += 1;
        }
        Ok(())
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("stack empty")
    }

    fn binaryOp(&mut self, op: OpCode) {
        let y = self.pop();
        let x = self.pop();
        let result = match op {
            OpAdd => x + y,
            OpSubtract => x - y,
            OpMultiply => x * y,
            OpDivide => x / y,
            _ => unreachable!(),
        };
        self.push(result);
    }

    fn log(&self) {
        print!("[ ");
        self.stack.iter().for_each(|value| print!("{:<16}", value));
        print!(" ]");
        debug::disassemble_instruction(&self.chunk, self.ip);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init() {
        let vm = Vm::default();
        println!("vm = {:?}", vm);
    }

    #[test]
    fn it_works() {
        let mut chunk = Chunk::default();

        let idx = chunk.add_constant(1.2.into());
        chunk.write_chunk(OpConstant(idx), (1, 2));
        chunk.write_chunk(OpNegate, (1, 2));
        chunk.write_chunk(OpReturn, (1, 2));

        let mut vm = Vm::new(chunk);
        vm.interpret().unwrap();
    }

    #[test]
    fn arithmetic() {
        let mut chunk = Chunk::default();

        let idx = chunk.add_constant(1.2.into());
        chunk.write_chunk(OpConstant(idx), (1, 1));

        let idx = chunk.add_constant(3.4.into());
        chunk.write_chunk(OpConstant(idx), (1, 3));

        chunk.write_chunk(OpAdd, (1, 2));

        let idx = chunk.add_constant(5.into());
        chunk.write_chunk(OpConstant(idx), (1, 5));

        chunk.write_chunk(OpSubtract, (1, 4));

        let mut vm = Vm::new(chunk);

        vm.interpret().unwrap();
        println!("stack: {:?}", vm.stack);
        let result = vm.pop();
        assert_eq!(result, (1.2 + 3.4 - 5.0).into());
    }
}
