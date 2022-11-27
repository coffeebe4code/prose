use opcode::Op;

pub enum Instr {
    Operation(Op, u8, u8, u8),
    Data(Op, usize, usize),
}

impl Instr {
    pub fn make_data(op: Op, data: usize, size: usize) -> Self {
        Instr::Data(op, data, size)
    }
    pub fn make_op(op: Op, dst: u8, srcl: u8, srcr: u8) -> Self {
        Instr::Operation(op, dst, srcl, srcr)
    }
}
