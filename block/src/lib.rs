use instr::Instr;

pub enum BlockKind {
    PlainBlock,
    IfBlock,
    RetBlock,
    RetBlockVoid,
    FirstBlock,
    TopBlock,
    LeafBlock,
}

pub struct Block<'a> {
    pub kind: BlockKind,
    preds: Vec<&'a Block<'a>>,
    succs: Vec<&'a Block<'a>>,
    instructions: Vec<&'a Instr>,
    pub id: usize,
}

impl<'a> Block<'a> {
    pub fn new(id: usize) -> Self {
        Block {
            kind: BlockKind::PlainBlock,
            preds: vec![],
            succs: vec![],
            instructions: vec![],
            id,
        }
    }
    pub fn insert_instr(&mut self, instr: &'a Instr) -> () {
        self.instructions.push(instr);
    }
    pub fn insert_pred(&mut self, pred: &'a Block<'a>) -> () {
        self.preds.push(pred);
    }
    pub fn insert_succ(&mut self, succ: &'a Block<'a>) -> () {
        self.succs.push(succ);
    }
}
