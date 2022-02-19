use super::{opcode::ConstantIndex, OpCode, Value};

pub type Pos = (usize, usize); // (ln, col)

#[derive(Debug)]
pub struct Chunk {
    pub codes: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub positions: Vec<Pos>,
}

impl Chunk {
    pub fn write_chunk(&mut self, code: OpCode, pos: Pos) {
        self.codes.push(code);
        self.positions.push(pos);
    }

    pub fn add_constant(&mut self, value: Value) -> ConstantIndex {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn read_constant(&self, idx: ConstantIndex) -> Value {
        self.constants[idx].clone()
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            codes: Default::default(),
            constants: Default::default(),
            positions: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use OpCode::*;
        let mut chunk = Chunk::default();
        chunk.write_chunk(OpReturn, (1, 1));

        let idx = chunk.add_constant(1.2.into());
        chunk.write_chunk(OpConstant(idx), (1, 2));

        println!("{:?}", chunk);
    }
}
