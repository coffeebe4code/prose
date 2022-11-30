use ast::Expr;
use block::Block;
use gen::GenSource;

pub struct IrSource<'a> {
    reg_id: usize,
    block_id: usize,
    main_exit: usize,
    blocks: Vec<Block<'a>>,
    gen: GenSource,
}

impl<'a> IrSource<'a> {
    pub fn ir_new() -> Self {
        IrSource {
            reg_id: 0,
            block_id: 0,
            main_exit: 0,
            blocks: vec![],
            gen: GenSource::new(),
        }
    }
    pub fn ir_recurse(&mut self, recurse: &Expr) -> usize {
        let mut result = 0;
        match recurse {
            Expr::Body(exprs) => {
                for e in exprs.into_iter() {
                    result = self.ir_recurse(e);
                }
            }
            _ => {
                panic!("not implemented");
            }
        }
        return result;
    }
    pub fn ir_begin(&mut self, top: &Expr) -> &mut Self {
        self.main_exit = self.ir_recurse(top);
        return self;
    }
}
