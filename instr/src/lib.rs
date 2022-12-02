use opcode::Op;

pub enum Instr {
    ModOp(Op, u8, u8, u8),
    Operation(Op, u8, u8, u8),
    Data(Op, u8, usize, u8),
}

impl Instr {
    pub fn make_data(op: Op, dst: u8, data: usize, size: u8) -> Self {
        Instr::Data(op, dst, data, size)
    }
    pub fn make_op(op: Op, dst: u8, srcl: u8, srcr: u8) -> Self {
        Instr::Operation(op, dst, srcl, srcr)
    }
    pub fn make_mod(op: Op, dst: u8, data: u8, modop: u8) -> Self {
        Instr::ModOp(op, dst, data, modop)
    }
}

pub fn instr_raw(op: Op, dst: u8, srcl: u8, srcr: u8) -> [u8; 4] {
    let mut val: [u8; 4] = [0; 4];
    val[0] = op.into();
    val[1] = dst;
    val[2] = srcl;
    val[3] = srcr;
    val
}
