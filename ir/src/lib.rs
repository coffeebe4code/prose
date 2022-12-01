use ast::Expr;
use block::Block;
use gen::GenSource;
use instr::*;
use opcode::Op;

pub struct IrSource<'a> {
    reg_id: usize,
    block_id: usize,
    main_exit: usize,
    blocks: Vec<Block<'a>>,
    gen: GenSource,
}

impl<'a> IrSource<'a> {
    pub fn new() -> Self {
        IrSource {
            reg_id: 0,
            block_id: 0,
            main_exit: 0,
            blocks: vec![],
            gen: GenSource::new(),
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
    pub fn data_gen(&mut self, opcode: Op, data: usize) -> &mut Self {
        if self.gen.is_64_aligned() {
            let new_id = self.reg_inc();
            self.gen.add32(instr_raw(opcode, new_id as u8, 0, 1));
        }
        self.gen.add64(data.to_ne_bytes());
        return self;
    }
    pub fn reg_inc(&mut self) -> usize {
        let val = self.reg_id;
        self.reg_id += 1;
        return val;
    }
}
