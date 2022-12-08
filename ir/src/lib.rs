use ast::*;
use block::Block;
use gen::GenSource;
use instr::*;
use opcode::*;

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
        let mut result = 0;
        match recurse {
            Expr::Body(exprs) => {
                for e in exprs.into_iter() {
                    result = self.recurse(e);
                }
            }
            Expr::BinOp(leftexpr, token, rightexpr) => {
                let left = self.recurse(leftexpr);
                let right = self.recurse(rightexpr);
            }
            _ => {
                panic!("not implemented");
            }
        }
        return result;
    }
    pub fn begin(&mut self, top: &Expr) -> &mut Self {
        self.main_exit = self.recurse(top);
        return self;
    }
    pub fn reg_inc(&mut self) -> usize {
        let val = self.reg_id;
        self.reg_id += 1;
        return val;
    }
}
