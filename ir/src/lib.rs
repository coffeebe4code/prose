use ast::*;
use block::Block;
use gen::GenSource;
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
            Expr::BinOp(leftexpr, token, rightexpr) => {
                let left = self.recurse(leftexpr);
                let right = self.recurse(rightexpr);
                match *token {
                    Token::Sub => {
                        // how to get the type f64, u64 etc.
                        let instr = Instr::new_op(Op::F64Sub, self.reg_id, left, right);
                        self.blocks[self.block_id].insert_instr(instr);
                        return self.reg_inc();
                    }
                    _ => {
                        panic!("not implemented");
                    }
                }
            }
            _ => {
                panic!("not implemented");
            }
        }
    }
    pub fn begin(&mut self, top: &Expr) -> &mut Self {
        self.main_exit = self.recurse(top);
        return self;
    }
    pub fn flush(self, gen: &mut GenSource) -> () {
        for b in self.blocks.iter() {
            for i in b.instructions.iter() {
                match i {
                    Instr::Operation(o, d, l, r) => {
                        panic!("not implemented");
                    }
                }
            }
        }
    }
    pub fn reg_inc(&mut self) -> usize {
        let val = self.reg_id;
        self.reg_id += 1;
        return val;
    }
}
