use opcode::Op;

pub enum Instr {
    Operation(Op, usize, usize, usize),
}

impl Instr {
    pub fn new_op(op: Op, dst: usize, srcl: usize, srcr: usize) -> Self {
        Instr::Operation(op, dst, srcl, srcr)
    }
}
