use super::debug;
use super::Chunk;
use super::OpCode;
use super::OpCode::*;
use super::Value;

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub fn interpret(chunk: Chunk) -> Result<(), InterpretError> {
    let mut vm = Vm::new(chunk);
    vm.run()
}

#[derive(Debug)]
pub struct Vm {
    chunk: Chunk,
    ip: usize, // instruction pointer
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

    fn run(&mut self) -> Result<(), InterpretError> {
        while self.ip < self.chunk.codes.len() {
            self.trace();

            match self.chunk.codes.get(self.ip).unwrap() {
                OpReturn => {
                    let value = self.pop();
                    println!("\n\n-----------------\nRETURN VALUE: \n");
                    println!("> {:?}\n\n", value);
                }
                OpConstant(idx) => {
                    let value = self.chunk.read_constant(*idx);
                    self.push(value);
                }
                OpNegate => {
                    let val = -self.pop();
                    self.push(val);
                }
                OpAdd => self.binary_op(OpAdd),
                OpSubtract => self.binary_op(OpSubtract),
                OpMultiply => self.binary_op(OpMultiply),
                OpDivide => self.binary_op(OpDivide),
            };
            self.ip += 1;
        }
        Ok(())
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value::Null)
    }

    fn binary_op(&mut self, op: OpCode) {
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

    fn trace(&self) {
        print!("          ");
        for i in &self.stack {
            print!("[ ");
            print!("{:?}", i);
            print!(" ]");
        }
        println!();

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

        let idx = chunk.add_constant(3.2.into());
        chunk.write(OpConstant(idx), (1, 1));
        chunk.write(OpReturn, (1, 2));

        interpret(chunk).unwrap();
    }

    #[test]
    fn test_add() {
        let mut chunk = Chunk::default();

        let idx = chunk.add_constant(1.into());
        chunk.write(OpConstant(idx), (1, 1));
        let idx = chunk.add_constant(2.into());
        chunk.write(OpConstant(idx), (1, 2));
        chunk.write(OpAdd, (1, 3));
        chunk.write(OpReturn, (1, 4));

        interpret(chunk).unwrap();
    }
}
