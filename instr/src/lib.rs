use opcode::Op;

pub enum CompInstr {
    COperation(Op, u8, u8, u8),
}

pub enum Instr {
    Operation(Op, usize, usize, usize),
}

impl CompInstr {
    pub fn new(op: Op, dst: u8, srcl: u8, srcr: u8) -> Self {
        CompInstr::COperation(op, dst, srcl, srcr)
    }
    pub fn to_raw(&self) -> [u8; 4] {
        let mut val: [u8; 4] = [0; 4];
        match self {
            CompInstr::COperation(o, d, l, r) => {
                val[0] = (*o).into();
                val[1] = *d;
                val[2] = *l;
                val[3] = *r;
                val
            }
        }
    }
}

impl Instr {
    pub fn new_op(op: Op, dst: usize, srcl: usize, srcr: usize) -> Self {
        Instr::Operation(op, dst, srcl, srcr)
    }
}
