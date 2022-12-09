use instr::Instr;
use ssa::*;

pub enum BlockKind {
    // nested block no returns, possible other function calls
    PlainBlock,
    // pattern matching block
    MatchBlock,
    // if block
    IfBlock,
    // these blocks have other function calls in them
    RetBlock,
    RetBlockVoid,
    // the entry block
    MainBlock,
    // top level vars and functions.
    GlobalBlock,
    // no other method calls in the block.
    LeafBlockVoid,
    LeafBlock,
}

pub struct Block<'block, 'source> {
    pub kind: BlockKind,
    pub id: usize,
    pub instructions: Vec<Instr>,
    vars: Vec<Var<'source>>,
    preds: Vec<&'block Block<'block, 'source>>,
    succs: Vec<&'block Block<'block, 'source>>,
}

impl<'block, 'source> Block<'block, 'source> {
    pub fn new(id: usize) -> Self {
        Block {
            kind: BlockKind::PlainBlock,
            id,
            instructions: vec![],
            vars: vec![],
            preds: vec![],
            succs: vec![],
        }
    }
    pub fn insert_instr(&mut self, instr: Instr) -> () {
        self.instructions.push(instr);
    }
    pub fn insert_pred(&mut self, pred: &'block Block<'block, 'source>) -> () {
        self.preds.push(pred);
    }
    pub fn insert_succ(&mut self, succ: &'block Block<'block, 'source>) -> () {
        self.succs.push(succ);
    }
    pub fn search_symbol(&self, hash: usize, comp: &'block str) -> Option<usize> {
        let iter = self.vars.iter().rev();
        for val in iter {
            if let Some(x) = val.symbol_comp_last(hash, comp) {
                return Some(x);
            }
        }
        return None;
    }
}
