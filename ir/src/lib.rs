use ast::*;
use block::*;
use gen::*;
use instr::*;
use opcode::*;
use token::*;

pub struct IrSource<'block, 'source> {
    reg_id: usize,
    block_id: usize,
    main_exit: usize,
    blocks: Vec<Block<'block, 'source>>,
}

impl<'block, 'source> IrSource<'block, 'source> {
    pub fn new() -> Self {
        IrSource {
            reg_id: 0,
            block_id: 0,
            main_exit: 0,
            blocks: vec![],
        }
    }
    pub fn recurse(&mut self, recurse: &Expr) -> usize {
        match recurse {
            Expr::BinOp(leftexpr, lexeme, rightexpr) => {
                let left = self.recurse(leftexpr);
                let right = self.recurse(rightexpr);
                match lexeme.token {
                    Token::Sub => {
                        let instr = Instr::new_op(Op::F64Sub, self.reg_id, left, right);
                        self.blocks[self.block_id].insert_instr(instr);
                        return self.reg_inc();
                    }
                    Token::Plus => {
                        let instr = Instr::new_op(Op::F64Add, self.reg_id, left, right);
                        self.blocks[self.block_id].insert_instr(instr);
                        return self.reg_inc();
                    }
                    Token::Mul => {
                        let instr = Instr::new_op(Op::F64Mul, self.reg_id, left, right);
                        self.blocks[self.block_id].insert_instr(instr);
                        return self.reg_inc();
                    }
                    _ => {
                        panic!("not implemented, token in recurse");
                    }
                }
            }
            Expr::Number(lexeme) => {
                let instr =
                    Instr::new_op(Op::LoopOp, self.reg_id, lexeme.slice.parse().unwrap(), 0);
                self.blocks[self.block_id].insert_instr(instr);
                return self.reg_inc();
            }
            _ => {
                panic!("not implemented, expr in recurse");
            }
        }
    }
    pub fn begin(&mut self, top: &Expr) -> &mut Self {
        self.main_exit = self.recurse(top);
        return self;
    }
    pub fn flush(self, gen: &mut GenSource) -> () {
        self.blocks.iter().for_each(|b| {
            b.instructions.iter().for_each(|i| match i {
                Instr::Operation(o, d, l, r) => {
                    gen.add32([(*o).into(), 0, 0, 0]);
                    gen.add64(usize::to_ne_bytes(*d));
                    gen.add64(usize::to_ne_bytes(*l));
                    gen.add64(usize::to_ne_bytes(*r));
                }
            })
        })
    }
    pub fn reg_inc(&mut self) -> usize {
        let val = self.reg_id;
        self.reg_id += 1;
        return val;
    }
}
