use opcode::Op;

pub struct Instr {
    opcode: Op,
    dst: usize,
}
