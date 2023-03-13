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
                        return std_instr!(self, Op::F64Sub, self.reg_id, left, right);
                    }
                    Token::Plus => {
                        return std_instr!(self, Op::F64Add, self.reg_id, left, right);
                    }
                    Token::Mul => {
                        return std_instr!(self, Op::F64Mul, self.reg_id, left, right);
                    }
                    _ => {
                        panic!("not implemented, token in recurse");
                    }
                }
            }
            Expr::Number(lexeme) => {
                return std_instr!(
                    self,
                    Op::F64Const,
                    self.reg_id,
                    lexeme.slice.parse().unwrap(),
                    0
                );
            }
            _ => {
                panic!("not implemented, expr in recurse");
            }
        }
    }
    pub fn begin_repl(&mut self, top: &Expr) {
        self.blocks.push(Block::new(self.block_id));
        self.blocks[self.block_id].kind = BlockKind::MainBlock;
        self.main_exit = self.recurse(top);
    }
    pub fn begin(&mut self, top: &Expr) {
        self.main_exit = self.recurse(top);
    }
    pub fn flush(self, gen: &mut GenSource) -> () {
        self.blocks.iter().for_each(|b| {
            b.instructions.iter().for_each(|i| match i {
                Instr::Operation(o, d, l, r) => {
                    gen.add64([(*o).into(), 0, 0, 0, 0, 0, 0, 0]);
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

#[macro_export]
macro_rules! std_instr {
    ($self:ident, $val:expr, $dst:expr, $srcl:expr, $srcr:expr) => {{
        let instr = Instr::new_op($val, $dst, $srcl, $srcr);
        $self.blocks[$self.block_id].insert_instr(instr);
        $self.reg_inc()
    }};
}
