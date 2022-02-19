pub type ConstantIndex = usize;

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant(ConstantIndex),
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

impl OpCode {
    pub fn get_const_index(&self) -> Option<ConstantIndex> {
        if let Self::OpConstant(idx) = self {
            Some(*idx)
        } else {
            None
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpCode::*;
        match self {
            OpReturn => write!(f, "OP_RETURN"),
            OpConstant(_) => write!(f, "OP_CONSTANT"),
            OpNegate => write!(f, "OP_NEGATE"),
            OpAdd => write!(f, "OP_ADD"),
            OpSubtract => write!(f, "OP_SUBTRACT"),
            OpMultiply => write!(f, "OP_MULTIPLY"),
            OpDivide => write!(f, "OP_DIVIDE"),
        }
    }
}
